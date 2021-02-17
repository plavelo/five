pub struct ProgramCounter {
    pc: u32,
}

#[allow(dead_code)]
impl ProgramCounter {
    pub fn new() -> Self {
        Self { pc: 0 }
    }

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
        self.pc = 0;
    }
}
