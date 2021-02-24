use crate::emulator::cpu::csr::Csr;
use std::collections::HashMap;

pub struct UserLevelCsr {
    csr: HashMap<u64, u64>,
}

impl Csr for UserLevelCsr {
    fn contains(&self, address: u64) -> bool {
        self.csr.contains_key(&address)
    }

    fn csrrw(&mut self, address: u64, value: u64) -> u64 {
        let t = self.csr[&address];
        *self.csr.get_mut(&address).unwrap() = value;
        t
    }

    fn csrrs(&mut self, address: u64, value: u64) -> u64 {
        let t = self.csr[&address];
        *self.csr.get_mut(&address).unwrap() = self.csr[&address] | value;
        t
    }

    fn csrrc(&mut self, address: u64, value: u64) -> u64 {
        let t = self.csr[&address];
        *self.csr.get_mut(&address).unwrap() = self.csr[&address] & !value;
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
                CYCLEH,
                TIMEH,
                INSTRETH,
                HPMCOUNTER3H,
                HPMCOUNTER4H,
                HPMCOUNTER5H,
                HPMCOUNTER6H,
                HPMCOUNTER7H,
                HPMCOUNTER8H,
                HPMCOUNTER9H,
                HPMCOUNTER10H,
                HPMCOUNTER11H,
                HPMCOUNTER12H,
                HPMCOUNTER13H,
                HPMCOUNTER14H,
                HPMCOUNTER15H,
                HPMCOUNTER16H,
                HPMCOUNTER17H,
                HPMCOUNTER18H,
                HPMCOUNTER19H,
                HPMCOUNTER20H,
                HPMCOUNTER21H,
                HPMCOUNTER22H,
                HPMCOUNTER23H,
                HPMCOUNTER24H,
                HPMCOUNTER25H,
                HPMCOUNTER26H,
                HPMCOUNTER27H,
                HPMCOUNTER28H,
                HPMCOUNTER29H,
                HPMCOUNTER30H,
                HPMCOUNTER31H,
            ]
            .iter()
            .cloned()
            .map(|a| (a, 0))
            .collect::<HashMap<_, _>>(),
        }
    }
}

// User Trap Setup (URW)
pub const USTATUS: u64 = 0x000; // User status register.
pub const UIE: u64 = 0x004; // User interrupt-enable register.
pub const UTVEC: u64 = 0x005; // User trap handler base address.

// User Trap Handling (URW)
pub const USCRATCH: u64 = 0x040; // Scratch register for user trap handlers.
pub const UEPC: u64 = 0x041; // User exception program counter.
pub const UCAUSE: u64 = 0x042; // User trap cause.
pub const UTVAL: u64 = 0x043; // User bad address or instruction.
pub const UIP: u64 = 0x044; // User interrupt pending.

// User Floating-Point CSRs (URW)
pub const FFLAGS: u64 = 0x001; // Floating-Point Accrued Exceptions.
pub const FRM: u64 = 0x002; // Floating-Point Dynamic Rounding Mode.
pub const FCSR: u64 = 0x003; // Floating-Point Control and Status Register (frm + fflags).

