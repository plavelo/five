use crate::{
    emulator::{
        bus::SystemBus,
        cpu::{csr::ControlAndStatusRegister, pc::ProgramCounter, x::IntegerRegister},
    },
    isa::instruction::privileged::{PrivilegedInstruction, PrivilegedOpcodeR},
};

pub fn execute_privileged(
    instruction: PrivilegedInstruction,
    _: &mut ProgramCounter,
    _: &mut IntegerRegister,
    _: &mut ControlAndStatusRegister,
    _: &mut SystemBus,
) {
    match instruction {
        PrivilegedInstruction::TypeR {
            opcode,
            rs1: _,
            rs2: _,
            rd: _,
        } => match opcode {
            PrivilegedOpcodeR::Uret => {}      // not yet supported
            PrivilegedOpcodeR::Sret => {}      // not yet supported
            PrivilegedOpcodeR::Mret => {}      // not yet supported
            PrivilegedOpcodeR::Wfi => {}       // not yet supported
            PrivilegedOpcodeR::SfenceVma => {} // not yet supported
        },
    }
}
