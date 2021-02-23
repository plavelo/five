mod csr;
mod decoder;
mod executor;
mod pc;
mod x;

use crate::emulator::{
    bus::SystemBus,
    cpu::{
        csr::ControlAndStatusRegister,
        decoder::{
            privileged::PrivilegedInstructionDecoder, rv32i::Rv32iInstructionDecoder,
            InstructionDecoder,
        },
        executor::{privileged::execute_privileged, rv32i::execute_rv32i},
        pc::ProgramCounter,
        x::{IntegerRegister, GP},
    },
};

#[derive(Default)]
pub struct Cpu {
    x: IntegerRegister,
    pc: ProgramCounter,
    csr: ControlAndStatusRegister,
    pub bus: SystemBus,
}

impl Cpu {
    pub fn run(&mut self) -> u32 {
        while self.pc.read() < self.bus.memory.size() {
            let address = self.pc.read();
            let fetched = self.bus.load32(address);

            if let Some(i) = PrivilegedInstructionDecoder::decode(fetched) {
                execute_privileged(i, &mut self.pc, &mut self.x, &mut self.csr, &mut self.bus)
            } else if let Some(i) = Rv32iInstructionDecoder::decode(fetched) {
                execute_rv32i(i, &mut self.pc, &mut self.x, &mut self.csr, &mut self.bus)
            } else {
                break;
            }
            self.pc.increment();
        }
        self.x.readu(GP)
    }
}
