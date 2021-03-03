use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Rv32mOpcodeR {
    Mul,
    Mulh,
    Mulhsu,
    Mulhu,
    Div,
    Divu,
    Rem,
    Remu,
}

impl fmt::Display for Rv32mOpcodeR {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Rv32mOpcodeR::Mul => f.write_str("Rv32m::Mul"),
            Rv32mOpcodeR::Mulh => f.write_str("Rv32m::Mulh"),
            Rv32mOpcodeR::Mulhsu => f.write_str("Rv32m::Mulhsu"),
            Rv32mOpcodeR::Mulhu => f.write_str("Rv32m::Mulhu"),
            Rv32mOpcodeR::Div => f.write_str("Rv32m::Div"),
            Rv32mOpcodeR::Divu => f.write_str("Rv32m::Divu"),
            Rv32mOpcodeR::Rem => f.write_str("Rv32m::Rem"),
            Rv32mOpcodeR::Remu => f.write_str("Rv32m::Remu"),
        }
    }
}

pub enum Rv32mOpcodeI {}
pub enum Rv32mOpcodeS {}
pub enum Rv32mOpcodeB {}
pub enum Rv32mOpcodeU {}
pub enum Rv32mOpcodeJ {}
