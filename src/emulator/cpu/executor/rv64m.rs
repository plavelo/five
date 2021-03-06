use crate::{
    emulator::{
        bus::SystemBus,
        cpu::{
            csr::ControlAndStatusRegister, executor::Executor, pc::ProgramCounter,
            x::IntegerRegister,
        },
    },
    isa::instruction::{
        rv64m::{
            Rv64mOpcodeB, Rv64mOpcodeI, Rv64mOpcodeJ, Rv64mOpcodeR, Rv64mOpcodeS, Rv64mOpcodeU,
        },
        Instruction,
    },
};

pub struct Rv64mExecutor;

impl Executor for Rv64mExecutor {
    type OpcodeR = Rv64mOpcodeR;
    type OpcodeI = Rv64mOpcodeI;
    type OpcodeS = Rv64mOpcodeS;
    type OpcodeB = Rv64mOpcodeB;
    type OpcodeU = Rv64mOpcodeU;
    type OpcodeJ = Rv64mOpcodeJ;

    fn execute(
        instruction: Instruction<
            Rv64mOpcodeR,
            Rv64mOpcodeI,
            Rv64mOpcodeS,
            Rv64mOpcodeB,
            Rv64mOpcodeU,
            Rv64mOpcodeJ,
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
                Rv64mOpcodeR::Mulw => x.writei(
                    rd,
                    x.readu(rs1).wrapping_mul(x.readu(rs2)) as u32 as i32 as i64,
                ),
                Rv64mOpcodeR::Divw => x.writei(
                    rd,
                    (x.readi(rs1) as i32).wrapping_div(x.readi(rs2) as i32) as i64,
                ),
                Rv64mOpcodeR::Remw => x.writei(
                    rd,
                    (x.readi(rs1) as i32).wrapping_rem(x.readi(rs2) as i32) as i64,
                ),
                Rv64mOpcodeR::Remuw => x.writei(
                    rd,
                    (x.readu(rs1) as u32).wrapping_rem(x.readu(rs2) as u32) as i32 as i64,
                ),
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
