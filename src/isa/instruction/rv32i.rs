use crate::{MASK_3BIT, MASK_5BIT, MASK_7BIT};
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Rv32iInstruction {
    TypeR {
        opcode: Rv32iOpcodeR,
        rs1: usize,
        rs2: usize,
        rd: usize,
    },
    TypeI {
        opcode: Rv32iOpcodeI,
        rs1: usize,
        rd: usize,
        imm: u32,
    },
    TypeS {
        opcode: Rv32iOpcodeS,
        rs1: usize,
        rs2: usize,
        imm: u32,
    },
    TypeB {
        opcode: Rv32iOpcodeB,
        rs1: usize,
        rs2: usize,
        imm: u32,
    },
    TypeU {
        opcode: Rv32iOpcodeU,
        rd: usize,
        imm: u32,
    },
    TypeJ {
        opcode: Rv32iOpcodeJ,
        rd: usize,
        imm: u32,
    },
}

impl fmt::Display for Rv32iInstruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Rv32iInstruction::TypeR {
                opcode,
                rs1,
                rs2,
                rd,
            } => write!(
                f,
                "Rv32iInstruction::TypeR {{ opcode: {}, rs1: {}, rs2: {}, rd: {} }}",
                opcode, rs1, rs2, rd,
            ),
            Rv32iInstruction::TypeI {
                opcode,
                rs1,
                rd,
                imm,
            } => write!(
                f,
                "Rv32iInstruction::TypeI {{ opcode: {}, rs1: {}, rd: {}, imm: {} }}",
                opcode, rs1, rd, imm,
            ),
            Rv32iInstruction::TypeS {
                opcode,
                rs1,
                rs2,
                imm,
            } => write!(
                f,
                "Rv32iInstruction::TypeS {{ opcode: {}, rs1: {}, rs2: {}, imm: {} }}",
                opcode, rs1, rs2, imm,
            ),
            Rv32iInstruction::TypeB {
                opcode,
                rs1,
                rs2,
                imm,
            } => write!(
                f,
                "Rv32iInstruction::TypeB {{ opcode: {}, rs1: {}, rs2: {}, imm: {} }}",
                opcode, rs1, rs2, imm,
            ),
            Rv32iInstruction::TypeU { opcode, rd, imm } => write!(
                f,
                "Rv32iInstruction::TypeU {{ opcode: {}, rd: {}, imm: {} }}",
                opcode, rd, imm,
            ),
            Rv32iInstruction::TypeJ { opcode, rd, imm } => write!(
                f,
                "Rv32iInstruction::TypeJ {{ opcode: {}, rd: {}, imm: {} }}",
                opcode, rd, imm,
            ),
        }
    }
}

