use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Rv64mOpcodeR {
    Mulw,
    Divw,
    Divuw,
    Remw,
    Remuw,
}

impl fmt::Display for Rv64mOpcodeR {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Rv64mOpcodeR::Mulw => f.write_str("Rv64m::Mulw"),
            Rv64mOpcodeR::Divw => f.write_str("Rv64m::Divw"),
            Rv64mOpcodeR::Divuw => f.write_str("Rv64m::Divuw"),
            Rv64mOpcodeR::Remw => f.write_str("Rv64m::Remw"),
            Rv64mOpcodeR::Remuw => f.write_str("Rv64m::Remuw"),
        }
    }
}

pub enum Rv64mOpcodeI {}
pub enum Rv64mOpcodeS {}
pub enum Rv64mOpcodeB {}
pub enum Rv64mOpcodeU {}
pub enum Rv64mOpcodeJ {}
