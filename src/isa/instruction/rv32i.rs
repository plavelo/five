use std::fmt;

#[derive(Debug, PartialEq)]
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
        match *self {
            Rv32iOpcodeR::Sll => f.write_str("Rv32i::Sll"),
            Rv32iOpcodeR::Srl => f.write_str("Rv32i::Srl"),
            Rv32iOpcodeR::Sra => f.write_str("Rv32i::Sra"),
            Rv32iOpcodeR::Add => f.write_str("Rv32i::Add"),
            Rv32iOpcodeR::Sub => f.write_str("Rv32i::Sub"),
            Rv32iOpcodeR::Xor => f.write_str("Rv32i::Xor"),
            Rv32iOpcodeR::Or => f.write_str("Rv32i::Or"),
            Rv32iOpcodeR::And => f.write_str("Rv32i::And"),
            Rv32iOpcodeR::Slt => f.write_str("Rv32i::Slt"),
            Rv32iOpcodeR::Sltu => f.write_str("Rv32i::Sltu"),
        }
    }
}

#[derive(Debug, PartialEq)]
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
        match *self {
            Rv32iOpcodeI::Slli => f.write_str("Rv32i::Slli"),
            Rv32iOpcodeI::Srli => f.write_str("Rv32i::Srli"),
            Rv32iOpcodeI::Srai => f.write_str("Rv32i::Srai"),
            Rv32iOpcodeI::Addi => f.write_str("Rv32i::Addi"),
            Rv32iOpcodeI::Xori => f.write_str("Rv32i::Xori"),
            Rv32iOpcodeI::Ori => f.write_str("Rv32i::Ori"),
            Rv32iOpcodeI::Andi => f.write_str("Rv32i::Andi"),
            Rv32iOpcodeI::Slti => f.write_str("Rv32i::Slti"),
            Rv32iOpcodeI::Sltiu => f.write_str("Rv32i::Sltiu"),
            Rv32iOpcodeI::Jalr => f.write_str("Rv32i::Jalr"),
            Rv32iOpcodeI::Fence => f.write_str("Rv32i::Fence"),
            Rv32iOpcodeI::Ecall => f.write_str("Rv32i::Ecall"),
            Rv32iOpcodeI::Ebreak => f.write_str("Rv32i::Ebreak"),
            Rv32iOpcodeI::Lb => f.write_str("Rv32i::Lb"),
            Rv32iOpcodeI::Lh => f.write_str("Rv32i::Lh"),
            Rv32iOpcodeI::Lbu => f.write_str("Rv32i::Lbu"),
            Rv32iOpcodeI::Lhu => f.write_str("Rv32i::Lhu"),
            Rv32iOpcodeI::Lw => f.write_str("Rv32i::Lw"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Rv32iOpcodeS {
    Sb,
    Sh,
    Sw,
}

impl fmt::Display for Rv32iOpcodeS {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Rv32iOpcodeS::Sb => f.write_str("Rv32i::Sb"),
            Rv32iOpcodeS::Sh => f.write_str("Rv32i::Sh"),
            Rv32iOpcodeS::Sw => f.write_str("Rv32i::Sw"),
        }
    }
}

#[derive(Debug, PartialEq)]
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
        match *self {
            Rv32iOpcodeB::Beq => f.write_str("Rv32i::Beq"),
            Rv32iOpcodeB::Bne => f.write_str("Rv32i::Bne"),
            Rv32iOpcodeB::Blt => f.write_str("Rv32i::Blt"),
            Rv32iOpcodeB::Bge => f.write_str("Rv32i::Bge"),
            Rv32iOpcodeB::Bltu => f.write_str("Rv32i::Bltu"),
            Rv32iOpcodeB::Bgeu => f.write_str("Rv32i::Bgeu"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Rv32iOpcodeU {
    Lui,
    Auipc,
}

impl fmt::Display for Rv32iOpcodeU {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Rv32iOpcodeU::Lui => f.write_str("Rv32i::Lui"),
            Rv32iOpcodeU::Auipc => f.write_str("Rv32i::Auipc"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Rv32iOpcodeJ {
    Jal,
}

impl fmt::Display for Rv32iOpcodeJ {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Rv32iOpcodeJ::Jal => f.write_str("Rv32i::Jal"),
        }
    }
}
