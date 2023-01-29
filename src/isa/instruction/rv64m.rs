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
            Self::Mulw => f.write_str("mulw"),
            Self::Divw => f.write_str("divw"),
            Self::Divuw => f.write_str("divuw"),
            Self::Remw => f.write_str("remw"),
            Self::Remuw => f.write_str("remuw"),
        }
    }
}

pub enum Rv64mOpcodeI {}

impl fmt::Display for Rv64mOpcodeI {
    fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
        fmt::Result::Err(fmt::Error)
    }
}

pub enum Rv64mOpcodeS {}

impl fmt::Display for Rv64mOpcodeS {
    fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
        fmt::Result::Err(fmt::Error)
    }
}

pub enum Rv64mOpcodeB {}

impl fmt::Display for Rv64mOpcodeB {
    fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
        fmt::Result::Err(fmt::Error)
    }
}

pub enum Rv64mOpcodeU {}

impl fmt::Display for Rv64mOpcodeU {
    fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
        fmt::Result::Err(fmt::Error)
    }
}

pub enum Rv64mOpcodeJ {}

impl fmt::Display for Rv64mOpcodeJ {
    fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
        fmt::Result::Err(fmt::Error)
    }
}
