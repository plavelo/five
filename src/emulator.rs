use crate::cpu::Cpu;
use crate::memory::DataMemory;

#[allow(dead_code)]
pub struct Emulator {
    cpu: Cpu,
    memory: DataMemory,
}

#[allow(dead_code)]
impl Emulator {
    fn initialize(&mut self, cpu: Cpu, memory: DataMemory) {
        self.cpu = cpu;
        self.memory = memory;
    }
}
