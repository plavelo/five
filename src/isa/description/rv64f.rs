use crate::isa::{
    description::{format2, Describer, Description},
    instruction::{
        rv64f::{
            Rv64fOpcodeB, Rv64fOpcodeI, Rv64fOpcodeJ, Rv64fOpcodeR, Rv64fOpcodeS, Rv64fOpcodeU,
        },
        Instruction,
    },
    register::{fname, xname},
};

impl Describer
    for Instruction<
        Rv64fOpcodeR,
        Rv64fOpcodeI,
        Rv64fOpcodeS,
        Rv64fOpcodeB,
        Rv64fOpcodeU,
        Rv64fOpcodeJ,
    >
{
    type OpcodeR = Rv64fOpcodeR;
    type OpcodeI = Rv64fOpcodeI;
    type OpcodeS = Rv64fOpcodeS;
    type OpcodeB = Rv64fOpcodeB;
    type OpcodeU = Rv64fOpcodeU;
    type OpcodeJ = Rv64fOpcodeJ;

    fn describe(&self) -> Description {
        let (description, assembly, signature, pseudocode) = match *self {
            Self::TypeR {
                opcode,
                rd,
                funct3: _,
                rs1,
                rs2: _,
                funct7: _,
            } => match opcode {
                Rv64fOpcodeR::FcvtLS => (
                    "Floating-point Convert to Long from Single",
                    format2(opcode.to_string(), xname(rd), fname(rs1)),
                    "fcvt.l.s rd,rs1",
                    "x[rd] = f32_to_s64(f[rs1])",
                ),
                Rv64fOpcodeR::FcvtLuS => (
                    "Floating-point Convert to Unsigned Long from Single",
                    format2(opcode.to_string(), xname(rd), fname(rs1)),
                    "fcvt.lu.s rd,rs1",
                    "x[rd] = f32_to_u64(f[rs1])",
                ),
                Rv64fOpcodeR::FcvtSL => (
                    "Floating-point Convert to Single from Long",
                    format2(opcode.to_string(), fname(rd), xname(rs1)),
                    "fcvt.s.l rd,rs1",
                    "f[rd] = s64_to_f32(x[rs1])",
                ),
                Rv64fOpcodeR::FcvtSLu => (
                    "Floating-point Convert to Single from Unsigned Long",
                    format2(opcode.to_string(), fname(rd), xname(rs1)),
                    "fcvt.s.lu rd,rs1",
                    "f[rd] = u64_to_f32(x[rs1])",
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
