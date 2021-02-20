mod bus;
mod cpu;
mod memory;

use crate::emulator::{bus::Xlen, cpu::Cpu, memory::MEMORY_BASE_ADDRESS};
use std::fs::File;
use std::io::{BufReader, Read, Result};

#[allow(dead_code)]
#[derive(Default)]
pub struct Emulator {
    cpu: Cpu,
}

#[allow(dead_code)]
impl Emulator {
    pub fn load(&mut self, file: File) -> Result<()> {
        let buffer = BufReader::new(file);
        for (address, byte) in buffer.bytes().enumerate() {
            self.cpu.bus.memory.store(
                address as u32 + MEMORY_BASE_ADDRESS,
                byte? as u32,
                Xlen::Byte,
            );
        }
        Ok(())
    }

    pub fn run(&mut self) -> u32 {
        self.cpu.run()
    }
}
