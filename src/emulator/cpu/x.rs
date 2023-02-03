use crate::isa::register::{SP, ZERO};
use crate::{emulator::bus::memory::MEMORY_SIZE, isa::register::to_xname};
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
                to_xname(0),
                self.readu(0),
                to_xname(8),
                self.readu(8),
                to_xname(16),
                self.readu(16),
                to_xname(24),
                self.readu(24),
                to_xname(1),
                self.readu(1),
                to_xname(9),
                self.readu(9),
                to_xname(17),
                self.readu(17),
                to_xname(25),
                self.readu(25),
                to_xname(2),
                self.readu(2),
                to_xname(10),
                self.readu(10),
                to_xname(18),
                self.readu(18),
                to_xname(26),
                self.readu(26),
                to_xname(3),
                self.readu(3),
                to_xname(11),
                self.readu(11),
                to_xname(19),
                self.readu(19),
                to_xname(27),
                self.readu(27),
                to_xname(4),
                self.readu(4),
                to_xname(12),
                self.readu(12),
                to_xname(20),
                self.readu(20),
                to_xname(28),
                self.readu(28),
                to_xname(5),
                self.readu(5),
                to_xname(13),
                self.readu(13),
                to_xname(21),
                self.readu(21),
                to_xname(29),
                self.readu(29),
                to_xname(6),
                self.readu(6),
                to_xname(14),
                self.readu(14),
                to_xname(22),
                self.readu(22),
                to_xname(30),
                self.readu(30),
                to_xname(7),
                self.readu(7),
                to_xname(15),
                self.readu(15),
                to_xname(23),
                self.readu(23),
                to_xname(31),
                self.readu(31),
            )
            .as_str(),
        )
    }
}
