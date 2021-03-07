use crate::{
    emulator::cpu::decoder::{Decoder, MASK_3BIT, MASK_7BIT},
    isa::instruction::{
        zifencei::{
            ZifenceiOpcodeB, ZifenceiOpcodeI, ZifenceiOpcodeJ, ZifenceiOpcodeR, ZifenceiOpcodeS,
            ZifenceiOpcodeU,
        },
        Instruction,
    },
};

pub struct ZifenceiDecoder;

impl Decoder for ZifenceiDecoder {
    type OpcodeR = ZifenceiOpcodeR;
    type OpcodeI = ZifenceiOpcodeI;
    type OpcodeS = ZifenceiOpcodeS;
    type OpcodeB = ZifenceiOpcodeB;
    type OpcodeU = ZifenceiOpcodeU;
    type OpcodeJ = ZifenceiOpcodeJ;

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
        match opcode {
            0b0001111 => Self::decode_i(
                match funct3 {
                    0b001 => Some(ZifenceiOpcodeI::FenceI),
                    _ => None,
                },
                instruction,
            ),
            _ => None,
        }
    }
}
