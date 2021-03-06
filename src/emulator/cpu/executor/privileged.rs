use crate::{
    emulator::{
        bus::SystemBus,
        cpu::{
            csr::ControlAndStatusRegister, executor::Executor, pc::ProgramCounter,
            x::IntegerRegister,
        },
    },
    isa::instruction::{
        privileged::{
            PrivilegedOpcodeB, PrivilegedOpcodeI, PrivilegedOpcodeJ, PrivilegedOpcodeR,
            PrivilegedOpcodeS, PrivilegedOpcodeU,
        },
        Instruction,
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
        _: &mut ProgramCounter,
        _: &mut IntegerRegister,
        _: &mut ControlAndStatusRegister,
        _: &mut SystemBus,
    ) {
        if let Instruction::TypeR {
            opcode,
            rs1: _,
            rs2: _,
            rd: _,
        } = instruction
        {
            match opcode {
                PrivilegedOpcodeR::Uret => {}      // not yet supported
                PrivilegedOpcodeR::Sret => {}      // not yet supported
                PrivilegedOpcodeR::Mret => {}      // not yet supported
                PrivilegedOpcodeR::Wfi => {}       // not yet supported
                PrivilegedOpcodeR::SfenceVma => {} // not yet supported
            }
        }
    }
}
