pub mod privileged;
pub mod rv32f;
pub mod rv32i;
pub mod rv32m;
pub mod rv64i;
pub mod rv64m;
pub mod zicsr;
pub mod zifencei;

use crate::isa::register::register_name;
use std::fmt;

const MASK_5BIT: u64 = 0b11111;
const MASK_6BIT: u64 = 0b111111;

fn format2(opcode: String, x1: &usize, x2: &usize) -> String {
    format!("{} {},{}", opcode, register_name(*x1), register_name(*x2),)
}

fn format3(opcode: String, x1: &usize, x2: &usize, x3: &usize) -> String {
    format!(
        "{} {},{},{}",
        opcode,
        register_name(*x1),
        register_name(*x2),
        register_name(*x3),
    )
}

fn format4(opcode: String, x1: &usize, x2: &usize, x3: &usize, x4: &usize) -> String {
    format!(
        "{} {},{},{},{}",
        opcode,
        register_name(*x1),
        register_name(*x2),
        register_name(*x3),
        register_name(*x4),
    )
}

fn format_immediate(opcode: String, x1: &usize, x2: &usize, imm: &u64) -> String {
    format!(
        "{} {},{},0x{:x}",
        opcode,
        register_name(*x1),
        register_name(*x2),
        imm,
    )
}

fn format_upper_immediate(opcode: String, x1: &usize, imm: &u64) -> String {
    format!("{} {},0x{:x}", opcode, register_name(*x1), imm)
}

fn format_offset(opcode: String, x1: &usize, offset: &u64, x2: &usize) -> String {
    format!(
        "{} {},{}({})",
        opcode,
        register_name(*x1),
        offset,
        register_name(*x2),
    )
}

pub struct Description {
    description: String,
    singnature: String,
    assembly: String,
    pseudocode: String,
}

impl fmt::Display for Description {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}\n[{}] {} ({})",
            self.assembly, self.description, self.singnature, self.pseudocode,
        )
    }
}

pub trait Describer {
    type OpcodeR;
    type OpcodeI;
    type OpcodeS;
    type OpcodeB;
    type OpcodeU;
    type OpcodeJ;

    fn describe(&self) -> Description;
}
