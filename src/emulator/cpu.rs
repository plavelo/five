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
            privileged::PrivilegedDecoder, rv32i::Rv32iDecoder, rv32m::Rv32mDecoder,
            rv64i::Rv64iDecoder, rv64m::Rv64mDecoder, Decoder,
        },
        executor::{
            privileged::PrivilegedExecutor, rv32i::Rv32iExecutor, rv32m::Rv32mExecutor,
            rv64i::Rv64iExecutor, rv64m::Rv64mExecutor, Executor,
        },
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
    pub fn run(&mut self) -> u64 {
        while self.pc.read() < self.bus.memory.size() {
            // read an address from the pc
            let address = self.pc.read();
            // fetch an instruction
            let instruction = self.bus.load32(address);
            // decode and execute the instruction
            if let Some(decoded) = PrivilegedDecoder::decode(instruction) {
                PrivilegedExecutor::execute(
                    decoded,
                    &mut self.pc,
                    &mut self.x,
                    &mut self.csr,
                    &mut self.bus,
                )
            } else if let Some(decoded) = Rv32iDecoder::decode(instruction) {
                Rv32iExecutor::execute(
                    decoded,
                    &mut self.pc,
                    &mut self.x,
                    &mut self.csr,
                    &mut self.bus,
                )
            } else if let Some(decoded) = Rv64iDecoder::decode(instruction) {
                Rv64iExecutor::execute(
                    decoded,
                    &mut self.pc,
                    &mut self.x,
                    &mut self.csr,
                    &mut self.bus,
                )
            } else if let Some(decoded) = Rv32mDecoder::decode(instruction) {
                Rv32mExecutor::execute(
                    decoded,
                    &mut self.pc,
                    &mut self.x,
                    &mut self.csr,
                    &mut self.bus,
                )
            } else if let Some(decoded) = Rv64mDecoder::decode(instruction) {
                Rv64mExecutor::execute(
                    decoded,
                    &mut self.pc,
                    &mut self.x,
                    &mut self.csr,
                    &mut self.bus,
                )
            } else {
                // end the loop when unable to decode the instruction
                break;
            };
            // increment the pc when the pc has not been updated
            if self.pc.read() == address {
                self.pc.increment();
            }
        }
        self.x.readu(GP)
    }
}
