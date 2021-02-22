use crate::{MASK_3BIT, MASK_5BIT, MASK_7BIT};
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum PrivilegedInstruction {
    TypeR {
        opcode: PrivilegedOpcodeR,
        rs1: usize,
        rs2: usize,
        rd: usize,
    },
}

impl fmt::Display for PrivilegedInstruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PrivilegedInstruction::TypeR {
                opcode,
                rs1,
                rs2,
                rd,
            } => write!(
                f,
                "PrivilegedInstruction::TypeR {{ opcode: {}, rs1: {}, rs2: {}, rd: {} }}",
                opcode, rs1, rs2, rd,
            ),
        }
    }
}

impl PrivilegedInstruction {
    pub fn decode(instruction: u32) -> Option<Self> {
        let opcode = instruction & MASK_7BIT;
        let funct3 = (instruction >> 12) & MASK_3BIT;
        let funct7 = (instruction >> 25) & MASK_7BIT;
        let rs2 = (instruction >> 20) & MASK_5BIT;
        match opcode {
            0b1110011 => Self::decode_r(
                match funct3 {
                    0b000 => match funct7 {
                        0b0000000 => Some(PrivilegedOpcodeR::Uret),
                        0b0001000 => match rs2 {
                            0b00010 => Some(PrivilegedOpcodeR::Sret),
                            0b00101 => Some(PrivilegedOpcodeR::Wfi),
                            _ => None,
                        },
                        0b0011000 => Some(PrivilegedOpcodeR::Mret),
                        0b0001001 => Some(PrivilegedOpcodeR::SfenceVma),
                        _ => None,
                    },
                    _ => None,
                },
                instruction,
            ),
            _ => None,
        }
    }

    fn decode_r(opcode: Option<PrivilegedOpcodeR>, instruction: u32) -> Option<Self> {
        opcode.map(|o| {
            let rs1 = ((instruction >> 15) & MASK_5BIT) as usize;
            let rs2 = ((instruction >> 20) & MASK_5BIT) as usize;
            let rd = ((instruction >> 7) & MASK_5BIT) as usize;
            Self::TypeR {
                opcode: o,
                rs1,
                rs2,
                rd,
            }
        })
    }
}

#[derive(Debug, PartialEq)]
pub enum PrivilegedOpcodeR {
    Uret,
    Sret,
    Mret,
    Wfi,
    SfenceVma,
}

impl fmt::Display for PrivilegedOpcodeR {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PrivilegedOpcodeR::Uret => f.write_str("Uret"),
            PrivilegedOpcodeR::Sret => f.write_str("Sret"),
            PrivilegedOpcodeR::Mret => f.write_str("Mret"),
            PrivilegedOpcodeR::Wfi => f.write_str("Wfi"),
            PrivilegedOpcodeR::SfenceVma => f.write_str("SfenceVma"),
        }
    }
}
