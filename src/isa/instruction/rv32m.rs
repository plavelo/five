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
            Self::Mul => f.write_str("mul"),
            Self::Mulh => f.write_str("mulh"),
            Self::Mulhsu => f.write_str("mulhsu"),
            Self::Mulhu => f.write_str("mulhu"),
            Self::Div => f.write_str("div"),
            Self::Divu => f.write_str("divu"),
            Self::Rem => f.write_str("rem"),
            Self::Remu => f.write_str("remu"),
        }
    }
}

pub enum Rv32mOpcodeI {}

impl fmt::Display for Rv32mOpcodeI {
    fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
        fmt::Result::Err(fmt::Error)
    }
}

pub enum Rv32mOpcodeS {}

impl fmt::Display for Rv32mOpcodeS {
    fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
        fmt::Result::Err(fmt::Error)
    }
}

pub enum Rv32mOpcodeB {}

impl fmt::Display for Rv32mOpcodeB {
    fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
        fmt::Result::Err(fmt::Error)
    }
}

pub enum Rv32mOpcodeU {}

impl fmt::Display for Rv32mOpcodeU {
    fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
        fmt::Result::Err(fmt::Error)
    }
}

pub enum Rv32mOpcodeJ {}

impl fmt::Display for Rv32mOpcodeJ {
    fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
        fmt::Result::Err(fmt::Error)
    }
}
