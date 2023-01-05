use crate::{
    emulator::{
        bus::SystemBus,
        cpu::{
            csr::ControlAndStatusRegister,
            executor::{Executor, MASK_3BIT, MASK_5BIT},
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
        privileged::{cause::Cause, mode::PrivilegeMode},
    },
};
use rustc_apfloat::ieee::Single;
use rustc_apfloat::{Float, Round, StatusAnd};

trait StatusAndExt {
    fn bits(self) -> u32;
}

impl StatusAndExt for StatusAnd<Single> {
    fn bits(self) -> u32 {
        self.value.bits()
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
                // TODO: invalid rm handling
                let rm = select_rm(funct3, csr).unwrap();
                let rs3 = (funct7 >> 2) & MASK_5BIT as usize;
                match opcode {
                    Rv32fOpcodeR::FmaddS => {
                        f.write(
                            rd,
                            Single::decode(f.read(rs1))
                                .mul_add_r(
                                    Single::decode(f.read(rs2)),
                                    Single::decode(f.read(rs3)),
                                    rm,
                                )
                                .bits(),
                        );
                        Ok(())
                    }
                    Rv32fOpcodeR::FmsubS => {
                        f.write(
                            rd,
                            Single::decode(f.read(rs1))
                                .mul_add_r(
                                    Single::decode(f.read(rs2)),
                                    -Single::decode(f.read(rs3)),
                                    rm,
                                )
                                .bits(),
                        );
                        Ok(())
                    }
                    Rv32fOpcodeR::FnmsubS => {
                        f.write(
                            rd,
                            Single::decode(f.read(rs1))
                                .mul_add_r(
                                    -Single::decode(f.read(rs2)),
                                    Single::decode(f.read(rs3)),
                                    rm,
                                )
                                .bits(),
                        );
                        Ok(())
                    }
                    Rv32fOpcodeR::FnmaddS => {
                        f.write(
                            rd,
                            (-Single::decode(f.read(rs1)))
                                .mul_add_r(
                                    Single::decode(f.read(rs2)),
                                    -Single::decode(f.read(rs3)),
                                    rm,
                                )
                                .bits(),
                        );
                        Ok(())
                    }
                    Rv32fOpcodeR::FaddS => {
                        f.write(
                            rd,
                            Single::decode(f.read(rs1))
                                .add_r(Single::decode(f.read(rs2)), rm)
                                .bits(),
                        );
                        Ok(())
                    }
                    Rv32fOpcodeR::FsubS => {
                        f.write(
                            rd,
                            Single::decode(f.read(rs1))
                                .sub_r(Single::decode(f.read(rs2)), rm)
                                .bits(),
                        );
                        Ok(())
                    }
                    Rv32fOpcodeR::FmulS => {
                        f.write(
                            rd,
                            Single::decode(f.read(rs1))
                                .mul_r(Single::decode(f.read(rs2)), rm)
                                .bits(),
                        );
                        Ok(())
                    }
                    Rv32fOpcodeR::FdivS => {
                        f.write(
                            rd,
                            Single::decode(f.read(rs1))
                                .div_r(Single::decode(f.read(rs2)), rm)
                                .bits(),
                        );
                        Ok(())
                    }
                    Rv32fOpcodeR::FsqrtS => {
                        // sqrt is not supported by the library
                        f.write(
                            rd,
                            Single::from_f32(Single::decode(f.read(rs1)).to_f32().sqrt()).bits(),
                        );
                        Ok(())
                    }
                    Rv32fOpcodeR::FsgnjS => {
                        let sign = Single::decode(f.read(rs2));
                        let ret = Single::decode(f.read(rs1)).copy_sign(sign);
                        f.write(rd, ret.bits());
                        Ok(())
                    }
                    Rv32fOpcodeR::FsgnjnS => {
                        let sign = Single::decode(f.read(rs2));
                        let ret = Single::decode(f.read(rs1)).copy_sign(-sign);
                        f.write(rd, ret.bits());
                        Ok(())
                    }
                    Rv32fOpcodeR::FsgnjxS => {
                        let ret = Single::decode(f.read(rs1));
                        let sign = Single::decode(f.read(rs2)).is_negative() ^ ret.is_negative();
                        f.write(
                            rd,
                            (if sign == ret.is_negative() { ret } else { -ret }).bits(),
                        );
                        Ok(())
                    }
                    Rv32fOpcodeR::FminS => {
                        let val1 = Single::decode(f.read(rs1));
                        let val2 = Single::decode(f.read(rs2));
                        let min = if val1 < val2 { val1 } else { val2 };
                        f.write(rd, min.bits());
                        Ok(())
                    }
                    Rv32fOpcodeR::FmaxS => {
                        let val1 = Single::decode(f.read(rs1));
                        let val2 = Single::decode(f.read(rs2));
                        let max = if val1 < val2 { val2 } else { val1 };
                        f.write(rd, max.bits());
                        Ok(())
                    }
                    Rv32fOpcodeR::FcvtWs => {
                        x.writei(
                            rd,
                            Single::decode(f.read(rs1))
                                .to_u128_r(32, rm, &mut false)
                                .value as i64,
                        );
                        Ok(())
                    }
                    Rv32fOpcodeR::FcvtWuS => {
                        x.writei(
                            rd,
                            Single::decode(f.read(rs1))
                                .to_u128_r(32, rm, &mut false)
                                .value as i32 as i64,
                        );
                        Ok(())
                    }
                    Rv32fOpcodeR::FmvXw => {
                        x.writei(rd, f.read(rs1) as i32 as i64);
                        Ok(())
                    }
                    Rv32fOpcodeR::FeqS => {
                        let val1 = Single::decode(f.read(rs1));
                        let val2 = Single::decode(f.read(rs2));
                        let ret = u32::from(val1 == val2);
                        f.write(rd, ret);
                        Ok(())
                    }
                    Rv32fOpcodeR::FltS => {
                        let val1 = Single::decode(f.read(rs1));
                        let val2 = Single::decode(f.read(rs2));
                        let ret = u32::from(val1 < val2);
                        f.write(rd, ret);
                        Ok(())
                    }
                    Rv32fOpcodeR::FleS => {
                        let val1 = Single::decode(f.read(rs1));
                        let val2 = Single::decode(f.read(rs2));
                        let ret = u32::from(val1 <= val2);
                        f.write(rd, ret);
                        Ok(())
                    }
                    Rv32fOpcodeR::FclassS => {
                        let val = Single::decode(f.read(rs1));
                        let class = if val.is_negative() && val.is_infinite() {
                            0
                        } else if val.is_negative() && val.is_normal() {
                            1
                        } else if val.is_negative() && val.is_denormal() {
                            2
                        } else if val.is_neg_zero() {
                            3
                        } else if val.is_pos_zero() {
                            4
                        } else if !val.is_negative() && val.is_denormal() {
                            5
                        } else if !val.is_negative() && val.is_normal() {
                            6
                        } else if !val.is_negative() && val.is_infinite() {
                            7
                        } else {
                            // TODO: supports signaling NaN / quiet NaN
                            9
                        };
                        x.writeu(rd, class);
                        Ok(())
                    }
                    Rv32fOpcodeR::FcvtSw => {
                        f.write(
                            rd,
                            Single::from_i128_r(x.readi(rs1) as i32 as i128, rm).bits(),
                        );
                        Ok(())
                    }
                    Rv32fOpcodeR::FcvtSWu => {
                        f.write(rd, Single::from_u128_r(x.readi(rs1) as u128, rm).bits());
                        Ok(())
                    }
                    Rv32fOpcodeR::FmvWx => {
                        f.write(rd, x.readu(rs1) as u32);
                        Ok(())
                    }
                }
            }
            Instruction::TypeI {
                opcode,
                rd,
                funct3: _,
                rs1,
                imm,
            } => match opcode {
                Rv32fOpcodeI::Flw => {
                    f.write(rd, bus.load32(x.readi(rs1).wrapping_add(imm as i64) as u64));
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
                    bus.store32(x.readi(rs1).wrapping_add(imm as i64) as u64, f.read(rs2));
                    Ok(())
                }
            },
            _ => Ok(()),
        }
    }
}
