pub mod privileged;
pub mod rv32f;
pub mod rv32i;
pub mod rv32m;
pub mod rv64i;
pub mod rv64m;
pub mod zicsr;
pub mod zifencei;

use std::fmt;

fn format2(opcode: String, r1: &str, r2: &str) -> String {
    format!("{} {},{}", opcode, r1, r2)
}

fn format3(opcode: String, r1: &str, r2: &str, r3: &str) -> String {
    format!("{} {},{},{}", opcode, r1, r2, r3)
}

fn format4(opcode: String, r1: &str, r2: &str, r3: &str, r4: &str) -> String {
    format!("{} {},{},{},{}", opcode, r1, r2, r3, r4)
}

fn format_immediate(opcode: String, rd: &str, rs1: &str, imm: i64) -> String {
    format!("{} {},{},0x{:x}({})", opcode, rd, rs1, imm, imm)
}

fn format_upper_immediate(opcode: String, rd: &str, imm: i64) -> String {
    format!("{} {},0x{:x}", opcode, rd, imm)
}

fn format_offset(opcode: String, rd: &str, offset: i64, rs1: &str) -> String {
    format!("{} {},{:x}({})", opcode, rd, offset, rs1)
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
