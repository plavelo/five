pub const MASK_3BIT: u64 = 0b111;
pub const MASK_4BIT: u64 = 0b1111;
pub const MASK_5BIT: u64 = 0b11111;
pub const MASK_6BIT: u64 = 0b111111;
pub const MASK_8BIT: u64 = 0b11111111;
pub const MASK_12BIT: u64 = 0b111111111111;

pub fn extend_sign(value: u64, nbits: u32) -> i64 {
    let target = value as i64;
    let shamt = 64 - nbits;
    target.wrapping_shl(shamt).wrapping_shr(shamt)
}

pub fn shift_amount(value: u64) -> u64 {
    value & MASK_6BIT
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extend_sign_ok() {
        let value1: u64 = 0b110011;
        assert_eq!(
            extend_sign(value1, 6),
            0b1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_0011u64
                as i64,
        );

        let value2: u64 = 0b1_0011_0011_0011_0011;
        assert_eq!(
            extend_sign(value2, 17),
            0b1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_0011_0011_0011_0011u64
                as i64,
        );
    }
}
