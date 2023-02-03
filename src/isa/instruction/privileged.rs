use std::fmt;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum PrivilegedOpcodeR {
    Uret,
    Sret,
    Mret,
    Wfi,
    SfenceVma,
}

impl fmt::Display for PrivilegedOpcodeR {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Uret => f.write_str("uret"),
            Self::Sret => f.write_str("sret"),
            Self::Mret => f.write_str("mret"),
            Self::Wfi => f.write_str("wfi"),
            Self::SfenceVma => f.write_str("sfence.vma"),
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum PrivilegedOpcodeI {}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum PrivilegedOpcodeS {}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum PrivilegedOpcodeB {}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum PrivilegedOpcodeU {}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum PrivilegedOpcodeJ {}
