pub mod privileged;
pub mod rv32i;

use crate::{isa::instruction::Instruction, MASK_5BIT};

pub trait Decoder {
    type OpcodeR;
    type OpcodeI;
    type OpcodeS;
    type OpcodeB;
    type OpcodeU;
    type OpcodeJ;

    #[allow(clippy::type_complexity)]
    fn decode(
        instruction: u32,
    ) -> Option<
        Instruction<
            Self::OpcodeR,
            Self::OpcodeI,
            Self::OpcodeS,
            Self::OpcodeB,
            Self::OpcodeU,
            Self::OpcodeJ,
        >,
    >;

    #[allow(clippy::type_complexity)]
    fn decode_r(
        opcode: Option<Self::OpcodeR>,
        instruction: u32,
    ) -> Option<
        Instruction<
            Self::OpcodeR,
            Self::OpcodeI,
            Self::OpcodeS,
            Self::OpcodeB,
            Self::OpcodeU,
            Self::OpcodeJ,
        >,
    > {
        opcode.map(|o| {
            let rs1 = ((instruction >> 15) & MASK_5BIT) as usize;
            let rs2 = ((instruction >> 20) & MASK_5BIT) as usize;
            let rd = ((instruction >> 7) & MASK_5BIT) as usize;
            Instruction::TypeR {
                opcode: o,
                rs1,
                rs2,
                rd,
            }
        })
    }

    #[allow(clippy::type_complexity)]
    fn decode_i(
        opcode: Option<Self::OpcodeI>,
        instruction: u32,
    ) -> Option<
        Instruction<
            Self::OpcodeR,
            Self::OpcodeI,
            Self::OpcodeS,
            Self::OpcodeB,
            Self::OpcodeU,
            Self::OpcodeJ,
        >,
    > {
        opcode.map(|o| {
            let rd = ((instruction >> 7) & MASK_5BIT) as usize;
            let rs1 = ((instruction >> 15) & MASK_5BIT) as usize;
            let imm = ((instruction as i32) >> 20) as u32;
            Instruction::TypeI {
                opcode: o,
                rd,
                rs1,
                imm,
            }
        })
    }

    #[allow(clippy::type_complexity)]
    fn decode_s(
        opcode: Option<Self::OpcodeS>,
        instruction: u32,
    ) -> Option<
        Instruction<
            Self::OpcodeR,
            Self::OpcodeI,
            Self::OpcodeS,
            Self::OpcodeB,
            Self::OpcodeU,
            Self::OpcodeJ,
        >,
    > {
        opcode.map(|o| {
            let rs1 = ((instruction >> 15) & MASK_5BIT) as usize;
            let rs2 = ((instruction >> 20) & MASK_5BIT) as usize;
            let imm = ((instruction & 0xfe000000) as i32 >> 20) as u32 | (instruction >> 7) & 0x1f;
            Instruction::TypeS {
                opcode: o,
                rs1,
                rs2,
                imm,
            }
        })
    }

    #[allow(clippy::type_complexity)]
    fn decode_b(
        opcode: Option<Self::OpcodeB>,
        instruction: u32,
    ) -> Option<
        Instruction<
            Self::OpcodeR,
            Self::OpcodeI,
            Self::OpcodeS,
            Self::OpcodeB,
            Self::OpcodeU,
            Self::OpcodeJ,
        >,
    > {
        opcode.map(|o| {
            let rs1 = ((instruction >> 15) & MASK_5BIT) as usize;
            let rs2 = ((instruction >> 20) & MASK_5BIT) as usize;
            let imm = ((instruction & 0x80000000) as i32 >> 19) as u32
                | (instruction & 0x80) << 4
                | (instruction >> 20) & 0x7e0
                | (instruction >> 7) & 0x1e;
            Instruction::TypeB {
                opcode: o,
                rs1,
                rs2,
                imm,
            }
        })
    }

    #[allow(clippy::type_complexity)]
    fn decode_u(
        opcode: Option<Self::OpcodeU>,
        instruction: u32,
    ) -> Option<
        Instruction<
            Self::OpcodeR,
            Self::OpcodeI,
            Self::OpcodeS,
            Self::OpcodeB,
            Self::OpcodeU,
            Self::OpcodeJ,
        >,
    > {
        opcode.map(|o| {
            let rd = ((instruction >> 7) & MASK_5BIT) as usize;
            let imm = instruction & 0xfffff000;
            Instruction::TypeU { opcode: o, rd, imm }
        })
    }

    #[allow(clippy::type_complexity)]
    fn decode_j(
        opcode: Option<Self::OpcodeJ>,
        instruction: u32,
    ) -> Option<
        Instruction<
            Self::OpcodeR,
            Self::OpcodeI,
            Self::OpcodeS,
            Self::OpcodeB,
            Self::OpcodeU,
            Self::OpcodeJ,
        >,
    > {
        opcode.map(|o| {
            let rd = ((instruction >> 7) & MASK_5BIT) as usize;
            let imm = ((instruction & 0x80000000) as i32 >> 11) as u32
                | instruction & 0xff000
                | (instruction >> 9) & 0x800
                | (instruction >> 20) & 0x7fe;
            Instruction::TypeJ { opcode: o, rd, imm }
        })
    }
}
