use std::fmt;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ZifenceiOpcodeR {}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ZifenceiOpcodeI {
    FenceI,
}

impl fmt::Display for ZifenceiOpcodeI {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::FenceI => f.write_str("fence.i"),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ZifenceiOpcodeS {}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ZifenceiOpcodeB {}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ZifenceiOpcodeU {}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ZifenceiOpcodeJ {}
