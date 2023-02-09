use crate::{
    bitops::MASK_3BIT,
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
            rv64f::{
                Rv64fOpcodeB, Rv64fOpcodeI, Rv64fOpcodeJ, Rv64fOpcodeR, Rv64fOpcodeS, Rv64fOpcodeU,
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
}

impl SingleExt for Single {
    fn decode(input: u32) -> Self {
        Self::from_bits(input as u128)
    }

    fn bits(self) -> u32 {
        self.to_bits() as u32
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

pub struct Rv64fExecutor;

impl Executor for Rv64fExecutor {
    type OpcodeR = Rv64fOpcodeR;
    type OpcodeI = Rv64fOpcodeI;
    type OpcodeS = Rv64fOpcodeS;
    type OpcodeB = Rv64fOpcodeB;
    type OpcodeU = Rv64fOpcodeU;
    type OpcodeJ = Rv64fOpcodeJ;

    fn execute(
        instruction: Instruction<
            Rv64fOpcodeR,
            Rv64fOpcodeI,
            Rv64fOpcodeS,
            Rv64fOpcodeB,
            Rv64fOpcodeU,
            Rv64fOpcodeJ,
        >,
        _: &PrivilegeMode,
        _: &mut ProgramCounter,
        x: &mut IntegerRegister,
        f: &mut FloatingPointRegister,
        csr: &mut ControlAndStatusRegister,
        _: &mut SystemBus,
    ) -> Result<(), Cause> {
        match instruction {
            Instruction::TypeR {
                opcode,
                rd,
                funct3,
                rs1,
                rs2: _,
                funct7: _,
            } => {
                // TODO: NaN handling
                let rm = match select_rm(funct3, csr) {
                    Some(round) => round,
                    None => return Err(Cause::Exception(Exception::IllegalInstruction)),
                };
                let status = match opcode {
                    Rv64fOpcodeR::FcvtLS => {
                        // Accumulating CSRs: frm, NV, NX
                        let target = Single::decode(f.reads(rs1));
                        let result = target.to_i128_r(64, rm, &mut false);
                        let value = if is_invalid(result.status) && target.is_nan() {
                            0x7fffffffffffffff
                        } else {
                            result.value as i64
                        };
                        x.writei(rd, value);
                        Some(convert_to_fflags(result.status.bits()))
                    }
                    Rv64fOpcodeR::FcvtLuS => {
                        // Accumulating CSRs: frm, NV, NX
                        let target = Single::decode(f.reads(rs1));
                        let result = target.to_u128_r(64, rm, &mut false);
                        let value = if is_invalid(result.status) && target.is_nan() {
                            0xffffffffffffffff
                        } else {
                            result.value as u64
                        };
                        x.writeu(rd, value);
                        Some(convert_to_fflags(result.status.bits()))
                    }
                    Rv64fOpcodeR::FcvtSL => {
                        // Accumulating CSRs: frm, NX
                        let result = Single::from_i128_r(x.readi(rs1) as i128, rm);
                        f.writes(rd, result.bits());
                        Some(result.fflags())
                    }
                    Rv64fOpcodeR::FcvtSLu => {
                        // Accumulating CSRs: frm, NX
                        let result = Single::from_u128_r(x.readu(rs1) as u128, rm);
                        f.writes(rd, result.bits());
                        Some(result.fflags())
                    }
                };
                if let Some(fflags) = status {
                    let frm = encode_rm(rm);
                    let fcsr = (frm << 5) | fflags;
                    csr.write(FCSR, fcsr);
                }
                Ok(())
            }
            _ => Ok(()),
        }
    }
}
