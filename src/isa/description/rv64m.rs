use crate::isa::{
    description::{format3, Describer, Description},
    instruction::{
        rv64m::{
            Rv64mOpcodeB, Rv64mOpcodeI, Rv64mOpcodeJ, Rv64mOpcodeR, Rv64mOpcodeS, Rv64mOpcodeU,
        },
        Instruction,
    },
};

impl Describer
    for Instruction<
        Rv64mOpcodeR,
        Rv64mOpcodeI,
        Rv64mOpcodeS,
        Rv64mOpcodeB,
        Rv64mOpcodeU,
        Rv64mOpcodeJ,
    >
{
    type OpcodeR = Rv64mOpcodeR;
    type OpcodeI = Rv64mOpcodeI;
    type OpcodeS = Rv64mOpcodeS;
    type OpcodeB = Rv64mOpcodeB;
    type OpcodeU = Rv64mOpcodeU;
    type OpcodeJ = Rv64mOpcodeJ;

    fn describe(&self) -> Description {
        let (description, assembly, signature, pseudocode) = match self {
            Self::TypeR {
                opcode,
                rd,
                funct3: _,
                rs1,
                rs2,
                funct7: _,
            } => match opcode {
                Rv64mOpcodeR::Mulw => (
                    "Multiply Word",
                    format3(opcode.to_string(), rd, rs1, rs2),
                    "mulw rd,rs1,rs2",
                    "x[rd] = sext((x[rs1] * x[rs2])[31:0])",
                ),
                Rv64mOpcodeR::Divw => (
                    "Divide Word",
                    format3(opcode.to_string(), rd, rs1, rs2),
                    "divw rd,rs1,rs2",
                    "x[rd] = sext(x[rs1][31:0] /s x[rs2][31:0])",
                ),
                Rv64mOpcodeR::Divuw => (
                    "Divide Word, Unsigned",
                    format3(opcode.to_string(), rd, rs1, rs2),
                    "divuw rd,rs1,rs2",
                    "x[rd] = sext(x[rs1][31:0] /u x[rs2][31:0])",
                ),
                Rv64mOpcodeR::Remw => (
                    "Reminder Word",
                    format3(opcode.to_string(), rd, rs1, rs2),
                    "remw rd,rs1,rs2",
                    "x[rd] = sext(x[rs1][31:0] %s x[rs2][31:0])",
                ),
                Rv64mOpcodeR::Remuw => (
                    "Reminder Word, Unsigned",
                    format3(opcode.to_string(), rd, rs1, rs2),
                    "remuw rd,rs1,rs2",
                    "x[rd] = sext(x[rs1][31:0] %u x[rs2][31:0])",
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
