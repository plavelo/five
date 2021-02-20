use crate::emulator::memory::MEMORY_BASE_ADDRESS;

pub struct ProgramCounter {
    pc: u32,
}

impl Default for ProgramCounter {
    fn default() -> Self {
        Self {
            pc: MEMORY_BASE_ADDRESS,
        }
    }
}

#[allow(dead_code)]
impl ProgramCounter {
    pub fn read(&self) -> u32 {
        self.pc
    }

    pub fn increment(&mut self) {
        self.pc += 4;
    }

    pub fn jump(&mut self, address: u32) {
        self.pc = address;
    }

    pub fn jumpr(&mut self, relative_address: i32) {
        self.pc = (self.pc as i32 + relative_address) as u32;
    }

    pub fn reset(&mut self) {
        self.pc = MEMORY_BASE_ADDRESS;
    }
}
