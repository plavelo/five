use crate::{
    emulator::{
        bus::SystemBus,
        cpu::{
            csr::ControlAndStatusRegister, executor::Executor, pc::ProgramCounter,
            x::IntegerRegister,
        },
    },
    isa::instruction::{
        rv32m::{
            Rv32mOpcodeB, Rv32mOpcodeI, Rv32mOpcodeJ, Rv32mOpcodeR, Rv32mOpcodeS, Rv32mOpcodeU,
        },
        Instruction,
    },
};

pub struct Rv32mExecutor;

impl Executor for Rv32mExecutor {
    type OpcodeR = Rv32mOpcodeR;
    type OpcodeI = Rv32mOpcodeI;
    type OpcodeS = Rv32mOpcodeS;
    type OpcodeB = Rv32mOpcodeB;
    type OpcodeU = Rv32mOpcodeU;
    type OpcodeJ = Rv32mOpcodeJ;

    fn execute(
        instruction: Instruction<
            Rv32mOpcodeR,
            Rv32mOpcodeI,
            Rv32mOpcodeS,
            Rv32mOpcodeB,
            Rv32mOpcodeU,
            Rv32mOpcodeJ,
        >,
        _: &mut ProgramCounter,
        x: &mut IntegerRegister,
        _: &mut ControlAndStatusRegister,
        _: &mut SystemBus,
    ) {
        match instruction {
            Instruction::TypeR {
                opcode,
                rs1,
                rs2,
                rd,
            } => match opcode {
                Rv32mOpcodeR::Mul => x.writeu(rd, x.readu(rs1).wrapping_mul(x.readu(rs2))),
                Rv32mOpcodeR::Mulh => x.writeu(
                    rd,
                    ((x.readi(rs1) as i128).wrapping_mul(x.readi(rs2) as i128) >> 64) as u64,
                ),
                Rv32mOpcodeR::Mulhsu => x.writeu(
                    rd,
                    ((x.readi(rs1) as i128 as u128).wrapping_mul(x.readi(rs2) as u128) >> 64)
                        as u64,
                ),
                Rv32mOpcodeR::Mulhu => x.writeu(
                    rd,
                    ((x.readi(rs1) as u128).wrapping_mul(x.readi(rs2) as u128) >> 64) as u64,
                ),
                Rv32mOpcodeR::Div => x.writei(rd, x.readi(rs1).wrapping_div(x.readi(rs2))),
                Rv32mOpcodeR::Divu => x.writeu(rd, x.readu(rs1).wrapping_div(x.readu(rs2))),
                Rv32mOpcodeR::Rem => x.writei(rd, x.readi(rs1).wrapping_rem(x.readi(rs2))),
                Rv32mOpcodeR::Remu => x.writeu(rd, x.readu(rs1).wrapping_rem(x.readu(rs2))),
            },
            Instruction::TypeI {
                opcode: _,
                rs1: _,
                rd: _,
                imm: _,
            } => {}
            Instruction::TypeS {
                opcode: _,
                rs1: _,
                rs2: _,
                imm: _,
            } => {}
            Instruction::TypeB {
                opcode: _,
                rs1: _,
                rs2: _,
                imm: _,
            } => {}
            Instruction::TypeU {
                opcode: _,
                rd: _,
                imm: _,
            } => {}
            Instruction::TypeJ {
                opcode: _,
                rd: _,
                imm: _,
            } => {}
        }
    }
}
