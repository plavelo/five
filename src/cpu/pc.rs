pub struct ProgramCounter {
    pc: u64,
}

#[allow(dead_code)]
impl ProgramCounter {
    pub fn new() -> Self {
        Self { pc: 0 }
    }

    pub fn read(&self) -> u64 {
        self.pc
    }

    pub fn increment(&mut self) {
        self.pc += 4;
    }

    pub fn jump(&mut self, relative_address: i16) {
        self.pc = (self.pc as i64 + relative_address as i64) as u64;
    }

    pub fn reset(&mut self) {
        self.pc = 0;
    }
}
