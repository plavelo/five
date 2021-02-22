mod csr;
mod executor;
mod pc;
mod x;

use crate::{
    emulator::{
        bus::SystemBus,
        cpu::{
            csr::ControlAndStatusRegister,
            executor::{privileged::execute_privileged, rv32i::execute_rv32i},
            pc::ProgramCounter,
            x::{IntegerRegister, GP},
        },
    },
    isa::instruction::Instruction,
};

#[allow(dead_code)]
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

            let decoded = Instruction::decode(fetched);

            if let Some(instruction) = decoded {
                match instruction {
                    Instruction::Privileged(i) => execute_privileged(
                        i,
                        &mut self.pc,
                        &mut self.x,
                        &mut self.csr,
                        &mut self.bus,
                    ),
                    Instruction::Rv32i(i) => {
                        execute_rv32i(i, &mut self.pc, &mut self.x, &mut self.csr, &mut self.bus)
                    }
                }
            } else {
                break;
            }
            self.pc.increment();
        }
        self.x.readu(GP)
    }
}
