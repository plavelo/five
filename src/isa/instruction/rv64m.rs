use std::fmt;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Rv64mOpcodeR {
    Mulw,
    Divw,
    Divuw,
    Remw,
    Remuw,
}

impl fmt::Display for Rv64mOpcodeR {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Mulw => f.write_str("mulw"),
            Self::Divw => f.write_str("divw"),
            Self::Divuw => f.write_str("divuw"),
            Self::Remw => f.write_str("remw"),
            Self::Remuw => f.write_str("remuw"),
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Rv64mOpcodeI {}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Rv64mOpcodeS {}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Rv64mOpcodeB {}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Rv64mOpcodeU {}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Rv64mOpcodeJ {}
