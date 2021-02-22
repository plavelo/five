use crate::emulator::cpu::csr::Csr;
use std::collections::HashMap;

pub struct MachineLevelCsr {
    csr: HashMap<u32, u32>,
}

impl Csr for MachineLevelCsr {
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

impl Default for MachineLevelCsr {
    fn default() -> Self {
        Self {
            csr: [
                // Machine Information Registers (MRO)
                MVENDORID,
                MARCHID,
                MIMPID,
                MHARTID,
                // Machine Trap Setup (MRW)
                MSTATUS,
                MISA,
                MEDELEG,
                MIDELEG,
                MIE,
                MTVEC,
                MCOUNTEREN,
                // Machine Trap Handling (MRW)
                MSCRATCH,
                MEPC,
                MCAUSE,
                MTVAL,
                MIP,
                // Machine Memory Protection (MRW)
                PMPCFG0,
                PMPCFG1,
                PMPCFG2,
                PMPCFG3,
                PMPADDR0,
                PMPADDR1,
                PMPADDR2,
                PMPADDR3,
                PMPADDR4,
                PMPADDR5,
                PMPADDR6,
                PMPADDR7,
                PMPADDR8,
                PMPADDR9,
                PMPADDR10,
                PMPADDR11,
                PMPADDR12,
                PMPADDR13,
                PMPADDR14,
                PMPADDR15,
                // Machine Counter/Timers (MRW)
                MCYCLE,
                MINSTRET,
                MHPMCOUNTER,
                MHPMCOUNTER4,
                MHPMCOUNTER5,
                MHPMCOUNTER6,
                MHPMCOUNTER7,
                MHPMCOUNTER8,
                MHPMCOUNTER9,
                MHPMCOUNTER10,
                MHPMCOUNTER11,
                MHPMCOUNTER12,
                MHPMCOUNTER13,
                MHPMCOUNTER14,
                MHPMCOUNTER15,
                MHPMCOUNTER16,
                MHPMCOUNTER17,
                MHPMCOUNTER18,
                MHPMCOUNTER19,
                MHPMCOUNTER20,
                MHPMCOUNTER21,
                MHPMCOUNTER22,
                MHPMCOUNTER23,
                MHPMCOUNTER24,
                MHPMCOUNTER25,
                MHPMCOUNTER26,
                MHPMCOUNTER27,
                MHPMCOUNTER28,
                MHPMCOUNTER29,
                MHPMCOUNTER30,
                MHPMCOUNTER31,
                MCYCLEH,
                MINSTRETH,
                MHPMCOUNTER3H,
                MHPMCOUNTER31H,
                // Machine Counter Setup (MRW)
                MCOUNTINHIBIT,
                MHPMEVENT3,
                MHPMEVENT4,
                MHPMEVENT5,
                MHPMEVENT6,
                MHPMEVENT7,
                MHPMEVENT8,
                MHPMEVENT9,
                MHPMEVENT10,
                MHPMEVENT11,
                MHPMEVENT12,
                MHPMEVENT13,
                MHPMEVENT14,
                MHPMEVENT15,
                MHPMEVENT16,
                MHPMEVENT17,
                MHPMEVENT18,
                MHPMEVENT19,
                MHPMEVENT20,
                MHPMEVENT21,
                MHPMEVENT22,
                MHPMEVENT23,
                MHPMEVENT24,
                MHPMEVENT25,
                MHPMEVENT26,
                MHPMEVENT27,
                MHPMEVENT28,
                MHPMEVENT29,
                MHPMEVENT30,
                MHPMEVENT31,
                // Debug/Trace Registers (shared with Debug Mode) (MRW)
                TSELECT,
                TDATA1,
                TDATA2,
                TDATA3,
                // Debug Mode Registers (DRW)
                DCSR,
                DPC,
                DSCRATCH0,
                DSCRATCH1,
            ]
            .iter()
            .cloned()
            .map(|a| (a, 0))
            .collect::<HashMap<_, _>>(),
        }
    }
}

// Machine Information Registers (MRO)
const MVENDORID: u32 = 0xf11; // Vendor ID.
const MARCHID: u32 = 0xf12; // Architecture ID.
const MIMPID: u32 = 0xf13; // Implementation ID.
const MHARTID: u32 = 0xf14; // Hardware thread ID.

// Machine Trap Setup (MRW)
const MSTATUS: u32 = 0x300; // Machine status register.
const MISA: u32 = 0x301; // ISA and extensions.
const MEDELEG: u32 = 0x302; // Machine exception delegation register.
const MIDELEG: u32 = 0x303; // Machine interrupt delegation register.
const MIE: u32 = 0x304; // Machine interrupt-enable register.
const MTVEC: u32 = 0x305; // Machine trap-handler base address.
const MCOUNTEREN: u32 = 0x306; // Machine counter enable.

// Machine Trap Handling (MRW)
const MSCRATCH: u32 = 0x340; // Scratch register for machine trap handlers.
const MEPC: u32 = 0x341; // Machine exception program counter.
const MCAUSE: u32 = 0x342; // Machine trap cause.
const MTVAL: u32 = 0x343; // Machine bad address or instruction.
const MIP: u32 = 0x344; // Machine interrupt pending.

// Machine Memory Protection (MRW)
const PMPCFG0: u32 = 0x3a0; // Physical memory protection configuration.
const PMPCFG1: u32 = 0x3a1; // Physical memory protection configuration, RV32 only.
const PMPCFG2: u32 = 0x3a2; // Physical memory protection configuration.
const PMPCFG3: u32 = 0x3a3; // Physical memory protection configuration, RV32 only.
const PMPADDR0: u32 = 0x3b0; // Physical memory protection address register.
const PMPADDR1: u32 = 0x3b1; // Physical memory protection address register.
const PMPADDR2: u32 = 0x3b2; // Physical memory protection address register.
const PMPADDR3: u32 = 0x3b3; // Physical memory protection address register.
const PMPADDR4: u32 = 0x3b4; // Physical memory protection address register.
const PMPADDR5: u32 = 0x3b5; // Physical memory protection address register.
const PMPADDR6: u32 = 0x3b6; // Physical memory protection address register.
const PMPADDR7: u32 = 0x3b7; // Physical memory protection address register.
const PMPADDR8: u32 = 0x3b8; // Physical memory protection address register.
const PMPADDR9: u32 = 0x3b9; // Physical memory protection address register.
const PMPADDR10: u32 = 0x3ba; // Physical memory protection address register.
const PMPADDR11: u32 = 0x3bb; // Physical memory protection address register.
const PMPADDR12: u32 = 0x3bc; // Physical memory protection address register.
const PMPADDR13: u32 = 0x3bd; // Physical memory protection address register.
const PMPADDR14: u32 = 0x3be; // Physical memory protection address register.
const PMPADDR15: u32 = 0x3bf; // Physical memory protection address register.

// Machine Counter/Timers (MRW)
const MCYCLE: u32 = 0xb00; // Machine cycle counter.
const MINSTRET: u32 = 0xb02; // Machine instructions-retired counter.
const MHPMCOUNTER: u32 = 0xb03; // Machine performance-monitoring counter.
const MHPMCOUNTER4: u32 = 0xb04; // Machine performance-monitoring counter.
const MHPMCOUNTER5: u32 = 0xb05; // Machine performance-monitoring counter.
const MHPMCOUNTER6: u32 = 0xb06; // Machine performance-monitoring counter.
const MHPMCOUNTER7: u32 = 0xb07; // Machine performance-monitoring counter.
const MHPMCOUNTER8: u32 = 0xb08; // Machine performance-monitoring counter.
const MHPMCOUNTER9: u32 = 0xb09; // Machine performance-monitoring counter.
const MHPMCOUNTER10: u32 = 0xb0a; // Machine performance-monitoring counter.
const MHPMCOUNTER11: u32 = 0xb0b; // Machine performance-monitoring counter.
const MHPMCOUNTER12: u32 = 0xb0c; // Machine performance-monitoring counter.
const MHPMCOUNTER13: u32 = 0xb0d; // Machine performance-monitoring counter.
const MHPMCOUNTER14: u32 = 0xb0e; // Machine performance-monitoring counter.
const MHPMCOUNTER15: u32 = 0xb0f; // Machine performance-monitoring counter.
const MHPMCOUNTER16: u32 = 0xb10; // Machine performance-monitoring counter.
const MHPMCOUNTER17: u32 = 0xb11; // Machine performance-monitoring counter.
const MHPMCOUNTER18: u32 = 0xb12; // Machine performance-monitoring counter.
const MHPMCOUNTER19: u32 = 0xb13; // Machine performance-monitoring counter.
const MHPMCOUNTER20: u32 = 0xb14; // Machine performance-monitoring counter.
const MHPMCOUNTER21: u32 = 0xb15; // Machine performance-monitoring counter.
const MHPMCOUNTER22: u32 = 0xb16; // Machine performance-monitoring counter.
const MHPMCOUNTER23: u32 = 0xb17; // Machine performance-monitoring counter.
const MHPMCOUNTER24: u32 = 0xb18; // Machine performance-monitoring counter.
const MHPMCOUNTER25: u32 = 0xb19; // Machine performance-monitoring counter.
const MHPMCOUNTER26: u32 = 0xb1a; // Machine performance-monitoring counter.
const MHPMCOUNTER27: u32 = 0xb1b; // Machine performance-monitoring counter.
const MHPMCOUNTER28: u32 = 0xb1c; // Machine performance-monitoring counter.
const MHPMCOUNTER29: u32 = 0xb1d; // Machine performance-monitoring counter.
const MHPMCOUNTER30: u32 = 0xb1e; // Machine performance-monitoring counter.
const MHPMCOUNTER31: u32 = 0xb1f; // Machine performance-monitoring counter.
const MCYCLEH: u32 = 0xb80; // Upper 32 bits of mcycle, RV32I only.
const MINSTRETH: u32 = 0xb82; // Upper 32 bits of minstret, RV32I only.
const MHPMCOUNTER3H: u32 = 0xb83; // Upper 32 bits of mhpmcounter3, RV32I only.
const MHPMCOUNTER31H: u32 = 0xb9f; // Upper 32 bits of mhpmcounter31, RV32I only.

// Machine Counter Setup (MRW)
const MCOUNTINHIBIT: u32 = 0x320; // Machine counter-inhibit register.
const MHPMEVENT3: u32 = 0x323; // Machine performance-monitoring event selector.
const MHPMEVENT4: u32 = 0x324; // Machine performance-monitoring event selector.
const MHPMEVENT5: u32 = 0x325; // Machine performance-monitoring event selector.
const MHPMEVENT6: u32 = 0x326; // Machine performance-monitoring event selector.
const MHPMEVENT7: u32 = 0x327; // Machine performance-monitoring event selector.
const MHPMEVENT8: u32 = 0x328; // Machine performance-monitoring event selector.
const MHPMEVENT9: u32 = 0x329; // Machine performance-monitoring event selector.
const MHPMEVENT10: u32 = 0x32a; // Machine performance-monitoring event selector.
const MHPMEVENT11: u32 = 0x32b; // Machine performance-monitoring event selector.
const MHPMEVENT12: u32 = 0x32c; // Machine performance-monitoring event selector.
const MHPMEVENT13: u32 = 0x32d; // Machine performance-monitoring event selector.
const MHPMEVENT14: u32 = 0x32e; // Machine performance-monitoring event selector.
const MHPMEVENT15: u32 = 0x32f; // Machine performance-monitoring event selector.
const MHPMEVENT16: u32 = 0x330; // Machine performance-monitoring event selector.
const MHPMEVENT17: u32 = 0x331; // Machine performance-monitoring event selector.
const MHPMEVENT18: u32 = 0x332; // Machine performance-monitoring event selector.
const MHPMEVENT19: u32 = 0x333; // Machine performance-monitoring event selector.
const MHPMEVENT20: u32 = 0x334; // Machine performance-monitoring event selector.
const MHPMEVENT21: u32 = 0x335; // Machine performance-monitoring event selector.
const MHPMEVENT22: u32 = 0x336; // Machine performance-monitoring event selector.
const MHPMEVENT23: u32 = 0x337; // Machine performance-monitoring event selector.
const MHPMEVENT24: u32 = 0x338; // Machine performance-monitoring event selector.
const MHPMEVENT25: u32 = 0x339; // Machine performance-monitoring event selector.
const MHPMEVENT26: u32 = 0x33a; // Machine performance-monitoring event selector.
const MHPMEVENT27: u32 = 0x33b; // Machine performance-monitoring event selector.
const MHPMEVENT28: u32 = 0x33c; // Machine performance-monitoring event selector.
const MHPMEVENT29: u32 = 0x33d; // Machine performance-monitoring event selector.
const MHPMEVENT30: u32 = 0x33e; // Machine performance-monitoring event selector.
const MHPMEVENT31: u32 = 0x33f; // Machine performance-monitoring event selector.

// Debug/Trace Registers (shared with Debug Mode) (MRW)
const TSELECT: u32 = 0x7a0; // Debug/Trace trigger register select.
const TDATA1: u32 = 0x7a1; // First Debug/Trace trigger data register.
const TDATA2: u32 = 0x7a2; // Second Debug/Trace trigger data register.
const TDATA3: u32 = 0x7a3; // Third Debug/Trace trigger data register.

// Debug Mode Registers (DRW)
const DCSR: u32 = 0x7b0; // Debug control and status register.
const DPC: u32 = 0x7b1; // Debug PC.
const DSCRATCH0: u32 = 0x7b2; // Debug scratch register 0.
const DSCRATCH1: u32 = 0x7b3; // Debug scratch register 1.
