use crate::isa::register::{SP, ZERO};
use crate::{emulator::bus::memory::MEMORY_SIZE, isa::register::register_name};
use std::fmt;

pub struct IntegerRegister {
    x: [u64; 32],
}

impl Default for IntegerRegister {
    fn default() -> Self {
        let mut x = [0; 32];
        x[SP] = MEMORY_SIZE;
        Self { x }
    }
}

impl IntegerRegister {
    pub fn readi(&self, register: usize) -> i64 {
        self.x[register] as i64
    }

    pub fn readu(&self, register: usize) -> u64 {
        self.x[register]
    }

    pub fn writei(&mut self, register: usize, value: i64) {
        if register != ZERO {
            self.x[register] = value as u64;
        }
    }

    pub fn writeu(&mut self, register: usize, value: u64) {
        if register != ZERO {
            self.x[register] = value;
        }
    }

    pub fn snapshot(&self) -> [u64; 32] {
        self.x
    }

    pub fn diff(&self, other: [u64; 32]) -> Vec<(usize, u64, u64)> {
        let mut result = vec![];
        for (i, x) in self.x.iter().enumerate() {
            let o = other[i];
            if *x != o {
                result.push((i, o, *x));
            }
        }
        result
    }
}

impl fmt::Display for IntegerRegister {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(
            format!(
                "{:4}:{:16x}, {:4}:{:16x}, {:4}:{:16x}, {:4}:{:16x}\n\
                {:4}:{:16x}, {:4}:{:16x}, {:4}:{:16x}, {:4}:{:16x}\n\
                {:4}:{:16x}, {:4}:{:16x}, {:4}:{:16x}, {:4}:{:16x}\n\
                {:4}:{:16x}, {:4}:{:16x}, {:4}:{:16x}, {:4}:{:16x}\n\
                {:4}:{:16x}, {:4}:{:16x}, {:4}:{:16x}, {:4}:{:16x}\n\
                {:4}:{:16x}, {:4}:{:16x}, {:4}:{:16x}, {:4}:{:16x}\n\
                {:4}:{:16x}, {:4}:{:16x}, {:4}:{:16x}, {:4}:{:16x}\n\
                {:4}:{:16x}, {:4}:{:16x}, {:4}:{:16x}, {:4}:{:16x}",
                register_name(0),
                self.readu(0),
                register_name(8),
                self.readu(8),
                register_name(16),
                self.readu(16),
                register_name(24),
                self.readu(24),
                register_name(1),
                self.readu(1),
                register_name(9),
                self.readu(9),
                register_name(17),
                self.readu(17),
                register_name(25),
                self.readu(25),
                register_name(2),
                self.readu(2),
                register_name(10),
                self.readu(10),
                register_name(18),
                self.readu(18),
                register_name(26),
                self.readu(26),
                register_name(3),
                self.readu(3),
                register_name(11),
                self.readu(11),
                register_name(19),
                self.readu(19),
                register_name(27),
                self.readu(27),
                register_name(4),
                self.readu(4),
                register_name(12),
                self.readu(12),
                register_name(20),
                self.readu(20),
                register_name(28),
                self.readu(28),
                register_name(5),
                self.readu(5),
                register_name(13),
                self.readu(13),
                register_name(21),
                self.readu(21),
                register_name(29),
                self.readu(29),
                register_name(6),
                self.readu(6),
                register_name(14),
                self.readu(14),
                register_name(22),
                self.readu(22),
                register_name(30),
                self.readu(30),
                register_name(7),
                self.readu(7),
                register_name(15),
                self.readu(15),
                register_name(23),
                self.readu(23),
                register_name(31),
                self.readu(31),
            )
            .as_str(),
        )
    }
}
