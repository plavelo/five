use crate::{
    bitops::extend_sign,
    bitops::{MASK_3BIT, MASK_5BIT},
    emulator::{
        bus::SystemBus,
        cpu::{
            csr::{ControlAndStatusRegister, Csr},
            executor::Executor,
            f::FloatingPointRegister,
            pc::ProgramCounter,
            x::IntegerRegister,
        },
    },
    isa::{
        csr::user_level::FCSR,
        instruction::{
            rv32f::{
                Rv32fOpcodeB, Rv32fOpcodeI, Rv32fOpcodeJ, Rv32fOpcodeR, Rv32fOpcodeS, Rv32fOpcodeU,
            },
            Instruction,
        },
        privileged::{
            cause::{Cause, Exception},
            mode::PrivilegeMode,
        },
    },
};
use rustc_apfloat::{ieee::Single, Float, Round, Status, StatusAnd};

fn convert_to_fflags(status: u8) -> u64 {
    let nv = status & 0b1; // Invalid Operation
    let dz = (status >> 1) & 0b1; // Divide by Zero
    let of = (status >> 2) & 0b1; // Overflow
    let uf = (status >> 3) & 0b1; // Underflow
    let nx = (status >> 4) & 0b1; // Inexact
    ((nv << 4) | (dz << 3) | (of << 2) | (uf << 1) | nx) as u64
}

fn is_invalid(status: Status) -> bool {
    status.bits() & 0b1 == 1 // Invalid Operation
}

trait StatusAndExt {
    fn bits(self) -> u32;
    fn fflags(self) -> u64;
}

impl StatusAndExt for StatusAnd<Single> {
    fn bits(self) -> u32 {
        self.value.bits()
    }

    fn fflags(self) -> u64 {
        let status = self.status.bits();
        convert_to_fflags(status)
    }
}

trait SingleExt {
    fn decode(input: u32) -> Self;
    fn bits(self) -> u32;
    fn from_f32(input: f32) -> Self;
    fn to_f32(self) -> f32;
}

impl SingleExt for Single {
    fn decode(input: u32) -> Self {
        Self::from_bits(input as u128)
    }

    fn bits(self) -> u32 {
        self.to_bits() as u32
    }

    fn from_f32(input: f32) -> Self {
        Self::from_bits(input.to_bits() as u128)
    }

    fn to_f32(self) -> f32 {
        f32::from_bits(self.to_bits() as u32)
    }
}

fn decode_rm(rm: usize) -> Option<Round> {
    match rm {
        0b000 => Some(Round::NearestTiesToEven), // RNE, Round to Nearest, ties to Even
        0b001 => Some(Round::TowardZero),        // RTZ, Round towards Zero
        0b010 => Some(Round::TowardNegative),    // RDN, Round Down (towards −∞)
        0b011 => Some(Round::TowardPositive),    // RUP, Round Up (towards +∞)
        0b100 => Some(Round::NearestTiesToAway), // RMM, Round to Nearest, ties to Max Magnitude
        _ => None,
    }
}

fn encode_rm(round: Round) -> u64 {
    match round {
        Round::NearestTiesToEven => 0b000,
        Round::TowardZero => 0b001,
        Round::TowardNegative => 0b010,
        Round::TowardPositive => 0b011,
        Round::NearestTiesToAway => 0b100,
    }
}

fn select_rm(rm: usize, csr: &mut ControlAndStatusRegister) -> Option<Round> {
    match rm {
        0b111 => {
            // DYN, In instruction’s rm field, selects dynamic rounding mode; In Rounding Mode register, Invalid.
            let frm = ((csr.csrrs(FCSR, 0) >> 5) & MASK_3BIT) as usize;
            decode_rm(frm)
        }
        _ => decode_rm(rm),
    }
}

pub struct Rv32fExecutor;

impl Executor for Rv32fExecutor {
    type OpcodeR = Rv32fOpcodeR;
    type OpcodeI = Rv32fOpcodeI;
    type OpcodeS = Rv32fOpcodeS;
    type OpcodeB = Rv32fOpcodeB;
    type OpcodeU = Rv32fOpcodeU;
    type OpcodeJ = Rv32fOpcodeJ;