impl Rv32iInstruction {
    pub fn decode(instruction: u32) -> Option<Self> {
        let opcode = instruction & MASK_7BIT;
        let funct3 = (instruction >> 12) & MASK_3BIT;
        let funct7 = (instruction >> 25) & MASK_7BIT;
        match opcode {
            0b0110111 => Self::decode_u(Some(Rv32iOpcodeU::Lui), instruction),
            0b0010111 => Self::decode_u(Some(Rv32iOpcodeU::Auipc), instruction),
            0b1101111 => Self::decode_j(Some(Rv32iOpcodeJ::Jal), instruction),
            0b1100111 => Self::decode_i(Some(Rv32iOpcodeI::Jalr), instruction),
            0b1100011 => Self::decode_b(
                match funct3 {
                    0b000 => Some(Rv32iOpcodeB::Beq),
                    0b001 => Some(Rv32iOpcodeB::Bne),
                    0b100 => Some(Rv32iOpcodeB::Blt),
                    0b101 => Some(Rv32iOpcodeB::Bge),
                    0b110 => Some(Rv32iOpcodeB::Bltu),
                    0b111 => Some(Rv32iOpcodeB::Bgeu),
                    _ => None,
                },
                instruction,
            ),
            0b0000011 => Self::decode_i(
                match funct3 {
                    0b000 => Some(Rv32iOpcodeI::Lb),
                    0b001 => Some(Rv32iOpcodeI::Lh),
                    0b010 => Some(Rv32iOpcodeI::Lw),
                    0b100 => Some(Rv32iOpcodeI::Lbu),
                    0b101 => Some(Rv32iOpcodeI::Lhu),
                    _ => None,
                },
                instruction,
            ),
            0b0100011 => Self::decode_s(
                match funct3 {
                    0b000 => Some(Rv32iOpcodeS::Sb),
                    0b001 => Some(Rv32iOpcodeS::Sh),
                    0b010 => Some(Rv32iOpcodeS::Sw),
                    _ => None,
                },
                instruction,
            ),
            0b0010011 => Self::decode_i(
                match funct3 {
                    0b000 => Some(Rv32iOpcodeI::Addi),
                    0b010 => Some(Rv32iOpcodeI::Slti),
                    0b011 => Some(Rv32iOpcodeI::Sltiu),
                    0b100 => Some(Rv32iOpcodeI::Xori),
                    0b110 => Some(Rv32iOpcodeI::Ori),
                    0b111 => Some(Rv32iOpcodeI::Andi),
                    0b001 => Some(Rv32iOpcodeI::Slli),
                    0b101 => match funct7 {
                        0b0000000 => Some(Rv32iOpcodeI::Srli),
                        0b0100000 => Some(Rv32iOpcodeI::Srai),
                        _ => None,
                    },
                    _ => None,
                },
                instruction,
            ),
            0b0110011 => Self::decode_r(
                match funct3 {
                    0b000 => match funct7 {
                        0b0000000 => Some(Rv32iOpcodeR::Add),
                        0b0100000 => Some(Rv32iOpcodeR::Sub),
                        _ => None,
                    },
                    0b001 => Some(Rv32iOpcodeR::Sll),
                    0b010 => Some(Rv32iOpcodeR::Slt),
                    0b011 => Some(Rv32iOpcodeR::Sltu),
                    0b100 => Some(Rv32iOpcodeR::Xor),
                    0b101 => match funct7 {
                        0b0000000 => Some(Rv32iOpcodeR::Srl),
                        0b0100000 => Some(Rv32iOpcodeR::Sra),
                        _ => None,
                    },
                    0b110 => Some(Rv32iOpcodeR::Or),
                    0b111 => Some(Rv32iOpcodeR::And),
                    _ => None,
                },
                instruction,
            ),
            0b0001111 => Self::decode_i(
                match funct3 {
                    0b000 => Some(Rv32iOpcodeI::Fence),
                    0b001 => Some(Rv32iOpcodeI::FenceI),
                    _ => None,
                },
                instruction,
            ),
            0b1110011 => Self::decode_i(
                match funct3 {
                    0b000 => match instruction >> 20 {
                        0b0 => Some(Rv32iOpcodeI::Ecall),
                        0b1 => Some(Rv32iOpcodeI::Ebreak),
                        _ => None,
                    },
                    0b001 => Some(Rv32iOpcodeI::Csrrw),
                    0b010 => Some(Rv32iOpcodeI::Csrrs),
                    0b011 => Some(Rv32iOpcodeI::Csrrc),
                    0b101 => Some(Rv32iOpcodeI::Csrrwi),
                    0b110 => Some(Rv32iOpcodeI::Csrrsi),
                    0b111 => Some(Rv32iOpcodeI::Csrrci),
                    _ => None,
                },
                instruction,
            ),
            _ => None,
        }
    }

