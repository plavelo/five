const MASK_5BIT: u32 = 0b11111;
const MASK_7BIT: u32 = 0b1111111;

impl Instruction {
    #[allow(dead_code)]
    pub fn decode(instruction: u32) -> Option<Self> {
        let opcode = instruction & MASK_7BIT;
        let funct3 = (instruction >> 12) & MASK_5BIT;
        let funct7 = (instruction >> 25) & MASK_7BIT;
        match opcode {
            0b0110111 => Some(Self::decode_u(OpcodeU::Lui, instruction)),
            0b0010111 => Some(Self::decode_u(OpcodeU::Auipc, instruction)),
            0b1101111 => Some(Self::decode_j(OpcodeJ::Jal, instruction)),
            0b1100111 => Some(Self::decode_i(OpcodeI::Jalr, instruction)),
            0b1100011 => match funct3 {
                0b000 => Some(Self::decode_b(OpcodeB::Beq, instruction)),
                0b001 => Some(Self::decode_b(OpcodeB::Bne, instruction)),
                0b100 => Some(Self::decode_b(OpcodeB::Blt, instruction)),
                0b101 => Some(Self::decode_b(OpcodeB::Bge, instruction)),
                0b110 => Some(Self::decode_b(OpcodeB::Bltu, instruction)),
                0b111 => Some(Self::decode_b(OpcodeB::Bgeu, instruction)),
                _ => None,
            },
            0b0000011 => match funct3 {
                0b000 => Some(Self::decode_i(OpcodeI::Lb, instruction)),
                0b001 => Some(Self::decode_i(OpcodeI::Lh, instruction)),
                0b010 => Some(Self::decode_i(OpcodeI::Lw, instruction)),
                0b100 => Some(Self::decode_i(OpcodeI::Lbu, instruction)),
                0b101 => Some(Self::decode_i(OpcodeI::Lhu, instruction)),
                _ => None,
            },
            0b0100011 => match funct3 {
                0b000 => Some(Self::decode_s(OpcodeS::Sb, instruction)),
                0b001 => Some(Self::decode_s(OpcodeS::Sh, instruction)),
                0b010 => Some(Self::decode_s(OpcodeS::Sw, instruction)),
                _ => None,
            },
            0b0010011 => match funct3 {
                0b000 => Some(Self::decode_i(OpcodeI::Addi, instruction)),
                0b010 => Some(Self::decode_i(OpcodeI::Slti, instruction)),
                0b011 => Some(Self::decode_i(OpcodeI::Sltiu, instruction)),
                0b100 => Some(Self::decode_i(OpcodeI::Xori, instruction)),
                0b110 => Some(Self::decode_i(OpcodeI::Ori, instruction)),
                0b111 => Some(Self::decode_i(OpcodeI::Andi, instruction)),
                0b001 => Some(Self::decode_i(OpcodeI::Slli, instruction)),
                0b101 => match funct7 {
                    0b0000000 => Some(Self::decode_i(OpcodeI::Srli, instruction)),
                    0b0100000 => Some(Self::decode_i(OpcodeI::Srai, instruction)),
                    _ => None,
                },
                _ => None,
            },
            0b0110011 => match funct3 {
                0b000 => match funct7 {
                    0b0000000 => Some(Self::decode_r(OpcodeR::Add, instruction)),
                    0b0100000 => Some(Self::decode_r(OpcodeR::Sub, instruction)),
                    _ => None,
                },
                0b001 => Some(Self::decode_r(OpcodeR::Sll, instruction)),
                0b010 => Some(Self::decode_r(OpcodeR::Slt, instruction)),
                0b011 => Some(Self::decode_r(OpcodeR::Sltu, instruction)),
                0b100 => Some(Self::decode_r(OpcodeR::Xor, instruction)),
                0b101 => match funct7 {
                    0b0000000 => Some(Self::decode_r(OpcodeR::Srl, instruction)),
                    0b0100000 => Some(Self::decode_r(OpcodeR::Sra, instruction)),
                    _ => None,
                },
                0b110 => Some(Self::decode_r(OpcodeR::Or, instruction)),
                0b111 => Some(Self::decode_r(OpcodeR::And, instruction)),
                _ => None,
            },
            0b0001111 => match funct3 {
                0b000 => Some(Self::decode_i(OpcodeI::Fence, instruction)),
                0b001 => Some(Self::decode_i(OpcodeI::FenceI, instruction)),
                _ => None,
            },
            0b1110011 => match funct3 {
                0b000 => match instruction >> 20 {
                    0b0 => Some(Self::decode_i(OpcodeI::Ecall, instruction)),
                    0b1 => Some(Self::decode_i(OpcodeI::Ebreak, instruction)),
                    _ => None,
                },
                0b001 => Some(Self::decode_i(OpcodeI::Csrrw, instruction)),
                0b010 => Some(Self::decode_i(OpcodeI::Csrrs, instruction)),
                0b011 => Some(Self::decode_i(OpcodeI::Csrrc, instruction)),
                0b101 => Some(Self::decode_i(OpcodeI::Csrrwi, instruction)),
                0b110 => Some(Self::decode_i(OpcodeI::Csrrsi, instruction)),
                0b111 => Some(Self::decode_i(OpcodeI::Csrrci, instruction)),
                _ => None,
            },
            _ => None,
        }
    }

