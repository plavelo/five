use std::fmt;

pub enum ZifenceiOpcodeR {}

impl fmt::Display for ZifenceiOpcodeR {
    fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
        fmt::Result::Err(fmt::Error)
    }
}

#[derive(Debug, PartialEq)]
pub enum ZifenceiOpcodeI {
    FenceI,
}

impl fmt::Display for ZifenceiOpcodeI {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::FenceI => f.write_str("fence.i"),
        }
    }
}

pub enum ZifenceiOpcodeS {}

impl fmt::Display for ZifenceiOpcodeS {
    fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
        fmt::Result::Err(fmt::Error)
    }
}

pub enum ZifenceiOpcodeB {}

impl fmt::Display for ZifenceiOpcodeB {
    fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
        fmt::Result::Err(fmt::Error)
    }
}

pub enum ZifenceiOpcodeU {}

impl fmt::Display for ZifenceiOpcodeU {
    fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
        fmt::Result::Err(fmt::Error)
    }
}

pub enum ZifenceiOpcodeJ {}

impl fmt::Display for ZifenceiOpcodeJ {
    fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
        fmt::Result::Err(fmt::Error)
    }
}
