use crate::{
    bitops::{extend_sign, shift_amount, MASK_4BIT},
    isa::{
        description::{
            xformat3, xformat_immediate, xformat_offset, xformat_upper_immediate, Describer,
            Description,
        },
        instruction::{
            rv32i::{
                Rv32iOpcodeB, Rv32iOpcodeI, Rv32iOpcodeJ, Rv32iOpcodeR, Rv32iOpcodeS, Rv32iOpcodeU,
            },
            Instruction,
        },
    },
};

impl Describer
    for Instruction<
        Rv32iOpcodeR,
        Rv32iOpcodeI,
        Rv32iOpcodeS,
        Rv32iOpcodeB,
        Rv32iOpcodeU,
        Rv32iOpcodeJ,
    >
{
    type OpcodeR = Rv32iOpcodeR;
    type OpcodeI = Rv32iOpcodeI;
    type OpcodeS = Rv32iOpcodeS;
    type OpcodeB = Rv32iOpcodeB;
    type OpcodeU = Rv32iOpcodeU;
    type OpcodeJ = Rv32iOpcodeJ;

    fn describe(&self) -> Description {
        let (description, assembly, signature, pseudocode) = match *self {
            Self::TypeR {
                opcode,
                rd,
                funct3: _,
                rs1,
                rs2,
                funct7: _,
            } => match opcode {
                Rv32iOpcodeR::Sll => (
                    "Shift Left Logical",
                    xformat3(opcode.to_string(), rd, rs1, rs2),
                    "sll rd,rs1,rs2",
                    "x[rd] = x[rs1] << x[rs2]",
                ),
                Rv32iOpcodeR::Srl => (
                    "Shift Right Logical",
                    xformat3(opcode.to_string(), rd, rs1, rs2),
                    "srl rd,rs1,rs2",
                    "x[rd] = x[rs1] >>u x[rs2]",
                ),
                Rv32iOpcodeR::Sra => (
                    "Shift Right Arithmetic",
                    xformat3(opcode.to_string(), rd, rs1, rs2),
                    "sra rd,rs1,rs2",
                    "x[rd] = x[rs1] >>s x[rs2]",
                ),
                Rv32iOpcodeR::Add => (
                    "Add",
                    xformat3(opcode.to_string(), rd, rs1, rs2),
                    "add rd,rs1,rs2",
                    "x[rd] = x[rs1] + x[rs2]",
                ),
                Rv32iOpcodeR::Sub => (
                    "Subtract",
                    xformat3(opcode.to_string(), rd, rs1, rs2),
                    "sub rd,rs1,rs2",
                    "x[rd] = x[rs1] - x[rs2]",
                ),
                Rv32iOpcodeR::Xor => (
                    "Exclusive-OR",
                    xformat3(opcode.to_string(), rd, rs1, rs2),
                    "xor rd,rs1,rs2",
                    "x[rd] = x[rs1] ^ x[rs2]",
                ),
                Rv32iOpcodeR::Or => (
                    "OR",
                    xformat3(opcode.to_string(), rd, rs1, rs2),
                    "or rd,rs1,rs2",
                    "x[rd] = x[rs1] | x[rs2]",
                ),
                Rv32iOpcodeR::And => (
                    "AND",
                    xformat3(opcode.to_string(), rd, rs1, rs2),
                    "and rd,rs1,rs2",
                    "x[rd] = x[rs1] & x[rs2]",
                ),
                Rv32iOpcodeR::Slt => (
                    "Set if Less Than",
                    xformat3(opcode.to_string(), rd, rs1, rs2),
                    "slt rd,rs1,rs2",
                    "x[rd] = x[rs1] <s x[rs2]",
                ),
                Rv32iOpcodeR::Sltu => (
                    "Set if Less Than, Unsigned",
                    xformat3(opcode.to_string(), rd, rs1, rs2),
                    "sltu rd,rs1,rs2",
                    "x[rd] = x[rs1] <u x[rs2]",
                ),
            },
            Self::TypeI {
                opcode,
                rd,
                funct3: _,
                rs1,
                imm,
            } => match opcode {
                Rv32iOpcodeI::Slli => (
                    "Shift Left Logical Immediate",
                    xformat_immediate(opcode.to_string(), rd, rs1, shift_amount(imm) as i64),
                    "slli rd,rs1,shamt",
                    "x[rd] = x[rs1] << shamt",
                ),
                Rv32iOpcodeI::Srli => (
                    "Shift Right Logical Immediate",
                    xformat_immediate(opcode.to_string(), rd, rs1, shift_amount(imm) as i64),
                    "srli rd,rs1,shamt",
                    "x[rd] = x[rs1] >>u shamt",
                ),
                Rv32iOpcodeI::Srai => (
                    "Shift Right Arithmetic Immediate",
                    xformat_immediate(opcode.to_string(), rd, rs1, shift_amount(imm) as i64),
                    "srai rd,rs1,shamt",
                    "x[rd] = x[rs1] >>s shamt",
                ),
                Rv32iOpcodeI::Addi => (
                    "Add Immediate",
                    xformat_immediate(opcode.to_string(), rd, rs1, extend_sign(imm, 12)),
                    "addi rd,rs1,imm",
                    "x[rd] = x[rs1] + sext(imm)",
                ),
                Rv32iOpcodeI::Xori => (
                    "Exclusive-OR Immediate",
                    xformat_immediate(opcode.to_string(), rd, rs1, imm as i64),
                    "xori rd,rs1,imm",
                    "x[rd] = x[rs1] ^ sext(imm)",
                ),
                Rv32iOpcodeI::Ori => (
                    "OR Immediate",
                    xformat_immediate(opcode.to_string(), rd, rs1, imm as i64),
                    "ori rd,rs1,imm",
                    "x[rd] = x[rs1] | sext(imm)",
                ),
                Rv32iOpcodeI::Andi => (
                    "AND Immediate",
                    xformat_immediate(opcode.to_string(), rd, rs1, imm as i64),
                    "andi rd,rs1,imm",
                    "x[rd] = x[rs1] & sext(imm)",
                ),
                Rv32iOpcodeI::Slti => (
                    "Set if Less Than Immediate",
                    xformat_immediate(opcode.to_string(), rd, rs1, imm as i64),
                    "slti rd,rs1,imm",
                    "x[rd] = x[rs1] <s sext(imm)",
                ),
                Rv32iOpcodeI::Sltiu => (
                    "Set if Less Than Immediate, Unsigned",
                    xformat_immediate(opcode.to_string(), rd, rs1, imm as i64),
                    "sltiu rd,rs1,imm",
                    "x[rd] = x[rs1] <u sext(imm)",
                ),
                Rv32iOpcodeI::Jalr => (
                    "Jump and Link Register",
                    xformat_offset(opcode.to_string(), rd, extend_sign(imm, 12), rs1),
                    "jalr rd,offset(rs1)",
                    "t=pc+4; pc=(x[rs1]+sext(offset))&~1; x[rd]=t",
                ),

                Rv32iOpcodeI::Fence => (
                    "Fence Memory and I/O",
                    format!(
                        "{} {:b},{:b}",
                        opcode,
                        (imm >> 4) & MASK_4BIT,
                        imm & MASK_4BIT
                    ),
                    "fence pred,succ",
                    "fence(pred, succ)",
                ),
                Rv32iOpcodeI::Ecall => (
                    "Environment Call",
                    xformat_immediate(opcode.to_string(), rd, rs1, imm as i64),
                    "ecall",
                    "raise EnvirnmentCall",
                ),
                Rv32iOpcodeI::Ebreak => (
                    "Environment Breakpoint",
                    xformat_immediate(opcode.to_string(), rd, rs1, imm as i64),
                    "ebreak",
                    "raise Breakpoint",
                ),
                Rv32iOpcodeI::Lb => (
                    "Load Byte",
                    xformat_offset(opcode.to_string(), rd, extend_sign(imm, 12), rs1),
                    "lb rd,offset(rs1)",
                    "x[rd] = sext(mem[x[rs1] + sext(offset)][7:0])",
                ),

                Rv32iOpcodeI::Lh => (
                    "Load Halfword",
                    xformat_offset(opcode.to_string(), rd, extend_sign(imm, 12), rs1),
                    "lh rd,offset(rs1)",
                    "x[rd] = sext(mem[x[rs1] + sext(offset)][15:0])",
                ),
                Rv32iOpcodeI::Lbu => (
                    "Load Byte, Unsigned",
                    xformat_offset(opcode.to_string(), rd, extend_sign(imm, 12), rs1),
                    "lbu rd,offset(rs1)",
                    "x[rd] = mem[x[rs1] + sext(offset)][7:0]",
                ),
                Rv32iOpcodeI::Lhu => (
                    "Load Halfword, Unsigned",
                    xformat_offset(opcode.to_string(), rd, extend_sign(imm, 12), rs1),
                    "lhu rd,offset(rs1)",
                    "x[rd] = mem[x[rs1] + sext(offset)][15:0]",
                ),
                Rv32iOpcodeI::Lw => (
                    "Load Word",
                    xformat_offset(opcode.to_string(), rd, extend_sign(imm, 12), rs1),
                    "lw rd,offset(rs1)",
                    "x[rd] = sext(mem[x[rs1] + sext(offset)][31:0])",
                ),
            },
            Self::TypeS {
                opcode,
                funct3: _,
                rs1,
                rs2,
                imm,
            } => match opcode {
                Rv32iOpcodeS::Sb => (
                    "Store Byte",
                    xformat_offset(opcode.to_string(), rs2, extend_sign(imm, 12), rs1),
                    "sb rs2,offset(rs1)",
                    "x[rd] = mem[x[rs1] + sext(offset)] = x[rs2][7:0]",
                ),
                Rv32iOpcodeS::Sh => (
                    "Store Halfword",
                    xformat_offset(opcode.to_string(), rs2, extend_sign(imm, 12), rs1),
                    "sh rs2,offset(rs1)",
                    "x[rd] = mem[x[rs1] + sext(offset)] = x[rs2][15:0]",
                ),
                Rv32iOpcodeS::Sw => (
                    "Store Word",
                    xformat_offset(opcode.to_string(), rs2, extend_sign(imm, 12), rs1),
                    "sw rs2,offset(rs1)",
                    "x[rd] = mem[x[rs1] + sext(offset)] = x[rs2][31:0]",
                ),
            },
            Self::TypeB {
                opcode,
                funct3: _,
                rs1,
                rs2,
                imm,
            } => match opcode {
                Rv32iOpcodeB::Beq => (
                    "Branch if Equal",
                    xformat_immediate(opcode.to_string(), rs1, rs2, imm as i64),
                    "beq rs1,rs2,offset",
                    "if (rs1 == rs2) pc += sext(offset)",
                ),
                Rv32iOpcodeB::Bne => (
                    "Branch if Not Equal",
                    xformat_immediate(opcode.to_string(), rs1, rs2, imm as i64),
                    "bne rs1,rs2,offset",
                    "if (rs1 != rs2) pc += sext(offset)",
                ),
                Rv32iOpcodeB::Blt => (
                    "Branch if Less Than",
                    xformat_immediate(opcode.to_string(), rs1, rs2, imm as i64),
                    "blt rs1,rs2,offset",
                    "if (rs1 >=s rs2) pc += sext(offset)",
                ),
                Rv32iOpcodeB::Bge => (
                    "Branch if Greater Than or Equal",
                    xformat_immediate(opcode.to_string(), rs1, rs2, imm as i64),
                    "bge rs1,rs2,offset",
                    "if (rs1 >=s rs2) pc += sext(offset)",
                ),
                Rv32iOpcodeB::Bltu => (
                    "Branch if Less Than, Unsigned",
                    xformat_immediate(opcode.to_string(), rs1, rs2, imm as i64),
                    "bltu rs1,rs2,offset",
                    "if (rs1 <u rs2) pc += sext(offset)",
                ),
                Rv32iOpcodeB::Bgeu => (
                    "Branch if Greater Than or Equal, Unsigned",
                    xformat_immediate(opcode.to_string(), rs1, rs2, imm as i64),
                    "bgeu rs1,rs2,offset",
                    "if (rs1 >=u rs2) pc += sext(offset)",
                ),
            },
            Self::TypeU { opcode, rd, imm } => match opcode {
                Rv32iOpcodeU::Lui => (
                    "Load Upper Immediate",
                    xformat_upper_immediate(opcode.to_string(), rd, imm as i64),
                    "lui rd,imm",
                    "x[rd] = sext(imm[31:12] << 12)",
                ),
                Rv32iOpcodeU::Auipc => (
                    "Add Upper Immediate to PC",
                    xformat_upper_immediate(opcode.to_string(), rd, imm as i64),
                    "auipc rd,imm",
                    "x[rd] = pc + sext(imm[31:12] << 12)",
                ),
            },
            Self::TypeJ { opcode, rd, imm } => match opcode {
                Rv32iOpcodeJ::Jal => (
                    "Jump and Link",
                    xformat_upper_immediate(opcode.to_string(), rd, imm as i64),
                    "jal rd,offset",
                    "x[rd] = pc+4; pc += sext(offset)",
                ),
            },
        };
        Description {
            description: description.to_string(),
            assembly,
            singnature: signature.to_string(),
            pseudocode: pseudocode.to_string(),
        }
    }
}
