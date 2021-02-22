pub mod emulator;
mod isa;

const MASK_3BIT: u32 = 0b111;
const MASK_5BIT: u32 = 0b11111;
const MASK_7BIT: u32 = 0b1111111;
const MASK_12BIT: u32 = 0b111111111111;
