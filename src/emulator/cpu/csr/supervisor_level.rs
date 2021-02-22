use crate::emulator::cpu::csr::Csr;
use std::collections::HashMap;

pub struct SupervisorLevelCsr {
    csr: HashMap<u32, u32>,
}

impl Csr for SupervisorLevelCsr {
    fn contains(&self, address: u32) -> bool {
        self.csr.contains_key(&address)
    }

    fn csrrw(&mut self, address: u32, value: u32) -> u32 {
        let t = self.csr[&address];
        *self.csr.get_mut(&address).unwrap() = value;
        t
    }

    fn csrrs(&mut self, address: u32, value: u32) -> u32 {
        let t = self.csr[&address];
        *self.csr.get_mut(&address).unwrap() = self.csr[&address] | value;
        t
    }

    fn csrrc(&mut self, address: u32, value: u32) -> u32 {
        let t = self.csr[&address];
        *self.csr.get_mut(&address).unwrap() = self.csr[&address] & !value;
        t
    }
}

impl Default for SupervisorLevelCsr {
    fn default() -> Self {
        Self {
            csr: [
                // Supervisor Trap Setup (SRW)
                SSTATUS, SEDELEG, SIDELEG, SIE, STVEC, SCOUNTEREN,
                // Supervisor Trap Handling (SRW)
                SSCRATCH, SEPC, SCAUSE, STVAL, SIP,
                // Supervisor Protection and Translation (SRW)
                SATP,
            ]
            .iter()
            .cloned()
            .map(|a| (a, 0))
            .collect::<HashMap<_, _>>(),
        }
    }
}

// Supervisor Trap Setup (SRW)
pub const SSTATUS: u32 = 0x100; // Supervisor status register.
pub const SEDELEG: u32 = 0x102; // Supervisor exception delegation register.
pub const SIDELEG: u32 = 0x103; // Supervisor interrupt delegation register.
pub const SIE: u32 = 0x104; // Supervisor interrupt-enable register.
pub const STVEC: u32 = 0x105; // Supervisor trap handler base address.
pub const SCOUNTEREN: u32 = 0x106; // Supervisor counter enable.

// Supervisor Trap Handling (SRW)
pub const SSCRATCH: u32 = 0x140; // Scratch register for supervisor trap handlers.
pub const SEPC: u32 = 0x141; // Supervisor exception program counter.
pub const SCAUSE: u32 = 0x142; // Supervisor trap cause.
pub const STVAL: u32 = 0x143; // Supervisor bad address or instruction.
pub const SIP: u32 = 0x144; // Supervisor interrupt pending.

// Supervisor Protection and Translation (SRW)
pub const SATP: u32 = 0x180; // Supervisor address translation and protection.
