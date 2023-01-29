use std::fmt;

pub enum ZicsrOpcodeR {}

impl fmt::Display for ZicsrOpcodeR {
    fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
        fmt::Result::Err(fmt::Error)
    }
}

#[derive(Debug, PartialEq)]
pub enum ZicsrOpcodeI {
    Csrrw,
    Csrrs,
    Csrrc,
    Csrrwi,
    Csrrsi,
    Csrrci,
}

impl fmt::Display for ZicsrOpcodeI {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Csrrw => f.write_str("csrrw"),
            Self::Csrrs => f.write_str("csrrs"),
            Self::Csrrc => f.write_str("csrrc"),
            Self::Csrrwi => f.write_str("csrrwi"),
            Self::Csrrsi => f.write_str("csrrsi"),
            Self::Csrrci => f.write_str("csrrci"),
        }
    }
}

pub enum ZicsrOpcodeS {}

impl fmt::Display for ZicsrOpcodeS {
    fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
        fmt::Result::Err(fmt::Error)
    }
}

pub enum ZicsrOpcodeB {}

impl fmt::Display for ZicsrOpcodeB {
    fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
        fmt::Result::Err(fmt::Error)
    }
}

pub enum ZicsrOpcodeU {}

impl fmt::Display for ZicsrOpcodeU {
    fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
        fmt::Result::Err(fmt::Error)
    }
}

pub enum ZicsrOpcodeJ {}

impl fmt::Display for ZicsrOpcodeJ {
    fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
        fmt::Result::Err(fmt::Error)
    }
}
