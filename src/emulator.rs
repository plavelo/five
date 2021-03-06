mod bus;
mod cpu;
mod memory;

use crate::emulator::{bus::Size, cpu::Cpu, memory::MEMORY_BASE_ADDRESS};
use std::fs::File;
use std::io::{BufReader, Read, Result};

#[allow(dead_code)]
#[derive(Default)]
pub struct Emulator {
    cpu: Cpu,
}

impl Emulator {
    pub fn load(&mut self, file: File) -> Result<()> {
        let buffer = BufReader::new(file);
        for (address, byte) in buffer.bytes().enumerate() {
            self.cpu.bus.memory.store(
                address as u64 + MEMORY_BASE_ADDRESS,
                byte? as u64,
                Size::Byte,
            );
        }
        Ok(())
    }

    pub fn run(&mut self) -> u64 {
        self.cpu.run()
    }
}