    fn decode_r(opcode: Option<Rv32iOpcodeR>, instruction: u32) -> Option<Self> {
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

    fn decode_i(opcode: Option<Rv32iOpcodeI>, instruction: u32) -> Option<Self> {
        opcode.map(|o| {
            let rd = ((instruction >> 7) & MASK_5BIT) as usize;
            let rs1 = ((instruction >> 15) & MASK_5BIT) as usize;
            let imm = ((instruction as i32) >> 20) as u32;
            Self::TypeI {
                opcode: o,
                rd,
                rs1,
                imm,
            }
        })
    }

    fn decode_s(opcode: Option<Rv32iOpcodeS>, instruction: u32) -> Option<Self> {
        opcode.map(|o| {
            let rs1 = ((instruction >> 15) & MASK_5BIT) as usize;
            let rs2 = ((instruction >> 20) & MASK_5BIT) as usize;
            let imm = ((instruction & 0xfe000000) as i32 >> 20) as u32 | (instruction >> 7) & 0x1f;
            Self::TypeS {
                opcode: o,
                rs1,
                rs2,
                imm,
            }
        })
    }

    fn decode_b(opcode: Option<Rv32iOpcodeB>, instruction: u32) -> Option<Self> {
        opcode.map(|o| {
            let rs1 = ((instruction >> 15) & MASK_5BIT) as usize;
            let rs2 = ((instruction >> 20) & MASK_5BIT) as usize;
            let imm = ((instruction & 0x80000000) as i32 >> 19) as u32
                | (instruction & 0x80) << 4
                | (instruction >> 20) & 0x7e0
                | (instruction >> 7) & 0x1e;
            Self::TypeB {
                opcode: o,
                rs1,
                rs2,
                imm,
            }
        })
    }

    fn decode_u(opcode: Option<Rv32iOpcodeU>, instruction: u32) -> Option<Self> {
        opcode.map(|o| {
            let rd = ((instruction >> 7) & MASK_5BIT) as usize;
            let imm = instruction & 0xfffff000;
            Self::TypeU { opcode: o, rd, imm }
        })
    }

    fn decode_j(opcode: Option<Rv32iOpcodeJ>, instruction: u32) -> Option<Self> {
        opcode.map(|o| {
            let rd = ((instruction >> 7) & MASK_5BIT) as usize;
            let imm = ((instruction & 0x80000000) as i32 >> 11) as u32
                | instruction & 0xff000
                | (instruction >> 9) & 0x800
                | (instruction >> 20) & 0x7fe;
            Self::TypeJ { opcode: o, rd, imm }
        })
    }
}

#[derive(Debug, PartialEq)]
pub enum Rv32iOpcodeR {
    Sll,
    Srl,
    Sra,
    Add,
    Sub,
    Xor,
    Or,
    And,
    Slt,
    Sltu,
}

impl fmt::Display for Rv32iOpcodeR {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Rv32iOpcodeR::Sll => f.write_str("Sll"),
            Rv32iOpcodeR::Srl => f.write_str("Srl"),
            Rv32iOpcodeR::Sra => f.write_str("Sra"),
            Rv32iOpcodeR::Add => f.write_str("Add"),
            Rv32iOpcodeR::Sub => f.write_str("Sub"),
            Rv32iOpcodeR::Xor => f.write_str("Xor"),
            Rv32iOpcodeR::Or => f.write_str("Or"),
            Rv32iOpcodeR::And => f.write_str("And"),
            Rv32iOpcodeR::Slt => f.write_str("Slt"),
            Rv32iOpcodeR::Sltu => f.write_str("Sltu"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Rv32iOpcodeI {
    Slli,
    Srli,
    Srai,
    Addi,
    Xori,
    Ori,
    Andi,
    Slti,
    Sltiu,
    Jalr,
    Fence,
    FenceI,
    Ecall,
    Ebreak,
    Csrrw,
    Csrrs,
    Csrrc,
    Csrrwi,
    Csrrsi,
    Csrrci,
    Lb,
    Lh,
    Lbu,
    Lhu,
    Lw,
}

impl fmt::Display for Rv32iOpcodeI {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Rv32iOpcodeI::Slli => f.write_str("Slli"),
            Rv32iOpcodeI::Srli => f.write_str("Srli"),
            Rv32iOpcodeI::Srai => f.write_str("Srai"),
            Rv32iOpcodeI::Addi => f.write_str("Addi"),
            Rv32iOpcodeI::Xori => f.write_str("Xori"),
            Rv32iOpcodeI::Ori => f.write_str("Ori"),
            Rv32iOpcodeI::Andi => f.write_str("Andi"),
            Rv32iOpcodeI::Slti => f.write_str("Slti"),
            Rv32iOpcodeI::Sltiu => f.write_str("Sltiu"),
            Rv32iOpcodeI::Jalr => f.write_str("Jalr"),
            Rv32iOpcodeI::Fence => f.write_str("Fence"),
            Rv32iOpcodeI::FenceI => f.write_str("FenceI"),
            Rv32iOpcodeI::Ecall => f.write_str("Ecall"),
            Rv32iOpcodeI::Ebreak => f.write_str("Ebreak"),
            Rv32iOpcodeI::Csrrw => f.write_str("Csrrw"),
            Rv32iOpcodeI::Csrrs => f.write_str("Csrrs"),
            Rv32iOpcodeI::Csrrc => f.write_str("Csrrc"),
            Rv32iOpcodeI::Csrrwi => f.write_str("Csrrwi"),
            Rv32iOpcodeI::Csrrsi => f.write_str("Csrrsi"),
            Rv32iOpcodeI::Csrrci => f.write_str("Csrrci"),
            Rv32iOpcodeI::Lb => f.write_str("Lb"),
            Rv32iOpcodeI::Lh => f.write_str("Lh"),
            Rv32iOpcodeI::Lbu => f.write_str("Lbu"),
            Rv32iOpcodeI::Lhu => f.write_str("Lhu"),
            Rv32iOpcodeI::Lw => f.write_str("Lw"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Rv32iOpcodeS {
    Sb,
    Sh,
    Sw,
}

impl fmt::Display for Rv32iOpcodeS {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Rv32iOpcodeS::Sb => f.write_str("Sb"),
            Rv32iOpcodeS::Sh => f.write_str("Sh"),
            Rv32iOpcodeS::Sw => f.write_str("Sw"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Rv32iOpcodeB {
    Beq,
    Bne,
    Blt,
    Bge,
    Bltu,
    Bgeu,
}

impl fmt::Display for Rv32iOpcodeB {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Rv32iOpcodeB::Beq => f.write_str("Beq"),
            Rv32iOpcodeB::Bne => f.write_str("Bne"),
            Rv32iOpcodeB::Blt => f.write_str("Blt"),
            Rv32iOpcodeB::Bge => f.write_str("Bge"),
            Rv32iOpcodeB::Bltu => f.write_str("Bltu"),
            Rv32iOpcodeB::Bgeu => f.write_str("Bgeu"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Rv32iOpcodeU {
    Lui,
    Auipc,
}

impl fmt::Display for Rv32iOpcodeU {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Rv32iOpcodeU::Lui => f.write_str("Lui"),
            Rv32iOpcodeU::Auipc => f.write_str("Auipc"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Rv32iOpcodeJ {
    Jal,
}

impl fmt::Display for Rv32iOpcodeJ {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Rv32iOpcodeJ::Jal => f.write_str("Jal"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_typeu_ok() {
        let inst = 0b00000000010101010101_00101_0010111;
        assert_eq!(
            Rv32iInstruction::decode(inst).unwrap(),
            Rv32iInstruction::TypeU {
                opcode: Rv32iOpcodeU::Auipc,
                rd: 0b00101,
                imm: 0b010101010101_00000_0000000,
            }
        );
    }

    #[test]
    fn decode_typej_ok() {
        let inst = 0b1_0000000010_1_01010101_00101_1101111;
        assert_eq!(
            Rv32iInstruction::decode(inst).unwrap(),
            Rv32iInstruction::TypeJ {
                opcode: Rv32iOpcodeJ::Jal,
                rd: 0b00101,
                imm: 0b11111111111_1_01010101_1_0000000010_0,
            }
        );
    }

    #[test]
    fn decode_typei_ok() {
        let inst = 0b100000000101_01010_000_00101_1100111;
        assert_eq!(
            Rv32iInstruction::decode(inst).unwrap(),
            Rv32iInstruction::TypeI {
                opcode: Rv32iOpcodeI::Jalr,
                rs1: 0b01010,
                rd: 0b00101,
                imm: 0b11111111111111111111100000000101,
            }
        );
    }

    #[test]
    fn decode_typeb_ok() {
        let inst = 0b1010101_00101_01010_000_10101_1100011;
        assert_eq!(
            Rv32iInstruction::decode(inst).unwrap(),
            Rv32iInstruction::TypeB {
                opcode: Rv32iOpcodeB::Beq,
                rs1: 0b01010,
                rs2: 0b00101,
                imm: 0b11111111111111111111_1_010101_1010_0,
            }
        );
    }

    #[test]
    fn decode_types_ok() {
        let inst = 0b1010101_00101_01010_000_10101_0100011;
        assert_eq!(
            Rv32iInstruction::decode(inst).unwrap(),
            Rv32iInstruction::TypeS {
                opcode: Rv32iOpcodeS::Sb,
                rs1: 0b01010,
                rs2: 0b00101,
                imm: 0b11111111111111111111_1010101_10101,
            }
        );
    }

    #[test]
    fn decode_typer_ok() {
        let inst = 0b0000000_00101_01010_000_10101_0110011;
        assert_eq!(
            Rv32iInstruction::decode(inst).unwrap(),
            Rv32iInstruction::TypeR {
                opcode: Rv32iOpcodeR::Add,
                rs1: 0b01010,
                rs2: 0b00101,
                rd: 0b10101,
            }
        );
    }
}
