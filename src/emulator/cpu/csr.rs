mod machine_level;
mod supervisor_level;
mod user_level;

use crate::emulator::cpu::csr::{
    machine_level::MachineLevelCsr, supervisor_level::SupervisorLevelCsr, user_level::UserLevelCsr,
};

#[derive(Default)]
pub struct ControlAndStatusRegister {
    ucsr: UserLevelCsr,
    scsr: SupervisorLevelCsr,
    mcsr: MachineLevelCsr,
}

impl Csr for ControlAndStatusRegister {
    fn contains(&self, address: u64) -> bool {
        self.ucsr.contains(address) || self.scsr.contains(address) || self.mcsr.contains(address)
    }

    fn read(&self, address: u64) -> u64 {
        if self.ucsr.contains(address) {
            return self.ucsr.read(address);
        }
        if self.scsr.contains(address) {
            return self.scsr.read(address);
        }
        if self.mcsr.contains(address) {
            return self.mcsr.read(address);
        }
        panic!("address not found. {:x}", address);
    }

    fn write(&mut self, address: u64, value: u64) {
        if self.ucsr.contains(address) {
            return self.ucsr.write(address, value);
        }
        if self.scsr.contains(address) {
            return self.scsr.write(address, value);
        }
        if self.mcsr.contains(address) {
            return self.mcsr.write(address, value);
        }
        panic!("address not found. {:x}", address);
    }

    fn csrrw(&mut self, address: u64, value: u64) -> u64 {
        if self.ucsr.contains(address) {
            return self.ucsr.csrrw(address, value);
        }
        if self.scsr.contains(address) {
            return self.scsr.csrrw(address, value);
        }
        if self.mcsr.contains(address) {
            return self.mcsr.csrrw(address, value);
        }
        panic!("address not found. {:x}", address);
    }

    fn csrrs(&mut self, address: u64, value: u64) -> u64 {
        if self.ucsr.contains(address) {
            return self.ucsr.csrrs(address, value);
        }
        if self.scsr.contains(address) {
            return self.scsr.csrrs(address, value);
        }
        if self.mcsr.contains(address) {
            return self.mcsr.csrrs(address, value);
        }
        panic!("address not found. {:x}", address);
    }

    fn csrrc(&mut self, address: u64, value: u64) -> u64 {
        if self.ucsr.contains(address) {
            return self.ucsr.csrrc(address, value);
        }
        if self.scsr.contains(address) {
            return self.scsr.csrrc(address, value);
        }
        if self.mcsr.contains(address) {
            return self.mcsr.csrrc(address, value);
        }
        panic!("address not found. {:x}", address);
    }
}

pub trait Csr {
    fn contains(&self, address: u64) -> bool;
    fn read(&self, address: u64) -> u64;
    fn write(&mut self, address: u64, value: u64);
    fn csrrw(&mut self, address: u64, value: u64) -> u64;
    fn csrrs(&mut self, address: u64, value: u64) -> u64;
    fn csrrc(&mut self, address: u64, value: u64) -> u64;
}
