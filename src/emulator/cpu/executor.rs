pub mod privileged;
pub mod rv32i;

use crate::{
    emulator::{
        bus::SystemBus,
        cpu::{csr::ControlAndStatusRegister, pc::ProgramCounter, x::IntegerRegister},
    },
    isa::instruction::Instruction,
};

pub trait Executor {
    type OpcodeR;
    type OpcodeI;
    type OpcodeS;
    type OpcodeB;
    type OpcodeU;
    type OpcodeJ;

    #[allow(clippy::type_complexity)]
    fn execute(
        instruction: Instruction<
            Self::OpcodeR,
            Self::OpcodeI,
            Self::OpcodeS,
            Self::OpcodeB,
            Self::OpcodeU,
            Self::OpcodeJ,
        >,
        pc: &mut ProgramCounter,
        x: &mut IntegerRegister,
        csr: &mut ControlAndStatusRegister,
        bus: &mut SystemBus,
    );
}
