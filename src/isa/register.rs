pub const ZERO: usize = 0;
pub const RA: usize = 1;
pub const SP: usize = 2;
pub const GP: usize = 3;
pub const TP: usize = 4;
pub const T0: usize = 5;
pub const T1: usize = 6;
pub const T2: usize = 7;
pub const S0: usize = 8;
pub const S1: usize = 9;
pub const A0: usize = 10;
pub const A1: usize = 11;
pub const A2: usize = 12;
pub const A3: usize = 13;
pub const A4: usize = 14;
pub const A5: usize = 15;
pub const A6: usize = 16;
pub const A7: usize = 17;
pub const S2: usize = 18;
pub const S3: usize = 19;
pub const S4: usize = 20;
pub const S5: usize = 21;
pub const S6: usize = 22;
pub const S7: usize = 23;
pub const S8: usize = 24;
pub const S9: usize = 25;
pub const S10: usize = 26;
pub const S11: usize = 27;
pub const T3: usize = 28;
pub const T4: usize = 29;
pub const T5: usize = 30;
pub const T6: usize = 31;

pub const FT0: usize = 0;
pub const FT1: usize = 1;
pub const FT2: usize = 2;
pub const FT3: usize = 3;
pub const FT4: usize = 4;
pub const FT5: usize = 5;
pub const FT6: usize = 6;
pub const FT7: usize = 7;
pub const FS0: usize = 8;
pub const FS1: usize = 9;
pub const FA0: usize = 10;
pub const FA1: usize = 11;
pub const FA2: usize = 12;
pub const FA3: usize = 13;
pub const FA4: usize = 14;
pub const FA5: usize = 15;
pub const FA6: usize = 16;
pub const FA7: usize = 17;
pub const FS2: usize = 18;
pub const FS3: usize = 19;
pub const FS4: usize = 20;
pub const FS5: usize = 21;
pub const FS6: usize = 22;
pub const FS7: usize = 23;
pub const FS8: usize = 24;
pub const FS9: usize = 25;
pub const FS10: usize = 26;
pub const FS11: usize = 27;
pub const FT8: usize = 28;
pub const FT9: usize = 29;
pub const FT10: usize = 30;
pub const FT11: usize = 31;

pub fn xname<'a>(register: usize) -> &'a str {
    match register {
        ZERO => "zero",
        RA => "ra",
        SP => "sp",
        GP => "gp",
        TP => "tp",
        T0 => "t0",
        T1 => "t1",
        T2 => "t2",
        S0 => "s0",
        S1 => "s1",
        A0 => "a0",
        A1 => "a1",
        A2 => "a2",
        A3 => "a3",
        A4 => "a4",
        A5 => "a5",
        A6 => "a6",
        A7 => "a7",
        S2 => "s2",
        S3 => "s3",
        S4 => "s4",
        S5 => "s5",
        S6 => "s6",
        S7 => "s7",
        S8 => "s8",
        S9 => "s9",
        S10 => "s10",
        S11 => "s11",
        T3 => "t3",
        T4 => "t4",
        T5 => "t5",
        T6 => "t6",
        _ => "unknown",
    }
}

pub fn fname<'a>(register: usize) -> &'a str {
    match register {
        FT0 => "ft0",
        FT1 => "ft1",
        FT2 => "ft2",
        FT3 => "ft3",
        FT4 => "ft4",
        FT5 => "ft5",
        FT6 => "ft6",
        FT7 => "ft7",
        FS0 => "fs0",
        FS1 => "fs1",
        FA0 => "fa0",
        FA1 => "fa1",
        FA2 => "fa2",
        FA3 => "fa3",
        FA4 => "fa4",
        FA5 => "fa5",
        FA6 => "fa6",
        FA7 => "fa7",
        FS2 => "fs2",
        FS3 => "fs3",
        FS4 => "fs4",
        FS5 => "fs5",
        FS6 => "fs6",
        FS7 => "fs7",
        FS8 => "fs8",
        FS9 => "fs9",
        FS10 => "fs10",
        FS11 => "fs11",
        FT8 => "ft8",
        FT9 => "ft9",
        FT10 => "ft10",
        FT11 => "ft11",
        _ => "unknown",
    }
}
