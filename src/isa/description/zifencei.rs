use crate::isa::{
    description::{Describer, Description},
    instruction::{
        zifencei::{
            ZifenceiOpcodeB, ZifenceiOpcodeI, ZifenceiOpcodeJ, ZifenceiOpcodeR, ZifenceiOpcodeS,
            ZifenceiOpcodeU,
        },
        Instruction,
    },
};

impl Describer
    for Instruction<
        ZifenceiOpcodeR,
        ZifenceiOpcodeI,
        ZifenceiOpcodeS,
        ZifenceiOpcodeB,
        ZifenceiOpcodeU,
        ZifenceiOpcodeJ,
    >
{
    type OpcodeR = ZifenceiOpcodeR;
    type OpcodeI = ZifenceiOpcodeI;
    type OpcodeS = ZifenceiOpcodeS;
    type OpcodeB = ZifenceiOpcodeB;
    type OpcodeU = ZifenceiOpcodeU;
    type OpcodeJ = ZifenceiOpcodeJ;

    fn describe(&self) -> Description {
        let (description, assembly, signature, pseudocode) = match self {
            Self::TypeI {
                opcode,
                rd: _,
                funct3: _,
                rs1: _,
                imm: _,
            } => match opcode {
                ZifenceiOpcodeI::FenceI => (
                    "Fence Instruction Stream",
                    "fence.i",
                    "fence.i",
                    "Fence(Store, Fetch)",
                ),
            },
            _ => panic!(),
        };
        Description {
            description: description.to_string(),
            assembly: assembly.to_string(),
            singnature: signature.to_string(),
            pseudocode: pseudocode.to_string(),
        }
    }
}
