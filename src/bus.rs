use crate::memory::Memory;

#[allow(dead_code)]
pub const HALFWORD: u8 = 16;
#[allow(dead_code)]
pub const WORD: u8 = 32;
#[allow(dead_code)]
pub const DOUBLEWORD: u8 = 64;

pub struct SystemBus {
    pub memory: Memory,
}

impl SystemBus {
    pub fn new() -> Self {
        Self {
            memory: Memory::new(),
        }
    }
}
