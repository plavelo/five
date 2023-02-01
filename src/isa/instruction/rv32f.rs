use std::fmt;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Rv32fOpcodeR {
    FmaddS,
    FmsubS,
    FnmsubS,
    FnmaddS,
    FaddS,
    FsubS,
    FmulS,
    FdivS,
    FsqrtS,
    FsgnjS,
    FsgnjnS,
    FsgnjxS,
    FminS,
    FmaxS,
    FcvtWs,
    FcvtWuS,
    FmvXw,
    FeqS,
    FltS,
    FleS,
    FclassS,
    FcvtSw,
    FcvtSWu,
    FmvWx,
}

impl fmt::Display for Rv32fOpcodeR {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::FmaddS => f.write_str("fmadd.s"),
            Self::FmsubS => f.write_str("fmsub.s"),
            Self::FnmsubS => f.write_str("fnmsub.s"),
            Self::FnmaddS => f.write_str("fnmadd.s"),
            Self::FaddS => f.write_str("fadd.s"),
            Self::FsubS => f.write_str("fsub.s"),
            Self::FmulS => f.write_str("fmul.s"),
            Self::FdivS => f.write_str("fdiv.s"),
            Self::FsqrtS => f.write_str("fsqrt.s"),
            Self::FsgnjS => f.write_str("fsgnj.s"),
            Self::FsgnjnS => f.write_str("fsgnjn.s"),
            Self::FsgnjxS => f.write_str("fsgnjx.s"),
            Self::FminS => f.write_str("fmin.s"),
            Self::FmaxS => f.write_str("fmax.s"),
            Self::FcvtWs => f.write_str("fcvt.w.s"),
            Self::FcvtWuS => f.write_str("fcvt.wu.s"),
            Self::FmvXw => f.write_str("fmv.x.w"),
            Self::FeqS => f.write_str("feq.s"),
            Self::FltS => f.write_str("flt.s"),
            Self::FleS => f.write_str("fle.s"),
            Self::FclassS => f.write_str("fclass.s"),
            Self::FcvtSw => f.write_str("fcvt.s.w"),
            Self::FcvtSWu => f.write_str("fcvat.s.wu"),
            Self::FmvWx => f.write_str("fmv.w.x"),
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Rv32fOpcodeI {
    Flw,
}

impl fmt::Display for Rv32fOpcodeI {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Flw => f.write_str("flw"),
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Rv32fOpcodeS {
    Fsw,
}

impl fmt::Display for Rv32fOpcodeS {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Fsw => f.write_str("fsw"),
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Rv32fOpcodeB {}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Rv32fOpcodeU {}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Rv32fOpcodeJ {}
