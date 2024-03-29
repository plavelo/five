use crate::{
    emulator::{
        bus::SystemBus,
        cpu::{
            csr::ControlAndStatusRegister, executor::Executor, f::FloatingPointRegister,
            pc::ProgramCounter, x::IntegerRegister,
        },
    },
    isa::{
        instruction::{
            privileged::{
                PrivilegedOpcodeB, PrivilegedOpcodeI, PrivilegedOpcodeJ, PrivilegedOpcodeR,
                PrivilegedOpcodeS, PrivilegedOpcodeU,
            },
            Instruction,
        },
        privileged::{
            cause::{Cause, Exception, ExceptionReturn},
            mode::PrivilegeMode,
        },
    },
};

pub struct PrivilegedExecutor;

impl Executor for PrivilegedExecutor {
    type OpcodeR = PrivilegedOpcodeR;
    type OpcodeI = PrivilegedOpcodeI;
    type OpcodeS = PrivilegedOpcodeS;
    type OpcodeB = PrivilegedOpcodeB;
    type OpcodeU = PrivilegedOpcodeU;
    type OpcodeJ = PrivilegedOpcodeJ;

    fn execute(
        instruction: Instruction<
            PrivilegedOpcodeR,
            PrivilegedOpcodeI,
            PrivilegedOpcodeS,
            PrivilegedOpcodeB,
            PrivilegedOpcodeU,
            PrivilegedOpcodeJ,
        >,
        prv: &PrivilegeMode,
        _: &mut ProgramCounter,
        _: &mut IntegerRegister,
        _: &mut FloatingPointRegister,
        _: &mut ControlAndStatusRegister,
        _: &mut SystemBus,
    ) -> Result<(), Cause> {
        if let Instruction::TypeR {
            opcode,
            rd: _,
            funct3: _,
            rs1: _,
            rs2: _,
            funct7: _,
        } = instruction
        {
            match opcode {
                PrivilegedOpcodeR::Uret => {
                    if prv == &PrivilegeMode::User {
                        Err(Cause::ExceptionReturn(ExceptionReturn::User))
                    } else {
                        Err(Cause::Exception(Exception::IllegalInstruction))
                    }
                }
                PrivilegedOpcodeR::Sret => {
                    if prv == &PrivilegeMode::Supervisor {
                        Err(Cause::ExceptionReturn(ExceptionReturn::Supervisor))
                    } else {
                        Err(Cause::Exception(Exception::IllegalInstruction))
                    }
                }
                PrivilegedOpcodeR::Mret => {
                    if prv == &PrivilegeMode::Machine {
                        Err(Cause::ExceptionReturn(ExceptionReturn::Machine))
                    } else {
                        Err(Cause::Exception(Exception::IllegalInstruction))
                    }
                }
                PrivilegedOpcodeR::Wfi => Ok(()), // not yet supported
                PrivilegedOpcodeR::SfenceVma => Ok(()), // not yet supported
            }
        } else {
            Ok(())
        }
    }
}
