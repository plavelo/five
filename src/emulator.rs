use crate::cpu::Cpu;

#[allow(dead_code)]
pub struct Emulator {
    cpu: Cpu,
}

#[allow(dead_code)]
impl Emulator {
    fn new() -> Self {
        Self { cpu: Cpu::new() }
    }

    fn cycle(&mut self) {
        self.cpu.cycle()
    }
}
