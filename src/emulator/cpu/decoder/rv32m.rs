use crate::{
    emulator::cpu::decoder::{Decoder, MASK_3BIT, MASK_7BIT},
    isa::instruction::{
        rv32m::{
            Rv32mOpcodeB, Rv32mOpcodeI, Rv32mOpcodeJ, Rv32mOpcodeR, Rv32mOpcodeS, Rv32mOpcodeU,
        },
        Instruction,
    },
};

pub struct Rv32mDecoder;

impl Decoder for Rv32mDecoder {
    type OpcodeR = Rv32mOpcodeR;
    type OpcodeI = Rv32mOpcodeI;
    type OpcodeS = Rv32mOpcodeS;
    type OpcodeB = Rv32mOpcodeB;
    type OpcodeU = Rv32mOpcodeU;
    type OpcodeJ = Rv32mOpcodeJ;

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
            0b0110011 => Self::decode_r(
                match funct7 {
                    0b1 => match funct3 {
                        0b000 => Some(Rv32mOpcodeR::Mul),
                        0b001 => Some(Rv32mOpcodeR::Mulh),
                        0b010 => Some(Rv32mOpcodeR::Mulhsu),
                        0b011 => Some(Rv32mOpcodeR::Mulhu),
                        0b100 => Some(Rv32mOpcodeR::Div),
                        0b101 => Some(Rv32mOpcodeR::Divu),
                        0b110 => Some(Rv32mOpcodeR::Rem),
                        0b111 => Some(Rv32mOpcodeR::Remu),
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
