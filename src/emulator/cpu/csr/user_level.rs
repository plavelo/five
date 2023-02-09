use crate::{
    bitops::{MASK_3BIT, MASK_5BIT, MASK_8BIT},
    emulator::cpu::csr::Csr,
    isa::csr::user_level::*,
};
use std::collections::HashMap;

pub struct UserLevelCsr {
    csr: HashMap<u64, u64>,
}

impl Csr for UserLevelCsr {
    fn contains(&self, address: u64) -> bool {
        self.csr.contains_key(&address)
    }

    fn read(&self, address: u64) -> u64 {
        self.csr[&address]
    }

    fn write(&mut self, address: u64, value: u64) {
        match address {
            FFLAGS => {
                let fflags = value & MASK_5BIT;
                *self.csr.get_mut(&FFLAGS).unwrap() = fflags;
                *self.csr.get_mut(&FCSR).unwrap() = ((self.csr[&FRM] << 5) | fflags) & MASK_8BIT;
            }
            FRM => {
                let frm = value & MASK_3BIT;
                *self.csr.get_mut(&FRM).unwrap() = frm;
                *self.csr.get_mut(&FCSR).unwrap() = (self.csr[&FFLAGS] | (frm << 5)) & MASK_8BIT;
            }
            FCSR => {
                *self.csr.get_mut(&FFLAGS).unwrap() = value & MASK_5BIT;
                *self.csr.get_mut(&FRM).unwrap() = (value >> 5) & MASK_3BIT;
                *self.csr.get_mut(&FCSR).unwrap() = value & MASK_8BIT;
            }
            _ => {
                *self.csr.get_mut(&address).unwrap() = value;
            }
        }
    }

    fn csrrw(&mut self, address: u64, value: u64) -> u64 {
        let t = self.csr[&address];
        self.write(address, value);
        t
    }

    fn csrrs(&mut self, address: u64, value: u64) -> u64 {
        let t = self.csr[&address];
        self.write(address, self.csr[&address] | value);
        t
    }

    fn csrrc(&mut self, address: u64, value: u64) -> u64 {
        let t = self.csr[&address];
        self.write(address, self.csr[&address] & !value);
        t
    }
}

impl Default for UserLevelCsr {
    fn default() -> Self {
        Self {
            csr: [
                // User Trap Setup (URW)
                USTATUS,
                UIE,
                UTVEC,
                // User Trap Handling (URW)
                USCRATCH,
                UEPC,
                UCAUSE,
                UTVAL,
                UIP,
                // User Floating-Point CSRs (URW)
                FFLAGS,
                FRM,
                FCSR,
                // User Counter/Timers (URO)
                CYCLE,
                TIME,
                INSTRET,
                HPMCOUNTER3,
                HPMCOUNTER4,
                HPMCOUNTER5,
                HPMCOUNTER6,
                HPMCOUNTER7,
                HPMCOUNTER8,
                HPMCOUNTER9,
                HPMCOUNTER10,
                HPMCOUNTER11,
                HPMCOUNTER12,
                HPMCOUNTER13,
                HPMCOUNTER14,
                HPMCOUNTER15,
                HPMCOUNTER16,
                HPMCOUNTER17,
                HPMCOUNTER18,
                HPMCOUNTER19,
                HPMCOUNTER20,
                HPMCOUNTER21,
                HPMCOUNTER22,
                HPMCOUNTER23,
                HPMCOUNTER24,
                HPMCOUNTER25,
                HPMCOUNTER26,
                HPMCOUNTER27,
                HPMCOUNTER28,
                HPMCOUNTER29,
                HPMCOUNTER30,
                HPMCOUNTER31,
                CYCLEH,        // RV32I only
                TIMEH,         // RV32I only
                INSTRETH,      // RV32I only
                HPMCOUNTER3H,  // RV32I only
                HPMCOUNTER4H,  // RV32I only
                HPMCOUNTER5H,  // RV32I only
                HPMCOUNTER6H,  // RV32I only
                HPMCOUNTER7H,  // RV32I only
                HPMCOUNTER8H,  // RV32I only
                HPMCOUNTER9H,  // RV32I only
                HPMCOUNTER10H, // RV32I only
                HPMCOUNTER11H, // RV32I only
                HPMCOUNTER12H, // RV32I only
                HPMCOUNTER13H, // RV32I only
                HPMCOUNTER14H, // RV32I only
                HPMCOUNTER15H, // RV32I only
                HPMCOUNTER16H, // RV32I only
                HPMCOUNTER17H, // RV32I only
                HPMCOUNTER18H, // RV32I only
                HPMCOUNTER19H, // RV32I only
                HPMCOUNTER20H, // RV32I only
                HPMCOUNTER21H, // RV32I only
                HPMCOUNTER22H, // RV32I only
                HPMCOUNTER23H, // RV32I only
                HPMCOUNTER24H, // RV32I only
                HPMCOUNTER25H, // RV32I only
                HPMCOUNTER26H, // RV32I only
                HPMCOUNTER27H, // RV32I only
                HPMCOUNTER28H, // RV32I only
                HPMCOUNTER29H, // RV32I only
                HPMCOUNTER30H, // RV32I only
                HPMCOUNTER31H, // RV32I only
            ]
            .iter()
            .cloned()
            .map(|a| (a, 0))
            .collect::<HashMap<_, _>>(),
        }
    }
}
