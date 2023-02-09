pub mod privileged;
pub mod rv32f;
pub mod rv32i;
pub mod rv32m;
pub mod rv64f;
pub mod rv64i;
pub mod rv64m;
pub mod zicsr;
pub mod zifencei;

use crate::isa::register::xname;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Instruction<OpcodeR, OpcodeI, OpcodeS, OpcodeB, OpcodeU, OpcodeJ> {
    TypeR {
        opcode: OpcodeR,
        rd: usize,
        funct3: usize,
        rs1: usize,
        rs2: usize,
        funct7: usize,
    },
    TypeI {
        opcode: OpcodeI,
        rd: usize,
        funct3: usize,
        rs1: usize,
        imm: u64,
    },
    TypeS {
        opcode: OpcodeS,
        funct3: usize,
        rs1: usize,
        rs2: usize,
        imm: u64,
    },
    TypeB {
        opcode: OpcodeB,
        funct3: usize,
        rs1: usize,
        rs2: usize,
        imm: u64,
    },
    TypeU {
        opcode: OpcodeU,
        rd: usize,
        imm: u64,
    },
    TypeJ {
        opcode: OpcodeJ,
        rd: usize,
        imm: u64,
    },
}

impl<
        OpcodeR: fmt::Display,
        OpcodeI: fmt::Display,
        OpcodeS: fmt::Display,
        OpcodeB: fmt::Display,
        OpcodeU: fmt::Display,
        OpcodeJ: fmt::Display,
    > fmt::Display for Instruction<OpcodeR, OpcodeI, OpcodeS, OpcodeB, OpcodeU, OpcodeJ>
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::TypeR {
                opcode,
                rd,
                funct3,
                rs1,
                rs2,
                funct7,
            } => write!(
                f,
                "{}, rd: {}, funct3: {}, rs1: {}, rs2: {}, funct7: {}",
                opcode,
                xname(*rd),
                funct3,
                xname(*rs1),
                xname(*rs2),
                funct7,
            ),
            Self::TypeI {
                opcode,
                rd,
                funct3,
                rs1,
                imm,
            } => write!(
                f,
                "{}, rd: {}, funct3: {}, rs1: {}, imm: {:x}",
                opcode,
                xname(*rd),
                funct3,
                xname(*rs1),
                imm,
            ),
            Self::TypeS {
                opcode,
                funct3,
                rs1,
                rs2,
                imm,
            } => write!(
                f,
                "{}, funct3: {}, rs1: {}, rs2: {}, imm: {:x}",
                opcode,
                funct3,
                xname(*rs1),
                xname(*rs2),
                imm,
            ),
            Self::TypeB {
                opcode,
                funct3,
                rs1,
                rs2,
                imm,
            } => write!(
                f,
                "{}, funct3: {}, rs1: {}, rs2: {}, imm: {:x}",
                opcode,
                funct3,
                xname(*rs1),
                xname(*rs2),
                imm,
            ),
            Self::TypeU { opcode, rd, imm } => {
                write!(f, "{}, rd: {}, imm: {:x}", opcode, xname(*rd), imm,)
            }
            Self::TypeJ { opcode, rd, imm } => {
                write!(f, "{}, rd: {}, imm: {:x}", opcode, xname(*rd), imm,)
            }
        }
    }
}