// User Counter/Timers (URO)
pub const CYCLE: u64 = 0xc00; // Cycle counter for RDCYCLE instruction.
pub const TIME: u64 = 0xC01; // Timer for RDTIME instruction.
pub const INSTRET: u64 = 0xc02; // Instructions-retired counter for RDINSTRET instruction.
pub const HPMCOUNTER3: u64 = 0xc03; // Performance-monitoring counter.
pub const HPMCOUNTER4: u64 = 0xc04; // Performance-monitoring counter.
pub const HPMCOUNTER5: u64 = 0xc05; // Performance-monitoring counter.
pub const HPMCOUNTER6: u64 = 0xc06; // Performance-monitoring counter.
pub const HPMCOUNTER7: u64 = 0xc07; // Performance-monitoring counter.
pub const HPMCOUNTER8: u64 = 0xc08; // Performance-monitoring counter.
pub const HPMCOUNTER9: u64 = 0xc09; // Performance-monitoring counter.
pub const HPMCOUNTER10: u64 = 0xc0a; // Performance-monitoring counter.
pub const HPMCOUNTER11: u64 = 0xc0b; // Performance-monitoring counter.
pub const HPMCOUNTER12: u64 = 0xc0c; // Performance-monitoring counter.
pub const HPMCOUNTER13: u64 = 0xc0d; // Performance-monitoring counter.
pub const HPMCOUNTER14: u64 = 0xc0e; // Performance-monitoring counter.
pub const HPMCOUNTER15: u64 = 0xc0f; // Performance-monitoring counter.
pub const HPMCOUNTER16: u64 = 0xc10; // Performance-monitoring counter.
pub const HPMCOUNTER17: u64 = 0xc11; // Performance-monitoring counter.
pub const HPMCOUNTER18: u64 = 0xc12; // Performance-monitoring counter.
pub const HPMCOUNTER19: u64 = 0xc13; // Performance-monitoring counter.
pub const HPMCOUNTER20: u64 = 0xc14; // Performance-monitoring counter.
pub const HPMCOUNTER21: u64 = 0xc15; // Performance-monitoring counter.
pub const HPMCOUNTER22: u64 = 0xc16; // Performance-monitoring counter.
pub const HPMCOUNTER23: u64 = 0xc17; // Performance-monitoring counter.
pub const HPMCOUNTER24: u64 = 0xc18; // Performance-monitoring counter.
pub const HPMCOUNTER25: u64 = 0xc19; // Performance-monitoring counter.
pub const HPMCOUNTER26: u64 = 0xc1a; // Performance-monitoring counter.
pub const HPMCOUNTER27: u64 = 0xc1b; // Performance-monitoring counter.
pub const HPMCOUNTER28: u64 = 0xc1c; // Performance-monitoring counter.
pub const HPMCOUNTER29: u64 = 0xc1d; // Performance-monitoring counter.
pub const HPMCOUNTER30: u64 = 0xc1e; // Performance-monitoring counter.
pub const HPMCOUNTER31: u64 = 0xc1f; // Performance-monitoring counter.
pub const CYCLEH: u64 = 0xc80; // Upper 32 bits of cycle, RV32I only.
pub const TIMEH: u64 = 0xc81; // Upper 32 bits of time, RV32I only.
pub const INSTRETH: u64 = 0xc82; // Upper 32 bits of instret, RV32I only.
pub const HPMCOUNTER3H: u64 = 0xc83; // Upper 32 bits of hpmcounter3, RV32I only.
pub const HPMCOUNTER4H: u64 = 0xc84; // Upper 32 bits of hpmcounter4, RV32I only.
pub const HPMCOUNTER5H: u64 = 0xc85; // Upper 32 bits of hpmcounter5, RV32I only.
pub const HPMCOUNTER6H: u64 = 0xc86; // Upper 32 bits of hpmcounter6, RV32I only.
pub const HPMCOUNTER7H: u64 = 0xc87; // Upper 32 bits of hpmcounter7, RV32I only.
pub const HPMCOUNTER8H: u64 = 0xc88; // Upper 32 bits of hpmcounter8, RV32I only.
pub const HPMCOUNTER9H: u64 = 0xc89; // Upper 32 bits of hpmcounter9, RV32I only.
pub const HPMCOUNTER10H: u64 = 0xc8a; // Upper 32 bits of hpmcounter10, RV32I only.
pub const HPMCOUNTER11H: u64 = 0xc8b; // Upper 32 bits of hpmcounter11, RV32I only.
pub const HPMCOUNTER12H: u64 = 0xc8c; // Upper 32 bits of hpmcounter12, RV32I only.
pub const HPMCOUNTER13H: u64 = 0xc8d; // Upper 32 bits of hpmcounter13, RV32I only.
pub const HPMCOUNTER14H: u64 = 0xc8e; // Upper 32 bits of hpmcounter14, RV32I only.
pub const HPMCOUNTER15H: u64 = 0xc8f; // Upper 32 bits of hpmcounter15, RV32I only.
pub const HPMCOUNTER16H: u64 = 0xc90; // Upper 32 bits of hpmcounter16, RV32I only.
pub const HPMCOUNTER17H: u64 = 0xc91; // Upper 32 bits of hpmcounter17, RV32I only.
pub const HPMCOUNTER18H: u64 = 0xc92; // Upper 32 bits of hpmcounter18, RV32I only.
pub const HPMCOUNTER19H: u64 = 0xc93; // Upper 32 bits of hpmcounter19, RV32I only.
pub const HPMCOUNTER20H: u64 = 0xc94; // Upper 32 bits of hpmcounter20, RV32I only.
pub const HPMCOUNTER21H: u64 = 0xc95; // Upper 32 bits of hpmcounter21, RV32I only.
pub const HPMCOUNTER22H: u64 = 0xc96; // Upper 32 bits of hpmcounter22, RV32I only.
pub const HPMCOUNTER23H: u64 = 0xc97; // Upper 32 bits of hpmcounter23, RV32I only.
pub const HPMCOUNTER24H: u64 = 0xc98; // Upper 32 bits of hpmcounter24, RV32I only.
pub const HPMCOUNTER25H: u64 = 0xc99; // Upper 32 bits of hpmcounter25, RV32I only.
pub const HPMCOUNTER26H: u64 = 0xc9a; // Upper 32 bits of hpmcounter26, RV32I only.
pub const HPMCOUNTER27H: u64 = 0xc9b; // Upper 32 bits of hpmcounter27, RV32I only.
pub const HPMCOUNTER28H: u64 = 0xc9c; // Upper 32 bits of hpmcounter28, RV32I only.
pub const HPMCOUNTER29H: u64 = 0xc9d; // Upper 32 bits of hpmcounter29, RV32I only.
pub const HPMCOUNTER30H: u64 = 0xc9e; // Upper 32 bits of hpmcounter30, RV32I only.
pub const HPMCOUNTER31H: u64 = 0xc9f; // Upper 32 bits of hpmcounter31, RV32I only.
