use crate::{
    bitops::{extend_sign, MASK_5BIT},
    isa::{
        description::{format2, format3, format4, format_offset, Describer, Description},
        instruction::{
            rv32f::{
                Rv32fOpcodeB, Rv32fOpcodeI, Rv32fOpcodeJ, Rv32fOpcodeR, Rv32fOpcodeS, Rv32fOpcodeU,
            },
            Instruction,
        },
        register::{fname, xname},
    },
};

impl Describer
    for Instruction<
        Rv32fOpcodeR,
        Rv32fOpcodeI,
        Rv32fOpcodeS,
        Rv32fOpcodeB,
        Rv32fOpcodeU,
        Rv32fOpcodeJ,
    >
{
    type OpcodeR = Rv32fOpcodeR;
    type OpcodeI = Rv32fOpcodeI;
    type OpcodeS = Rv32fOpcodeS;
    type OpcodeB = Rv32fOpcodeB;
    type OpcodeU = Rv32fOpcodeU;
    type OpcodeJ = Rv32fOpcodeJ;

    fn describe(&self) -> Description {
        let (description, assembly, signature, pseudocode) = match *self {
            Self::TypeR {
                opcode,
                rd,
                funct3: _,
                rs1,
                rs2,
                funct7,
            } => {
                let rs3 = (funct7 >> 2) & MASK_5BIT as usize;

                match opcode {
                    Rv32fOpcodeR::FmaddS => (
                        "Floating-point Fused Multiply-Add, Single-Precision",
                        format4(
                            opcode.to_string(),
                            fname(rd),
                            fname(rs1),
                            fname(rs2),
                            fname(rs3),
                        ),
                        "fmadd.s rd,rs1,rs2,rs3",
                        "f[rd] = f[rs1] * f[rs2] + f[rs3]",
                    ),
                    Rv32fOpcodeR::FmsubS => (
                        "Floating-point Fused Multiply-Subtract, Single-Precision",
                        format4(
                            opcode.to_string(),
                            fname(rd),
                            fname(rs1),
                            fname(rs2),
                            fname(rs3),
                        ),
                        "fmsub.s rd,rs1,rs2,rs3",
                        "f[rd] = f[rs1] * f[rs2] - f[rs3]",
                    ),
                    Rv32fOpcodeR::FnmsubS => (
                        "Floating-point Fused Negative Multiply-Subtract, Single-Precision",
                        format4(
                            opcode.to_string(),
                            fname(rd),
                            fname(rs1),
                            fname(rs2),
                            fname(rs3),
                        ),
                        "fnmsub.s rd,rs1,rs2,rs3",
                        "f[rd] = -f[rs1] * f[rs2] + f[rs3]",
                    ),
                    Rv32fOpcodeR::FnmaddS => (
                        "Floating-point Fused Negative Multiply-Add, Single-Precision",
                        format4(
                            opcode.to_string(),
                            fname(rd),
                            fname(rs1),
                            fname(rs2),
                            fname(rs3),
                        ),
                        "fnmadd.s rd,rs1,rs2,rs3",
                        "f[rd] = -f[rs1] * f[rs2] - f[rs3]",
                    ),
                    Rv32fOpcodeR::FaddS => (
                        "Floating-point Add, Single-Precision",
                        format3(opcode.to_string(), fname(rd), fname(rs1), fname(rs2)),
                        "fadd.s rd,rs1,rs2",
                        "f[rd] = f[rs1] + f[rs2]",
                    ),
                    Rv32fOpcodeR::FsubS => (
                        "Floating-point Subtract, Single-Precision",
                        format3(opcode.to_string(), fname(rd), fname(rs1), fname(rs2)),
                        "fsub.s rd,rs1,rs2",
                        "f[rd] = f[rs1] - f[rs2]",
                    ),
                    Rv32fOpcodeR::FmulS => (
                        "Floating-point Multiply, Single-Precision",
                        format3(opcode.to_string(), fname(rd), fname(rs1), fname(rs2)),
                        "fmul.s rd,rs1,rs2",
                        "f[rd] = f[rs1] * f[rs2]",
                    ),
                    Rv32fOpcodeR::FdivS => (
                        "Floating-point Divide, Single-Precision",
                        format3(opcode.to_string(), fname(rd), fname(rs1), fname(rs2)),
                        "fdiv.s rd,rs1,rs2",
                        "f[rd] = f[rs1] / f[rs2]",
                    ),
                    Rv32fOpcodeR::FsqrtS => (
                        "Floating-point Square Root, Single-Precision",
                        format2(opcode.to_string(), fname(rd), fname(rs1)),
                        "fsqrt.s rd,rs1",
                        "f[rd] = sqrt(f[rs1])",
                    ),
                    Rv32fOpcodeR::FsgnjS => (
                        "Floating-point Sign Inject, Single-Precision",
                        format3(opcode.to_string(), fname(rd), fname(rs1), fname(rs2)),
                        "fsgnj.s rd,rs1,rs2",
                        "f[rd] = {f[rs2][31], f[rs1][30:0]}",
                    ),
                    Rv32fOpcodeR::FsgnjnS => (
                        "Floating-point Sign Inject-Negate, Single-Precision",
                        format3(opcode.to_string(), fname(rd), fname(rs1), fname(rs2)),
                        "fsgnjn.s rd,rs1,rs2",
                        "f[rd] = {~f[rs2][31], f[rs1][30:0]}",
                    ),
                    Rv32fOpcodeR::FsgnjxS => (
                        "Floating-point Sign Inject-XOR, Single-Precision",
                        format3(opcode.to_string(), fname(rd), fname(rs1), fname(rs2)),
                        "fsgnjx.s rd,rs1,rs2",
                        "f[rd] = {f[rs1][31] ^ f[rs2][31], f[rs1][30:0]}",
                    ),
                    Rv32fOpcodeR::FminS => (
                        "Floating-point Minumum, Single-Precision",
                        format3(opcode.to_string(), fname(rd), fname(rs1), fname(rs2)),
                        "fmin.s rd,rs1,rs2",
                        "f[rd] = min(f[rs1], f[rs2])",
                    ),
                    Rv32fOpcodeR::FmaxS => (
                        "Floating-point Maximum, Single-Precision",
                        format3(opcode.to_string(), fname(rd), fname(rs1), fname(rs2)),
                        "fmax.s rd,rs1,rs2",
                        "f[rd] = max(f[rs1], f[rs2])",
                    ),
                    Rv32fOpcodeR::FcvtWs => (
                        "Floating-point Convert to Word from Single",
                        format2(opcode.to_string(), fname(rd), fname(rs1)),
                        "fcvt.w.s rd,rs1",
                        "f[rd] = sext(f32_to_s32(f[rs1]))",
                    ),
                    Rv32fOpcodeR::FcvtWuS => (
                        "Floating-point Convert to unsigned Word from Single",
                        format2(opcode.to_string(), fname(rd), fname(rs1)),
                        "fcvt.wu.s rd,rs1",
                        "f[rd] = sext(f32_to_u32(f[rs1]))",
                    ),
                    Rv32fOpcodeR::FmvXw => (
                        "Floating-point Move Word to Integer",
                        format3(opcode.to_string(), xname(rd), fname(rs1), fname(rs2)),
                        "fmv.x.w rd,rs1",
                        "x[rd] = sext(f[rs1][31:0])",
                    ),
                    Rv32fOpcodeR::FeqS => (
                        "Floating-point Equals, Single-Precision",
                        format3(opcode.to_string(), fname(rd), fname(rs1), fname(rs2)),
                        "feq.s rd,rs1,rs2",
                        "f[rd] = f[rs1] == f[rs2]",
                    ),
                    Rv32fOpcodeR::FltS => (
                        "Floating-point Less Than, Single-Precision",
                        format3(opcode.to_string(), fname(rd), fname(rs1), fname(rs2)),
                        "flt.s rd,rs1,rs2",
                        "f[rd] = f[rs1] < f[rs2]",
                    ),
                    Rv32fOpcodeR::FleS => (
                        "Floating-point Less Than or Equal, Single-Precision",
                        format3(opcode.to_string(), xname(rd), fname(rs1), fname(rs2)),
                        "fle.s rd,rs1,rs2",
                        "x[rd] = f[rs1] <= f[rs2]",
                    ),
                    Rv32fOpcodeR::FclassS => (
                        "Floating-point Classify, Single-Precision",
                        format2(opcode.to_string(), xname(rd), fname(rs1)),
                        "fclass.s rd,rs1",
                        "x[rd] = classify_s(f[rs1])",
                    ),
                    Rv32fOpcodeR::FcvtSw => (
                        "Floating-point Convert to Single from Word, Single-Precision",
                        format2(opcode.to_string(), fname(rd), fname(rs1)),
                        "fcvt.s.w rd,rs1",
                        "f[rd] = s32_to_f32(x[rs1])",
                    ),
                    Rv32fOpcodeR::FcvtSWu => (
                        "Floating-point Convert to Single from Unsigned Word",
                        format2(opcode.to_string(), fname(rd), fname(rs1)),
                        "fcvt.s.wu rd,rs1",
                        "f[rd] = u32_to_f32(x[rs1])",
                    ),
                    Rv32fOpcodeR::FmvWx => (
                        "Floating-point Move Word from Integer, Single-Precision",
                        format2(opcode.to_string(), fname(rd), fname(rs1)),
                        "fmv.w.x rd,rs1",
                        "f[rd] = x[rs1][31:0]",
                    ),
                }
            }
            Self::TypeI {
                opcode,
                rd,
                funct3: _,
                rs1,
                imm,
            } => match opcode {
                Rv32fOpcodeI::Flw => (
                    "Floating-point Load Word",
                    format_offset(
                        opcode.to_string(),
                        fname(rd),
                        extend_sign(imm, 12),
                        xname(rs1),
                    ),
                    "flw rd,offset(rs1)",
                    "f[rd] = mem[x[rs1] + sext(offset)][31:0]",
                ),
            },
            Self::TypeS {
                opcode,
                funct3: _,
                rs1,
                rs2,
                imm,
            } => match opcode {
                Rv32fOpcodeS::Fsw => (
                    "Floating-point Store Word",
                    format_offset(
                        opcode.to_string(),
                        fname(rs2),
                        extend_sign(imm, 12),
                        xname(rs1),
                    ),
                    "fsw rs2,offset(rs1)",
                    "mem[x[rs1] + sext(offset)] = f[rs2][31:0]",
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
