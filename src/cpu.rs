pub mod pc;
pub mod register;

use crate::cpu::register::Register;

#[allow(dead_code)]
pub struct Cpu {
    register: Register,
}
