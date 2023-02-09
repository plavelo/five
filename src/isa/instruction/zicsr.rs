use std::fmt;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum ZicsrOpcodeR {}

#[derive(Debug, PartialEq, Copy, Clone)]
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
        match self {
            Self::Csrrw => f.write_str("csrrw"),
            Self::Csrrs => f.write_str("csrrs"),
            Self::Csrrc => f.write_str("csrrc"),
            Self::Csrrwi => f.write_str("csrrwi"),
            Self::Csrrsi => f.write_str("csrrsi"),
            Self::Csrrci => f.write_str("csrrci"),
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum ZicsrOpcodeS {}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum ZicsrOpcodeB {}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum ZicsrOpcodeU {}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum ZicsrOpcodeJ {}
