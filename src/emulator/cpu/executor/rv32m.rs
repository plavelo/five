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
                Rv32mOpcodeR::Div => {
                    let dividend = x.readi(rs1);
                    let divisor = x.readi(rs2);
                    x.writei(
                        rd,
                        if divisor == 0 {
                            i64::MAX
                        } else {
                            dividend.wrapping_div(divisor)
                        },
                    )
                }
                Rv32mOpcodeR::Divu => {
                    let dividend = x.readu(rs1);
                    let divisor = x.readu(rs2);
                    x.writeu(
                        rd,
                        if divisor == 0 {
                            u64::MAX
                        } else {
                            dividend.wrapping_div(divisor)
                        },
                    )
                }
                Rv32mOpcodeR::Rem => {
                    let dividend = x.readi(rs1);
                    let divisor = x.readi(rs2);
                    x.writei(
                        rd,
                        if divisor == 0 {
                            dividend
                        } else {
                            dividend.wrapping_rem(divisor)
                        },
                    )
                }
                Rv32mOpcodeR::Remu => {
                    let dividend = x.readu(rs1);
                    let divisor = x.readu(rs2);
                    x.writeu(
                        rd,
                        if divisor == 0 {
                            dividend
                        } else {
                            dividend.wrapping_rem(divisor)
                        },
                    )
                }
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
