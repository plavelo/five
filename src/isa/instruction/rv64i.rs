use std::fmt;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Rv64iOpcodeR {
    Sllw,
    Srlw,
    Sraw,
    Addw,
    Subw,
}

impl fmt::Display for Rv64iOpcodeR {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Sllw => f.write_str("Sllw"),
            Self::Srlw => f.write_str("Srlw"),
            Self::Sraw => f.write_str("Sraw"),
            Self::Addw => f.write_str("Addw"),
            Self::Subw => f.write_str("Subw"),
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Rv64iOpcodeI {
    Slliw,
    Srliw,
    Sraiw,
    Addiw,
    Lwu,
    Ld,
}

impl fmt::Display for Rv64iOpcodeI {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Slliw => f.write_str("slliw"),
            Self::Srliw => f.write_str("srliw"),
            Self::Sraiw => f.write_str("sraiw"),
            Self::Addiw => f.write_str("addiw"),
            Self::Lwu => f.write_str("lwu"),
            Self::Ld => f.write_str("ld"),
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Rv64iOpcodeS {
    Sd,
}

impl fmt::Display for Rv64iOpcodeS {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Sd => f.write_str("sd"),
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Rv64iOpcodeB {}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Rv64iOpcodeU {}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Rv64iOpcodeJ {}
