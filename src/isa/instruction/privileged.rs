use std::fmt;

#[derive(Debug, PartialEq)]
pub enum PrivilegedOpcodeR {
    Uret,
    Sret,
    Mret,
    Wfi,
    SfenceVma,
}

impl fmt::Display for PrivilegedOpcodeR {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Uret => f.write_str("uret"),
            Self::Sret => f.write_str("sret"),
            Self::Mret => f.write_str("mret"),
            Self::Wfi => f.write_str("wfi"),
            Self::SfenceVma => f.write_str("sfence.vma"),
        }
    }
}

pub enum PrivilegedOpcodeI {}

impl fmt::Display for PrivilegedOpcodeI {
    fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
        fmt::Result::Err(fmt::Error)
    }
}

pub enum PrivilegedOpcodeS {}

impl fmt::Display for PrivilegedOpcodeS {
    fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
        fmt::Result::Err(fmt::Error)
    }
}

pub enum PrivilegedOpcodeB {}

impl fmt::Display for PrivilegedOpcodeB {
    fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
        fmt::Result::Err(fmt::Error)
    }
}

pub enum PrivilegedOpcodeU {}

impl fmt::Display for PrivilegedOpcodeU {
    fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
        fmt::Result::Err(fmt::Error)
    }
}

pub enum PrivilegedOpcodeJ {}

impl fmt::Display for PrivilegedOpcodeJ {
    fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
        fmt::Result::Err(fmt::Error)
    }
}
