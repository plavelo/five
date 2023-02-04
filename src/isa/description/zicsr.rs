use crate::isa::{
    description::{Describer, Description},
    instruction::{
        zicsr::{
            ZicsrOpcodeB, ZicsrOpcodeI, ZicsrOpcodeJ, ZicsrOpcodeR, ZicsrOpcodeS, ZicsrOpcodeU,
        },
        Instruction,
    },
    register::xname,
};

impl Describer
    for Instruction<
        ZicsrOpcodeR,
        ZicsrOpcodeI,
        ZicsrOpcodeS,
        ZicsrOpcodeB,
        ZicsrOpcodeU,
        ZicsrOpcodeJ,
    >
{
    type OpcodeR = ZicsrOpcodeR;
    type OpcodeI = ZicsrOpcodeI;
    type OpcodeS = ZicsrOpcodeS;
    type OpcodeB = ZicsrOpcodeB;
    type OpcodeU = ZicsrOpcodeU;
    type OpcodeJ = ZicsrOpcodeJ;

    fn describe(&self) -> Description {
        let (description, assembly, signature, pseudocode) = match *self {
            Self::TypeI {
                opcode,
                rd,
                funct3: _,
                rs1,
                imm,
            } => match opcode {
                ZicsrOpcodeI::Csrrw => (
                    "Control and Status Register Read and Write",
                    format!("{} {},{:x},{}", opcode, xname(rd), imm, xname(rs1),),
                    "csrrw rd,csr,rs1",
                    "t = CSRs[csr]; CSRs[csr] = x[rs1]; x[rd] = t",
                ),
                ZicsrOpcodeI::Csrrs => (
                    "Control And Status Register Read and Set",
                    format!("{} {},{:x},{}", opcode, xname(rd), imm, xname(rs1),),
                    "csrrs rd,csr,rs1",
                    "t = CSRs[csr]; CSRs[csr] = t | x[rs1]; x[rd] = t",
                ),
                ZicsrOpcodeI::Csrrc => (
                    "Control and Status Register Read and Clear",
                    format!("{} {},{:x},{}", opcode, xname(rd), imm, xname(rs1),),
                    "csrrc rd,csr,rs1",
                    "t = CSRs[csr]; CSRs[csr] = t &~ x[rs1]; x[rd] = t",
                ),
                ZicsrOpcodeI::Csrrwi => (
                    "Control and Status Register Read and Write Immediate",
                    format!("{} {},{:x},{:x}", opcode, xname(rd), imm, rs1),
                    "csrrwi rd,csr,zimm[4:0]",
                    "x[rd] = CSRs[csr]; CSRs[csr] = zimm",
                ),
                ZicsrOpcodeI::Csrrsi => (
                    "Control and Status Register RRead and Set Immediate",
                    format!("{} {},{:x},{:x}", opcode, xname(rd), imm, rs1),
                    "csrrsi rd,csr,zimm[4:0]",
                    "t= CSRs[csr]; CSRs[csr] = t | zimm; x[rd] = t",
                ),
                ZicsrOpcodeI::Csrrci => (
                    "Control and Status Register Read and Clear Immediate",
                    format!("{} {},{:x},{:x}", opcode, xname(rd), imm, rs1),
                    "csrrci rd,csr,zimm[4:0]",
                    "t= CSRs[csr]; CSRs[csr] = t &~ zimm; x[rd] = t",
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
