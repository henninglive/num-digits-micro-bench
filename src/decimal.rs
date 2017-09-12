pub mod dec_8 {
    pub const DEC_0: u8 = 0;
    pub const DEC_1: u8 = 10;
    pub const DEC_2: u8 = 100;
    pub static DEC: [u8; 4] = [
        DEC_0, DEC_1, DEC_2, DEC_2,
    ];
}

pub mod dec_16 {
    pub const DEC_0: u16 = 0;
    pub const DEC_1: u16 = 10;
    pub const DEC_2: u16 = 100;
    pub const DEC_3: u16 = 1000;
    pub const DEC_4: u16 = 10000;
    pub static DEC: [u16; 6] = [
        DEC_0, DEC_1, DEC_2, DEC_3, DEC_4,
        DEC_4,
    ];
}

pub mod dec_32 {
    pub const DEC_0: u32 = 0;
    pub const DEC_1: u32 = 10;
    pub const DEC_2: u32 = 100;
    pub const DEC_3: u32 = 1000;
    pub const DEC_4: u32 = 10000;
    pub const DEC_5: u32 = 100000;
    pub const DEC_6: u32 = 1000000;
    pub const DEC_7: u32 = 10000000;
    pub const DEC_8: u32 = 100000000;
    pub const DEC_9: u32 = 1000000000;
    pub static DEC: [u32; 11] = [
        DEC_0, DEC_1, DEC_2, DEC_3, DEC_4,
        DEC_5, DEC_6, DEC_7, DEC_8, DEC_9,
        DEC_9,
    ];
}

pub mod dec_64 {
    pub const DEC_0:  u64 = 0;
    pub const DEC_1:  u64 = 10;
    pub const DEC_2:  u64 = 100;
    pub const DEC_3:  u64 = 1000;
    pub const DEC_4:  u64 = 10000;
    pub const DEC_5:  u64 = 100000;
    pub const DEC_6:  u64 = 1000000;
    pub const DEC_7:  u64 = 10000000;
    pub const DEC_8:  u64 = 100000000;
    pub const DEC_9:  u64 = 1000000000;
    pub const DEC_10: u64 = 10000000000;
    pub const DEC_11: u64 = 100000000000;
    pub const DEC_12: u64 = 1000000000000;
    pub const DEC_13: u64 = 10000000000000;
    pub const DEC_14: u64 = 100000000000000;
    pub const DEC_15: u64 = 1000000000000000;
    pub const DEC_16: u64 = 10000000000000000;
    pub const DEC_17: u64 = 100000000000000000;
    pub const DEC_18: u64 = 1000000000000000000;
    pub const DEC_19: u64 = 10000000000000000000;
    pub static DEC: [u64; 21] = [
        DEC_0,  DEC_1,  DEC_2,  DEC_3,  DEC_4,
        DEC_5,  DEC_6,  DEC_7,  DEC_8,  DEC_9,
        DEC_10, DEC_11, DEC_12, DEC_13, DEC_14,
        DEC_15, DEC_16, DEC_17, DEC_18, DEC_19,
        DEC_19,
    ];
}

pub static BIN_TO_DEC: [u8; 130] = [
     1,  1,  1,  1,  1,  2,  2,  2,  3,  3,  3,  4,
     4,  4,  4,  5,  5,  5,  6,  6,  6,  7,  7,  7,
     7,  8,  8,  8,  9,  9,  9, 10, 10, 10, 10, 11,
    11, 11, 12, 12, 12, 13, 13, 13, 13, 14, 14, 14,
    15, 15, 15, 16, 16, 16, 16, 17, 17, 17, 18, 18,
    18, 19, 19, 19, 19, 20, 20, 20, 21, 21, 21, 22,
    22, 22, 22, 23, 23, 23, 24, 24, 24, 25, 25, 25,
    25, 26, 26, 26, 27, 27, 27, 28, 28, 28, 28, 29,
    29, 29, 30, 30, 30, 31, 31, 31, 32, 32, 32, 32,
    33, 33, 33, 34, 34, 34, 35, 35, 35, 35, 36, 36,
    36, 37, 37, 37, 38, 38, 38, 38, 39, 39,
];
