use std::fmt;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Rv64fOpcodeR {
    FcvtLS,
    FcvtLuS,
    FcvtSL,
    FcvtSLu,
}

impl fmt::Display for Rv64fOpcodeR {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::FcvtLS => f.write_str("fcvt.l.s"),
            Self::FcvtLuS => f.write_str("fcvt.lu.s"),
            Self::FcvtSL => f.write_str("fcvt.s.l"),
            Self::FcvtSLu => f.write_str("fcvt.s.lu"),
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Rv64fOpcodeI {}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Rv64fOpcodeS {}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Rv64fOpcodeB {}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Rv64fOpcodeU {}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Rv64fOpcodeJ {}
