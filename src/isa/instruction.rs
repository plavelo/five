pub mod privileged;
pub mod rv32i;

use crate::isa::instruction::{privileged::PrivilegedInstruction, rv32i::Rv32iInstruction};

pub enum Instruction {
    Privileged(PrivilegedInstruction),
    Rv32i(Rv32iInstruction),
}

impl Instruction {
    pub fn decode(instruction: u32) -> Option<Self> {
        if let Some(decoded) = PrivilegedInstruction::decode(instruction) {
            return Some(Instruction::Privileged(decoded));
        }
        if let Some(decoded) = Rv32iInstruction::decode(instruction) {
            return Some(Instruction::Rv32i(decoded));
        }
        None
    }
}
