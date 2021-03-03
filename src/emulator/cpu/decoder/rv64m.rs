use crate::{
    emulator::cpu::decoder::{Decoder, MASK_3BIT, MASK_7BIT},
    isa::instruction::{
        rv64m::{
            Rv64mOpcodeB, Rv64mOpcodeI, Rv64mOpcodeJ, Rv64mOpcodeR, Rv64mOpcodeS, Rv64mOpcodeU,
        },
        Instruction,
    },
};

pub struct Rv64mDecoder;

impl Decoder for Rv64mDecoder {
    type OpcodeR = Rv64mOpcodeR;
    type OpcodeI = Rv64mOpcodeI;
    type OpcodeS = Rv64mOpcodeS;
    type OpcodeB = Rv64mOpcodeB;
    type OpcodeU = Rv64mOpcodeU;
    type OpcodeJ = Rv64mOpcodeJ;

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
    > {
        let opcode = instruction & MASK_7BIT;
        let funct3 = (instruction >> 12) & MASK_3BIT;
        let funct7 = (instruction >> 25) & MASK_7BIT;
        match opcode {
            0b0111011 => Self::decode_r(
                match funct7 {
                    0b1 => match funct3 {
                        0b000 => Some(Rv64mOpcodeR::Mulw),
                        0b100 => Some(Rv64mOpcodeR::Divw),
                        0b110 => Some(Rv64mOpcodeR::Remw),
                        0b111 => Some(Rv64mOpcodeR::Remuw),
                        _ => None,
                    },
                    _ => None,
                },
                instruction,
            ),
            _ => None,
        }
    }
}
