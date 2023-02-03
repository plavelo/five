use crate::isa::{
    description::{xformat2, Describer, Description},
    instruction::{
        privileged::{
            PrivilegedOpcodeB, PrivilegedOpcodeI, PrivilegedOpcodeJ, PrivilegedOpcodeR,
            PrivilegedOpcodeS, PrivilegedOpcodeU,
        },
        Instruction,
    },
};

impl Describer
    for Instruction<
        PrivilegedOpcodeR,
        PrivilegedOpcodeI,
        PrivilegedOpcodeS,
        PrivilegedOpcodeB,
        PrivilegedOpcodeU,
        PrivilegedOpcodeJ,
    >
{
    type OpcodeR = PrivilegedOpcodeR;
    type OpcodeI = PrivilegedOpcodeI;
    type OpcodeS = PrivilegedOpcodeS;
    type OpcodeB = PrivilegedOpcodeB;
    type OpcodeU = PrivilegedOpcodeU;
    type OpcodeJ = PrivilegedOpcodeJ;

    fn describe(&self) -> Description {
        let (description, assembly, signature, pseudocode) = match *self {
            Self::TypeR {
                opcode,
                rd: _,
                funct3: _,
                rs1,
                rs2,
                funct7: _,
            } => match opcode {
                PrivilegedOpcodeR::Uret => (
                    "User-mode Exception Return",
                    "uret".to_string(),
                    "uret",
                    "ExceptionReturn(User)",
                ),
                PrivilegedOpcodeR::Sret => (
                    "Supervisor-mode Exception Return",
                    "sret".to_string(),
                    "sret",
                    "ExceptionReturn(Supervisor)",
                ),
                PrivilegedOpcodeR::Mret => (
                    "Machine-mode Exception Return",
                    "mret".to_string(),
                    "mret",
                    "ExceptionReturn(Machine)",
                ),
                PrivilegedOpcodeR::Wfi => (
                    "Wait for Interrupt",
                    "wfi".to_string(),
                    "wfi",
                    "while (noInterruptsPending) idle",
                ),
                PrivilegedOpcodeR::SfenceVma => (
                    "Fence Virtual Memory",
                    xformat2(opcode.to_string(), rs1, rs2),
                    "sfence.vma rs1,rs2",
                    "while (noInterruptsPending) idle",
                ),
            },
            _ => panic!(),
        };
        Description {
            description: description.to_string(),
            assembly,
            singnature: signature.to_string(),
            pseudocode: pseudocode.to_string(),
        }
    }
}
