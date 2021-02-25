use crate::{
    emulator::cpu::decoder::{Decoder, MASK_3BIT, MASK_7BIT},
    isa::instruction::{
        rv64i::{
            Rv64iOpcodeB, Rv64iOpcodeI, Rv64iOpcodeJ, Rv64iOpcodeR, Rv64iOpcodeS, Rv64iOpcodeU,
        },
        Instruction,
    },
};

pub struct Rv64iDecoder;

impl Decoder for Rv64iDecoder {
    type OpcodeR = Rv64iOpcodeR;
    type OpcodeI = Rv64iOpcodeI;
    type OpcodeS = Rv64iOpcodeS;
    type OpcodeB = Rv64iOpcodeB;
    type OpcodeU = Rv64iOpcodeU;
    type OpcodeJ = Rv64iOpcodeJ;

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
                match funct3 {
                    0b000 => match funct7 {
                        0b0000000 => Some(Rv64iOpcodeR::Addw),
                        0b0100000 => Some(Rv64iOpcodeR::Subw),
                        _ => None,
                    },
                    0b001 => Some(Rv64iOpcodeR::Sllw),
                    0b101 => match funct7 {
                        0b0000000 => Some(Rv64iOpcodeR::Srlw),
                        0b0100000 => Some(Rv64iOpcodeR::Sraw),
                        _ => None,
                    },
                    _ => None,
                },
                instruction,
            ),
            0b0011011 => Self::decode_i(
                match funct3 {
                    0b000 => Some(Rv64iOpcodeI::Addiw),
                    0b001 => Some(Rv64iOpcodeI::Slliw),
                    0b101 => match funct7 {
                        0b0000000 => Some(Rv64iOpcodeI::Srliw),
                        0b0100000 => Some(Rv64iOpcodeI::Sraiw),
                        _ => None,
                    },
                    _ => None,
                },
                instruction,
            ),
            0b0000011 => Self::decode_i(
                match funct3 {
                    0b011 => Some(Rv64iOpcodeI::Ld),
                    0b110 => Some(Rv64iOpcodeI::Lwu),
                    _ => None,
                },
                instruction,
            ),
            0b0100011 => Self::decode_s(
                match funct3 {
                    0b011 => Some(Rv64iOpcodeS::Sd),
                    _ => None,
                },
                instruction,
            ),
            _ => None,
        }
    }
}
