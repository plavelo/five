pub mod privileged;
pub mod rv32i;
pub mod rv64i;

use crate::{
    emulator::{
        bus::SystemBus,
        cpu::{csr::ControlAndStatusRegister, pc::ProgramCounter, x::IntegerRegister},
    },
    isa::instruction::Instruction,
};

const MASK_5BIT: u64 = 0b11111;
const MASK_12BIT: u64 = 0b111111111111;

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
