use crate::{
    emulator::cpu::decoder::{Decoder, MASK_5BIT, MASK_7BIT},
    isa::instruction::{
        rv64f::{
            Rv64fOpcodeB, Rv64fOpcodeI, Rv64fOpcodeJ, Rv64fOpcodeR, Rv64fOpcodeS, Rv64fOpcodeU,
        },
        Instruction,
    },
};

pub struct Rv64fDecoder;

impl Decoder for Rv64fDecoder {
    type OpcodeR = Rv64fOpcodeR;
    type OpcodeI = Rv64fOpcodeI;
    type OpcodeS = Rv64fOpcodeS;
    type OpcodeB = Rv64fOpcodeB;
    type OpcodeU = Rv64fOpcodeU;
    type OpcodeJ = Rv64fOpcodeJ;

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
        let rs2 = (instruction >> 20) & MASK_5BIT;
        let funct7 = (instruction >> 25) & MASK_7BIT;
        match opcode {
            0b1010011 => Self::decode_r(
                match funct7 {
                    0b1100000 => match rs2 {
                        0b00010 => Some(Rv64fOpcodeR::FcvtLS),
                        0b00011 => Some(Rv64fOpcodeR::FcvtLuS),
                        _ => None,
                    },
                    0b1101000 => match rs2 {
                        0b00010 => Some(Rv64fOpcodeR::FcvtSL),
                        0b00011 => Some(Rv64fOpcodeR::FcvtSLu),
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
