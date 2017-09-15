macro_rules! pow10 {
    (0)  => (1);
    (1)  => (pow10!{0} * 10);
    (2)  => (pow10!{1} * 10);
    (3)  => (pow10!{2} * 10);
    (4)  => (pow10!{3} * 10);
    (5)  => (pow10!{4} * 10);
    (6)  => (pow10!{5} * 10);
    (7)  => (pow10!{6} * 10);
    (8)  => (pow10!{7} * 10);
    (9)  => (pow10!{8} * 10);
    (10) => (pow10!{9} * 10);
    (11) => (pow10!{10} * 10);
    (12) => (pow10!{11} * 10);
    (13) => (pow10!{12} * 10);
    (14) => (pow10!{13} * 10);
    (15) => (pow10!{14} * 10);
    (16) => (pow10!{15} * 10);
    (17) => (pow10!{16} * 10);
    (18) => (pow10!{17} * 10);
    (19) => (pow10!{18} * 10);
    (20) => (pow10!{19} * 10);
    (21) => (pow10!{20} * 10);
    (22) => (pow10!{21} * 10);
    (23) => (pow10!{22} * 10);
    (24) => (pow10!{23} * 10);
    (25) => (pow10!{24} * 10);
    (26) => (pow10!{25} * 10);
    (27) => (pow10!{26} * 10);
    (28) => (pow10!{27} * 10);
    (29) => (pow10!{28} * 10);
    (30) => (pow10!{29} * 10);
    (31) => (pow10!{30} * 10);
    (32) => (pow10!{31} * 10);
    (33) => (pow10!{32} * 10);
    (34) => (pow10!{33} * 10);
    (35) => (pow10!{34} * 10);
    (36) => (pow10!{35} * 10);
    (37) => (pow10!{36} * 10);
    (38) => (pow10!{37} * 10);
    (39) => (pow10!{38} * 10);
}

pub mod dec_8 {
    pub const DEC_0: u8 = pow10!(0);
    pub const DEC_1: u8 = pow10!(1);
    pub const DEC_2: u8 = pow10!(2);
    pub static DEC: [u8; 4] = [
        DEC_0, DEC_1, DEC_2, DEC_2,
    ];
}

pub mod dec_16 {
    pub const DEC_0: u16 = pow10!(0);
    pub const DEC_1: u16 = pow10!(1);
    pub const DEC_2: u16 = pow10!(2);
    pub const DEC_3: u16 = pow10!(3);
    pub const DEC_4: u16 = pow10!(4);
    pub static DEC: [u16; 6] = [
        DEC_0, DEC_1, DEC_2, DEC_3, DEC_4,
        DEC_4,
    ];
}

pub mod dec_32 {
    pub const DEC_0: u32 = pow10!(0);
    pub const DEC_1: u32 = pow10!(1);
    pub const DEC_2: u32 = pow10!(2);
    pub const DEC_3: u32 = pow10!(3);
    pub const DEC_4: u32 = pow10!(4);
    pub const DEC_5: u32 = pow10!(5);
    pub const DEC_6: u32 = pow10!(6);
    pub const DEC_7: u32 = pow10!(7);
    pub const DEC_8: u32 = pow10!(8);
    pub const DEC_9: u32 = pow10!(9);
    pub static DEC: [u32; 11] = [
        DEC_0, DEC_1, DEC_2, DEC_3, DEC_4,
        DEC_5, DEC_6, DEC_7, DEC_8, DEC_9,
        DEC_9,
    ];
}

