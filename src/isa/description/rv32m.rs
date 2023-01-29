use crate::isa::{
    description::{format3, Describer, Description},
    instruction::{
        rv32m::{
            Rv32mOpcodeB, Rv32mOpcodeI, Rv32mOpcodeJ, Rv32mOpcodeR, Rv32mOpcodeS, Rv32mOpcodeU,
        },
        Instruction,
    },
};

impl Describer
    for Instruction<
        Rv32mOpcodeR,
        Rv32mOpcodeI,
        Rv32mOpcodeS,
        Rv32mOpcodeB,
        Rv32mOpcodeU,
        Rv32mOpcodeJ,
    >
{
    type OpcodeR = Rv32mOpcodeR;
    type OpcodeI = Rv32mOpcodeI;
    type OpcodeS = Rv32mOpcodeS;
    type OpcodeB = Rv32mOpcodeB;
    type OpcodeU = Rv32mOpcodeU;
    type OpcodeJ = Rv32mOpcodeJ;

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
                Rv32mOpcodeR::Mul => (
                    "Multiply",
                    format3(opcode.to_string(), rd, rs1, rs2),
                    "mul rd,rs1,rs2",
                    "x[rd] = x[rs1] * x[rs2]",
                ),
                Rv32mOpcodeR::Mulh => (
                    "Multiply High",
                    format3(opcode.to_string(), rd, rs1, rs2),
                    "mulh rd,rs1,rs2",
                    "x[rd] = (x[rs1] s*s x[rs2]) >>s XLEN",
                ),
                Rv32mOpcodeR::Mulhsu => (
                    "Multiply High Signed-Unsigned",
                    format3(opcode.to_string(), rd, rs1, rs2),
                    "mulhsu rd,rs1,rs2",
                    "x[rd] = (x[rs1] s*u x[rs2]) >>s XLEN",
                ),
                Rv32mOpcodeR::Mulhu => (
                    "Multiply High Unsined",
                    format3(opcode.to_string(), rd, rs1, rs2),
                    "mulhu rd,rs1,rs2",
                    "x[rd] = (x[rs1] u*u x[rs2]) >>u XLEN",
                ),
                Rv32mOpcodeR::Div => (
                    "Divide",
                    format3(opcode.to_string(), rd, rs1, rs2),
                    "div rd,rs1,rs2",
                    "x[rd] = x[rs1] /s x[rs2]",
                ),
                Rv32mOpcodeR::Divu => (
                    "Divide, Unsigned",
                    format3(opcode.to_string(), rd, rs1, rs2),
                    "divu rd,rs1,rs2",
                    "x[rd] = x[rs1] /u x[rs2]",
                ),
                Rv32mOpcodeR::Rem => (
                    "Reminder",
                    format3(opcode.to_string(), rd, rs1, rs2),
                    "rem rd,rs1,rs2",
                    "x[rd] = x[rs1] %s x[rs2]",
                ),
                Rv32mOpcodeR::Remu => (
                    "Reminder, Unsigned",
                    format3(opcode.to_string(), rd, rs1, rs2),
                    "remu rd,rs1,rs2",
                    "x[rd] = x[rs1] %u x[rs2]",
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
