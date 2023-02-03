use std::fmt;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Rv32iOpcodeR {
    Sll,
    Srl,
    Sra,
    Add,
    Sub,
    Xor,
    Or,
    And,
    Slt,
    Sltu,
}

impl fmt::Display for Rv32iOpcodeR {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Sll => f.write_str("sll"),
            Self::Srl => f.write_str("srl"),
            Self::Sra => f.write_str("sra"),
            Self::Add => f.write_str("add"),
            Self::Sub => f.write_str("sub"),
            Self::Xor => f.write_str("xor"),
            Self::Or => f.write_str("or"),
            Self::And => f.write_str("and"),
            Self::Slt => f.write_str("slt"),
            Self::Sltu => f.write_str("sltu"),
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Rv32iOpcodeI {
    Slli,
    Srli,
    Srai,
    Addi,
    Xori,
    Ori,
    Andi,
    Slti,
    Sltiu,
    Jalr,
    Fence,
    Ecall,
    Ebreak,
    Lb,
    Lh,
    Lbu,
    Lhu,
    Lw,
}

impl fmt::Display for Rv32iOpcodeI {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Slli => f.write_str("slli"),
            Self::Srli => f.write_str("srli"),
            Self::Srai => f.write_str("srai"),
            Self::Addi => f.write_str("addi"),
            Self::Xori => f.write_str("xori"),
            Self::Ori => f.write_str("ori"),
            Self::Andi => f.write_str("andi"),
            Self::Slti => f.write_str("slti"),
            Self::Sltiu => f.write_str("sltiu"),
            Self::Jalr => f.write_str("jalr"),
            Self::Fence => f.write_str("fence"),
            Self::Ecall => f.write_str("ecall"),
            Self::Ebreak => f.write_str("ebreak"),
            Self::Lb => f.write_str("lb"),
            Self::Lh => f.write_str("lh"),
            Self::Lbu => f.write_str("lbu"),
            Self::Lhu => f.write_str("lhu"),
            Self::Lw => f.write_str("lw"),
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Rv32iOpcodeS {
    Sb,
    Sh,
    Sw,
}

impl fmt::Display for Rv32iOpcodeS {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Sb => f.write_str("sb"),
            Self::Sh => f.write_str("sh"),
            Self::Sw => f.write_str("sw"),
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Rv32iOpcodeB {
    Beq,
    Bne,
    Blt,
    Bge,
    Bltu,
    Bgeu,
}

impl fmt::Display for Rv32iOpcodeB {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Beq => f.write_str("beq"),
            Self::Bne => f.write_str("bne"),
            Self::Blt => f.write_str("blt"),
            Self::Bge => f.write_str("bge"),
            Self::Bltu => f.write_str("bltu"),
            Self::Bgeu => f.write_str("bgeu"),
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Rv32iOpcodeU {
    Lui,
    Auipc,
}

impl fmt::Display for Rv32iOpcodeU {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Lui => f.write_str("lui"),
            Self::Auipc => f.write_str("auipc"),
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Rv32iOpcodeJ {
    Jal,
}

impl fmt::Display for Rv32iOpcodeJ {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Jal => f.write_str("jal"),
        }
    }
}
