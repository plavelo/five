pub mod privileged;
pub mod rv32f;
pub mod rv32i;
pub mod rv32m;
pub mod rv64i;
pub mod rv64m;
pub mod zicsr;
pub mod zifencei;

use crate::isa::register::{to_fname, to_xname};
use std::fmt;

fn xformat2(opcode: String, x1: usize, x2: usize) -> String {
    format!("{} {},{}", opcode, to_xname(x1), to_xname(x2),)
}

fn fformat2(opcode: String, f1: usize, f2: usize) -> String {
    format!("{} {},{}", opcode, to_fname(f1), to_fname(f2),)
}

fn fformat2x(opcode: String, x1: usize, f1: usize) -> String {
    format!("{} {},{}", opcode, to_xname(x1), to_fname(f1),)
}

fn xformat3(opcode: String, x1: usize, x2: usize, x3: usize) -> String {
    format!(
        "{} {},{},{}",
        opcode,
        to_xname(x1),
        to_xname(x2),
        to_xname(x3),
    )
}

fn fformat3(opcode: String, f1: usize, f2: usize, f3: usize) -> String {
    format!(
        "{} {},{},{}",
        opcode,
        to_fname(f1),
        to_fname(f2),
        to_fname(f3),
    )
}

fn fformat3x(opcode: String, x1: usize, f1: usize, f2: usize) -> String {
    format!(
        "{} {},{},{}",
        opcode,
        to_xname(x1),
        to_fname(f1),
        to_fname(f2),
    )
}

fn fformat4(opcode: String, f1: usize, f2: usize, f3: usize, f4: usize) -> String {
    format!(
        "{} {},{},{},{}",
        opcode,
        to_fname(f1),
        to_fname(f2),
        to_fname(f3),
        to_fname(f4),
    )
}

fn xformat_immediate(opcode: String, x1: usize, x2: usize, imm: i64) -> String {
    format!(
        "{} {},{},0x{:x}({})",
        opcode,
        to_xname(x1),
        to_xname(x2),
        imm,
        imm,
    )
}

fn xformat_upper_immediate(opcode: String, x1: usize, imm: i64) -> String {
    format!("{} {},0x{:x}", opcode, to_xname(x1), imm)
}

fn xformat_offset(opcode: String, x1: usize, offset: i64, x2: usize) -> String {
    format!("{} {},{:X}({})", opcode, to_xname(x1), offset, to_xname(x2),)
}

fn fformat_offset(opcode: String, f1: usize, offset: i64, f2: usize) -> String {
    format!("{} {},{:X}({})", opcode, to_fname(f1), offset, to_fname(f2),)
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
            "{}\n[{}] {} {{ {} }}",
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
