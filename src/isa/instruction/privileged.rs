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
            PrivilegedOpcodeR::Uret => f.write_str("Privilege::Uret"),
            PrivilegedOpcodeR::Sret => f.write_str("Privilege::Sret"),
            PrivilegedOpcodeR::Mret => f.write_str("Privilege::Mret"),
            PrivilegedOpcodeR::Wfi => f.write_str("Privilege::Wfi"),
            PrivilegedOpcodeR::SfenceVma => f.write_str("Privilege::SfenceVma"),
        }
    }
}

pub enum PrivilegedOpcodeI {}
pub enum PrivilegedOpcodeS {}
pub enum PrivilegedOpcodeB {}
pub enum PrivilegedOpcodeU {}
pub enum PrivilegedOpcodeJ {}
