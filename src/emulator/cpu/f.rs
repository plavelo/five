use crate::isa::register::to_fname;
use std::fmt;

#[derive(Default)]
pub struct FloatingPointRegister {
    f: [u64; 32],
}

impl FloatingPointRegister {
    pub fn reads(&self, register: usize) -> u32 {
        self.f[register] as u32
    }

    pub fn readd(&self, register: usize) -> u64 {
        self.f[register]
    }

    pub fn writes(&mut self, register: usize, value: u32) {
        self.f[register] = value as u64;
    }

    pub fn snapshot(&self) -> [u64; 32] {
        self.f
    }

    pub fn diff(&self, other: [u64; 32]) -> Vec<(usize, u64, u64)> {
        let mut result = vec![];
        for (i, x) in self.f.iter().enumerate() {
            let o = other[i];
            if *x != o {
                result.push((i, o, *x));
            }
        }
        result
    }
}

impl fmt::Display for FloatingPointRegister {
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
                to_fname(0),
                self.readd(0),
                to_fname(8),
                self.readd(8),
                to_fname(16),
                self.readd(16),
                to_fname(24),
                self.readd(24),
                to_fname(1),
                self.readd(1),
                to_fname(9),
                self.readd(9),
                to_fname(17),
                self.readd(17),
                to_fname(25),
                self.readd(25),
                to_fname(2),
                self.readd(2),
                to_fname(10),
                self.readd(10),
                to_fname(18),
                self.readd(18),
                to_fname(26),
                self.readd(26),
                to_fname(3),
                self.readd(3),
                to_fname(11),
                self.readd(11),
                to_fname(19),
                self.readd(19),
                to_fname(27),
                self.readd(27),
                to_fname(4),
                self.readd(4),
                to_fname(12),
                self.readd(12),
                to_fname(20),
                self.readd(20),
                to_fname(28),
                self.readd(28),
                to_fname(5),
                self.readd(5),
                to_fname(13),
                self.readd(13),
                to_fname(21),
                self.readd(21),
                to_fname(29),
                self.readd(29),
                to_fname(6),
                self.readd(6),
                to_fname(14),
                self.readd(14),
                to_fname(22),
                self.readd(22),
                to_fname(30),
                self.readd(30),
                to_fname(7),
                self.readd(7),
                to_fname(15),
                self.readd(15),
                to_fname(23),
                self.readd(23),
                to_fname(31),
                self.readd(31),
            )
            .as_str(),
        )
    }
}