pub mod dec_64 {
    pub const DEC_0:  u64 = pow10!(0);
    pub const DEC_1:  u64 = pow10!(1);
    pub const DEC_2:  u64 = pow10!(2);
    pub const DEC_3:  u64 = pow10!(3);
    pub const DEC_4:  u64 = pow10!(4);
    pub const DEC_5:  u64 = pow10!(5);
    pub const DEC_6:  u64 = pow10!(6);
    pub const DEC_7:  u64 = pow10!(7);
    pub const DEC_8:  u64 = pow10!(8);
    pub const DEC_9:  u64 = pow10!(9);
    pub const DEC_10: u64 = pow10!(10);
    pub const DEC_11: u64 = pow10!(11);
    pub const DEC_12: u64 = pow10!(12);
    pub const DEC_13: u64 = pow10!(13);
    pub const DEC_14: u64 = pow10!(14);
    pub const DEC_15: u64 = pow10!(15);
    pub const DEC_16: u64 = pow10!(16);
    pub const DEC_17: u64 = pow10!(17);
    pub const DEC_18: u64 = pow10!(18);
    pub const DEC_19: u64 = pow10!(19);
    pub static DEC: [u64; 21] = [
        DEC_0,  DEC_1,  DEC_2,  DEC_3,  DEC_4,
        DEC_5,  DEC_6,  DEC_7,  DEC_8,  DEC_9,
        DEC_10, DEC_11, DEC_12, DEC_13, DEC_14,
        DEC_15, DEC_16, DEC_17, DEC_18, DEC_19,
        DEC_19,
    ];
}

pub mod dec_128 {
    pub const DEC_0:  u128 = pow10!(0);
    pub const DEC_1:  u128 = pow10!(1);
    pub const DEC_2:  u128 = pow10!(2);
    pub const DEC_3:  u128 = pow10!(3);
    pub const DEC_4:  u128 = pow10!(4);
    pub const DEC_5:  u128 = pow10!(5);
    pub const DEC_6:  u128 = pow10!(6);
    pub const DEC_7:  u128 = pow10!(7);
    pub const DEC_8:  u128 = pow10!(8);
    pub const DEC_9:  u128 = pow10!(9);
    pub const DEC_10: u128 = pow10!(10);
    pub const DEC_11: u128 = pow10!(11);
    pub const DEC_12: u128 = pow10!(12);
    pub const DEC_13: u128 = pow10!(13);
    pub const DEC_14: u128 = pow10!(14);
    pub const DEC_15: u128 = pow10!(15);
    pub const DEC_16: u128 = pow10!(16);
    pub const DEC_17: u128 = pow10!(17);
    pub const DEC_18: u128 = pow10!(18);
    pub const DEC_19: u128 = pow10!(19);
    pub const DEC_20: u128 = pow10!(20);
    pub const DEC_21: u128 = pow10!(21);
    pub const DEC_22: u128 = pow10!(22);
    pub const DEC_23: u128 = pow10!(23);
    pub const DEC_24: u128 = pow10!(24);
    pub const DEC_25: u128 = pow10!(25);
    pub const DEC_26: u128 = pow10!(26);
    pub const DEC_27: u128 = pow10!(27);
    pub const DEC_28: u128 = pow10!(28);
    pub const DEC_29: u128 = pow10!(29);
    pub const DEC_30: u128 = pow10!(30);
    pub const DEC_31: u128 = pow10!(31);
    pub const DEC_32: u128 = pow10!(32);
    pub const DEC_33: u128 = pow10!(33);
    pub const DEC_34: u128 = pow10!(34);
    pub const DEC_35: u128 = pow10!(35);
    pub const DEC_36: u128 = pow10!(36);
    pub const DEC_37: u128 = pow10!(37);
    pub const DEC_38: u128 = pow10!(38);
    pub static DEC: [u128; 40] = [
        DEC_0,  DEC_1,  DEC_2,  DEC_3,  DEC_4,
        DEC_5,  DEC_6,  DEC_7,  DEC_8,  DEC_9,
        DEC_10, DEC_11, DEC_12, DEC_13, DEC_14,
        DEC_15, DEC_16, DEC_17, DEC_18, DEC_19,
        DEC_20, DEC_21, DEC_22, DEC_23, DEC_24,
        DEC_25, DEC_26, DEC_27, DEC_28, DEC_29,
        DEC_30, DEC_31, DEC_32, DEC_33, DEC_34,
        DEC_35, DEC_36, DEC_37, DEC_38, DEC_38,
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
