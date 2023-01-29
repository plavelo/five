pub mod privileged;
pub mod rv32f;
pub mod rv32i;
pub mod rv32m;
pub mod rv64i;
pub mod rv64m;
pub mod zicsr;
pub mod zifencei;

use crate::{
    emulator::{
        bus::SystemBus,
        cpu::{
            csr::ControlAndStatusRegister, f::FloatingPointRegister, pc::ProgramCounter,
            x::IntegerRegister,
        },
    },
    isa::{
        instruction::Instruction,
        privileged::{cause::Cause, mode::PrivilegeMode},
    },
};
use std::mem::size_of_val;

const MASK_3BIT: u64 = 0b111;
const MASK_5BIT: u64 = 0b11111;
const MASK_6BIT: u64 = 0b111111;
const MASK_12BIT: u64 = 0b111111111111;

pub fn extend_sign(value: u64, nbits: u32) -> i64 {
    let target = value as i64;
    let notherbits = size_of_val(&target) as u32 * 8 - nbits;
    target.wrapping_shl(notherbits).wrapping_shr(notherbits)
}

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
        prv: &PrivilegeMode,
        pc: &mut ProgramCounter,
        x: &mut IntegerRegister,
        f: &mut FloatingPointRegister,
        csr: &mut ControlAndStatusRegister,
        bus: &mut SystemBus,
    ) -> Result<(), Cause>;
}
