pub const MEMORY_SIZE: u64 = 1024 * 1024 * 1024;

pub struct DataMemory {
    memory: Vec<u8>,
}

#[allow(dead_code)]
impl DataMemory {
    pub fn new() -> Self {
        Self {
            memory: vec![0; MEMORY_SIZE as usize],
        }
    }

    pub fn fetch(&self, address: usize) -> u32 {
        self.memory[address] as u32
            | (self.memory[address + 1] as u32) << 8
            | (self.memory[address + 2] as u32) << 16
            | (self.memory[address + 3] as u32) << 24
    }
}