    fn decode_r(opcode: OpcodeR, instruction: u32) -> Self {
        let rs1 = ((instruction >> 15) & MASK_5BIT) as usize;
        let rs2 = ((instruction >> 20) & MASK_5BIT) as usize;
        let rd = ((instruction >> 7) & MASK_5BIT) as usize;
        Self::TypeR {
            opcode,
            rs1,
            rs2,
            rd,
        }
    }

    fn decode_i(opcode: OpcodeI, instruction: u32) -> Self {
        let rd = ((instruction >> 7) & MASK_5BIT) as usize;
        let rs1 = ((instruction >> 15) & MASK_5BIT) as usize;
        let imm = ((instruction as i32 as i64) >> 20) as u64;
        Self::TypeI {
            opcode,
            rd,
            rs1,
            imm,
        }
    }

    fn decode_s(opcode: OpcodeS, instruction: u32) -> Self {
        let rs1 = ((instruction >> 15) & MASK_5BIT) as usize;
        let rs2 = ((instruction >> 20) & MASK_5BIT) as usize;
        let imm = ((instruction & 0xfe000000) as i32 as i64 >> 20) as u64
            | ((instruction >> 7) & 0x1f) as u64;
        Self::TypeS {
            opcode,
            rs1,
            rs2,
            imm,
        }
    }

    fn decode_b(opcode: OpcodeB, instruction: u32) -> Self {
        let rs1 = ((instruction >> 15) & MASK_5BIT) as usize;
        let rs2 = ((instruction >> 20) & MASK_5BIT) as usize;
        let imm = ((instruction & 0x80000000) as i32 as i64 >> 19) as u64
            | ((instruction & 0x80) << 4) as u64
            | ((instruction >> 20) & 0x7e0) as u64
            | ((instruction >> 8) & 0x1e) as u64;
        Self::TypeB {
            opcode,
            rs1,
            rs2,
            imm,
        }
    }

    fn decode_u(opcode: OpcodeU, instruction: u32) -> Self {
        let rd = ((instruction >> 7) & MASK_5BIT) as usize;
        let imm = (instruction & 0xfffff000) as i32 as i64 as u64;
        Self::TypeU { opcode, rd, imm }
    }

    fn decode_j(opcode: OpcodeJ, instruction: u32) -> Self {
        let rd = ((instruction >> 7) & MASK_5BIT) as usize;
        let imm = ((instruction & 0x80000000) as i32 as i64 >> 11) as u64
            | (instruction & 0xff000) as u64
            | ((instruction >> 9) & 0x800) as u64
            | ((instruction >> 20) & 0x7fe) as u64;
        Self::TypeJ { opcode, rd, imm }
    }
}

pub enum Instruction {
    TypeR {
        opcode: OpcodeR,
        rs1: usize,
        rs2: usize,
        rd: usize,
    },
    TypeI {
        opcode: OpcodeI,
        rs1: usize,
        rd: usize,
        imm: u64,
    },
    TypeS {
        opcode: OpcodeS,
        rs1: usize,
        rs2: usize,
        imm: u64,
    },
    TypeB {
        opcode: OpcodeB,
        rs1: usize,
        rs2: usize,
        imm: u64,
    },
    TypeU {
        opcode: OpcodeU,
        rd: usize,
        imm: u64,
    },
    TypeJ {
        opcode: OpcodeJ,
        rd: usize,
        imm: u64,
    },
}

pub enum OpcodeR {
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

pub enum OpcodeI {
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

pub enum OpcodeS {
    Sb,
    Sh,
    Sw,
}

pub enum OpcodeB {
    Beq,
    Bne,
    Blt,
    Bge,
    Bltu,
    Bgeu,
}

pub enum OpcodeU {
    Lui,
    Auipc,
}

pub enum OpcodeJ {
    Jal,
}
