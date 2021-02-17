pub const MEMORY_SIZE: u32 = 1024 * 1024 * 1024;

pub struct Memory {
    memory: Vec<u8>,
}

impl Memory {
    pub fn new() -> Self {
        Self {
            memory: vec![0; MEMORY_SIZE as usize],
        }
    }

    pub fn size(&self) -> u32 {
        MEMORY_SIZE
    }

    pub fn fetch(&self, address: u32) -> u32 {
        self.load32(address)
    }

    pub fn load8(&self, address: u32) -> u8 {
        self.memory[address as usize]
    }

    pub fn load16(&self, address: u32) -> u16 {
        self.memory[address as usize] as u16 | (self.memory[address as usize + 1] as u16) << 8
    }

    pub fn load32(&self, address: u32) -> u32 {
        self.memory[address as usize] as u32
            | (self.memory[address as usize + 1] as u32) << 8
            | (self.memory[address as usize + 2] as u32) << 16
            | (self.memory[address as usize + 3] as u32) << 24
    }

    pub fn store8(&mut self, address: u32, value: u8) {
        self.memory[address as usize] = value;
    }

    pub fn store16(&mut self, address: u32, value: u16) {
        self.memory[address as usize] = value as u8;
        self.memory[address as usize + 1] = (value >> 8) as u8;
    }

    pub fn store32(&mut self, address: u32, value: u32) {
        self.memory[address as usize] = value as u8;
        self.memory[address as usize + 1] = (value >> 8) as u8;
        self.memory[address as usize + 2] = (value >> 16) as u8;
        self.memory[address as usize + 3] = (value >> 24) as u8;
    }
}
