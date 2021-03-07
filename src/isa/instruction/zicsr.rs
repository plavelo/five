use std::fmt;

pub enum ZicsrOpcodeR {}

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
            ZicsrOpcodeI::Csrrw => f.write_str("Zicsr::Csrrw"),
            ZicsrOpcodeI::Csrrs => f.write_str("Zicsr::Csrrs"),
            ZicsrOpcodeI::Csrrc => f.write_str("Zicsr::Csrrc"),
            ZicsrOpcodeI::Csrrwi => f.write_str("Zicsr::Csrrwi"),
            ZicsrOpcodeI::Csrrsi => f.write_str("Zicsr::Csrrsi"),
            ZicsrOpcodeI::Csrrci => f.write_str("Zicsr::Csrrci"),
        }
    }
}

pub enum ZicsrOpcodeS {}
pub enum ZicsrOpcodeB {}
pub enum ZicsrOpcodeU {}
pub enum ZicsrOpcodeJ {}