    fn execute(
        instruction: Instruction<
            Rv32fOpcodeR,
            Rv32fOpcodeI,
            Rv32fOpcodeS,
            Rv32fOpcodeB,
            Rv32fOpcodeU,
            Rv32fOpcodeJ,
        >,
        _: &PrivilegeMode,
        _: &mut ProgramCounter,
        x: &mut IntegerRegister,
        f: &mut FloatingPointRegister,
        csr: &mut ControlAndStatusRegister,
        bus: &mut SystemBus,
    ) -> Result<(), Cause> {
        match instruction {
            Instruction::TypeR {
                opcode,
                rd,
                funct3,
                rs1,
                rs2,
                funct7,
            } => {
                // TODO: NaN handling
                let rm = match select_rm(funct3, csr) {
                    Some(round) => round,
                    None => return Err(Cause::Exception(Exception::IllegalInstruction)),
                };
                let rs3 = (funct7 >> 2) & MASK_5BIT as usize;
                let status = match opcode {
                    Rv32fOpcodeR::FmaddS => {
                        // Accumulating CSRs: frm, NV, OF, UF, NX
                        let result = Single::decode(f.reads(rs1)).mul_add_r(
                            Single::decode(f.reads(rs2)),
                            Single::decode(f.reads(rs3)),
                            rm,
                        );
                        f.writes(rd, result.bits());
                        Some(result.fflags())
                    }
                    Rv32fOpcodeR::FmsubS => {
                        // Accumulating CSRs: frm, NV, OF, UF, NX
                        let result = Single::decode(f.reads(rs1)).mul_add_r(
                            Single::decode(f.reads(rs2)),
                            -Single::decode(f.reads(rs3)),
                            rm,
                        );
                        f.writes(rd, result.bits());
                        Some(result.fflags())
                    }
                    Rv32fOpcodeR::FnmsubS => {
                        // Accumulating CSRs: frm, NV, OF, UF, NX
                        let result = Single::decode(f.reads(rs1)).mul_add_r(
                            -Single::decode(f.reads(rs2)),
                            Single::decode(f.reads(rs3)),
                            rm,
                        );
                        f.writes(rd, result.bits());
                        Some(result.fflags())
                    }
                    Rv32fOpcodeR::FnmaddS => {
                        // Accumulating CSRs: frm, NV, OF, UF, NX
                        let result = (-Single::decode(f.reads(rs1))).mul_add_r(
                            Single::decode(f.reads(rs2)),
                            -Single::decode(f.reads(rs3)),
                            rm,
                        );
                        f.writes(rd, result.bits());
                        Some(result.fflags())
                    }
                    Rv32fOpcodeR::FaddS => {
                        // Accumulating CSRs: frm, NV, OF, UF, NX
                        let result =
                            Single::decode(f.reads(rs1)).add_r(Single::decode(f.reads(rs2)), rm);
                        f.writes(rd, result.bits());
                        Some(result.fflags())
                    }
                    Rv32fOpcodeR::FsubS => {
                        // Accumulating CSRs: frm, NV, OF, UF, NX
                        let result =
                            Single::decode(f.reads(rs1)).sub_r(Single::decode(f.reads(rs2)), rm);
                        f.writes(rd, result.bits());
                        Some(result.fflags())
                    }
                    Rv32fOpcodeR::FmulS => {
                        // Accumulating CSRs: frm, NV, OF, UF, NX
                        let result =
                            Single::decode(f.reads(rs1)).mul_r(Single::decode(f.reads(rs2)), rm);
                        f.writes(rd, result.bits());
                        Some(result.fflags())
                    }
                    Rv32fOpcodeR::FdivS => {
                        // Accumulating CSRs: frm, NV, DZ, OF, UF, NX
                        let result =
                            Single::decode(f.reads(rs1)).div_r(Single::decode(f.reads(rs2)), rm);
                        f.writes(rd, result.bits());
                        Some(result.fflags())
                    }
                    Rv32fOpcodeR::FsqrtS => {
                        // Accumulating CSRs: frm, NV, NX
                        // However, it is not supported by the library
                        f.writes(
                            rd,
                            Single::from_f32(Single::decode(f.reads(rs1)).to_f32().sqrt()).bits(),
                        );
                        None
                    }
                    Rv32fOpcodeR::FsgnjS => {
                        // Accumulating CSRs: None
                        let sign = Single::decode(f.reads(rs2));
                        let ret = Single::decode(f.reads(rs1)).copy_sign(sign);
                        f.writes(rd, ret.bits());
                        None
                    }
                    Rv32fOpcodeR::FsgnjnS => {
                        // Accumulating CSRs: None
                        let sign = Single::decode(f.reads(rs2));
                        let ret = Single::decode(f.reads(rs1)).copy_sign(-sign);
                        f.writes(rd, ret.bits());
                        None
                    }
                    Rv32fOpcodeR::FsgnjxS => {
                        // Accumulating CSRs: None
                        let ret = Single::decode(f.reads(rs1));
                        let sign = Single::decode(f.reads(rs2)).is_negative() ^ ret.is_negative();
                        f.writes(
                            rd,
                            (if sign == ret.is_negative() { ret } else { -ret }).bits(),
                        );
                        None
                    }
                    Rv32fOpcodeR::FminS => {
                        // Accumulating CSRs: NV
                        let val1 = Single::decode(f.reads(rs1));
                        let val2 = Single::decode(f.reads(rs2));
                        let min = if val1.is_nan() && !val2.is_nan() {
                            val2.bits()
                        } else if !val1.is_nan() && val2.is_nan() {
                            val1.bits()
                        } else if val1.is_nan() && val2.is_nan() {
                            0x7fc00000
                        } else if val1.is_neg_zero() && val2.is_pos_zero() {
                            val1.bits()
                        } else if val1.is_pos_zero() && val2.is_neg_zero() {
                            val2.bits()
                        } else if val1 < val2 {
                            val1.bits()
                        } else {
                            val2.bits()
                        };
                        f.writes(rd, min);
                        let result = if val1.is_signaling() || val2.is_signaling() {
                            0b10000
                        } else {
                            0
                        };
                        Some(result)
                    }
                    Rv32fOpcodeR::FmaxS => {
                        // Accumulating CSRs: NV
                        let val1 = Single::decode(f.reads(rs1));
                        let val2 = Single::decode(f.reads(rs2));
                        let max = if val1.is_nan() && !val2.is_nan() {
                            val2.bits()
                        } else if !val1.is_nan() && val2.is_nan() {
                            val1.bits()
                        } else if val1.is_nan() && val2.is_nan() {
                            0x7fc00000
                        } else if val1.is_neg_zero() && val2.is_pos_zero() {
                            val2.bits()
                        } else if val1.is_pos_zero() && val2.is_neg_zero() {
                            val1.bits()
                        } else if val1 < val2 {
                            val2.bits()
                        } else {
                            val1.bits()
                        };
                        f.writes(rd, max);
                        let result = if val1.is_signaling() || val2.is_signaling() {
                            0b10000
                        } else {
                            0
                        };
                        Some(result)
                    }
                    Rv32fOpcodeR::FcvtWS => {
                        // Accumulating CSRs: frm, NV, NX
                        let target = Single::decode(f.reads(rs1));
                        let result = target.to_i128_r(32, rm, &mut false);
                        let value = if is_invalid(result.status) && target.is_nan() {
                            0x7fffffff
                        } else {
                            result.value as i32 as i64
                        };
                        x.writei(rd, value);
                        Some(convert_to_fflags(result.status.bits()))
                    }
                    Rv32fOpcodeR::FcvtWuS => {
                        // Accumulating CSRs: frm, NV, NX
                        let target = Single::decode(f.reads(rs1));
                        let result = target.to_u128_r(32, rm, &mut false);
                        let value = if is_invalid(result.status) && target.is_nan() {
                            0xffffffffffffffffu64 as i64
                        } else {
                            result.value as i32 as i64
                        };
                        x.writei(rd, value);
                        Some(convert_to_fflags(result.status.bits()))
                    }
                    Rv32fOpcodeR::FmvXW => {
                        // Accumulating CSRs: None
                        x.writei(rd, extend_sign(f.reads(rs1) as u64, 32));
                        None
                    }
                    Rv32fOpcodeR::FeqS => {
                        // Accumulating CSRs: NV
                        let val1 = Single::decode(f.reads(rs1));
                        let val2 = Single::decode(f.reads(rs2));
                        let ret = u64::from(val1 == val2);
                        x.writeu(rd, ret);
                        let result = if val1.is_signaling() || val2.is_signaling() {
                            0b10000
                        } else {
                            0
                        };
                        Some(result)
                    }
                    Rv32fOpcodeR::FltS => {
                        // Accumulating CSRs: NV
                        let val1 = Single::decode(f.reads(rs1));
                        let val2 = Single::decode(f.reads(rs2));
                        let ret = u64::from(val1 < val2);
                        x.writeu(rd, ret);
                        let result = if val1.is_nan() || val2.is_nan() {
                            0b10000
                        } else {
                            0
                        };
                        Some(result)
                    }
                    Rv32fOpcodeR::FleS => {
                        // Accumulating CSRs: NV
                        let val1 = Single::decode(f.reads(rs1));
                        let val2 = Single::decode(f.reads(rs2));
                        let ret = u64::from(val1 <= val2);
                        x.writeu(rd, ret);
                        let result = if val1.is_nan() || val2.is_nan() {
                            0b10000
                        } else {
                            0
                        };
                        Some(result)
                    }
                    Rv32fOpcodeR::FclassS => {
                        // Accumulating CSRs: None
                        let value = Single::decode(f.reads(rs1));
                        let class = if value.is_negative() && value.is_infinite() {
                            0b0000000001
                        } else if value.is_negative() && value.is_normal() {
                            0b0000000010
                        } else if value.is_negative() && value.is_denormal() {
                            0b0000000100
                        } else if value.is_neg_zero() {
                            0b0000001000
                        } else if value.is_pos_zero() {
                            0b0000010000
                        } else if !value.is_negative() && value.is_denormal() {
                            0b0000100000
                        } else if !value.is_negative() && value.is_normal() {
                            0b0001000000
                        } else if !value.is_negative() && value.is_infinite() {
                            0b0010000000
                        } else if value.is_signaling() {
                            0b0100000000
                        } else {
                            0b1000000000
                        };
                        x.writeu(rd, class);
                        None
                    }
                    Rv32fOpcodeR::FcvtSW => {
                        // Accumulating CSRs: frm, NV, NX
                        let result = Single::from_i128_r(x.readi(rs1) as i32 as i128, rm);
                        f.writes(rd, result.bits());
                        Some(result.fflags())
                    }
                    Rv32fOpcodeR::FcvtSWu => {
                        // Accumulating CSRs: frm, NV, NX
                        let result = Single::from_u128_r(x.readu(rs1) as u32 as u128, rm);
                        f.writes(rd, result.bits());
                        Some(result.fflags())
                    }
                    Rv32fOpcodeR::FmvWX => {
                        // Accumulating CSRs: None
                        f.writes(rd, x.readu(rs1) as u32);
                        None
                    }
                };
                if let Some(fflags) = status {
                    let frm = encode_rm(rm);
                    let fcsr = (frm << 5) | fflags;
                    csr.write(FCSR, fcsr);
                }
                Ok(())
            }
            Instruction::TypeI {
                opcode,
                rd,
                funct3: _,
                rs1,
                imm,
            } => match opcode {
                Rv32fOpcodeI::Flw => {
                    // Accumulating CSRs: None
                    f.writes(
                        rd,
                        bus.load32(x.readi(rs1).wrapping_add(extend_sign(imm, 12)) as u64),
                    );
                    Ok(())
                }
            },
            Instruction::TypeS {
                opcode,
                funct3: _,
                rs1,
                rs2,
                imm,
            } => match opcode {
                Rv32fOpcodeS::Fsw => {
                    // Accumulating CSRs: None
                    bus.store32(x.readi(rs1).wrapping_add(imm as i64) as u64, f.reads(rs2));
                    Ok(())
                }
            },
            _ => Ok(()),
        }
    }
}
