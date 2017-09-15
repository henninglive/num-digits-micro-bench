macro_rules! pow10 {
    (0)  => (1);
    (1)  => (10);
    (2)  => (100);
    (3)  => (1_000);
    (4)  => (10_000);
    (5)  => (100_000);
    (6)  => (1_000_000);
    (7)  => (10_000_000);
    (8)  => (100_000_000);
    (9)  => (1_000_000_000);
    (10) => (10_000_000_000);
    (11) => (100_000_000_000);
    (12) => (1_000_000_000_000);
    (13) => (pow10!{12} * 10);
    (14) => (pow10!{13} * 10);
    (15) => (pow10!{12} * 1_000);
    (16) => (pow10!{15} * 10);
    (17) => (pow10!{16} * 10);
    (18) => (pow10!{15} * 1_000);
    (19) => (pow10!{18} * 10);
    (20) => (pow10!{19} * 10);
    (21) => (pow10!{18} * 1_000);
    (22) => (pow10!{21} * 10);
    (23) => (pow10!{22} * 10);
    (24) => (pow10!{21} * 1_000);
    (25) => (pow10!{24} * 10);
    (26) => (pow10!{25} * 10);
    (27) => (pow10!{24} * 1_000);
    (28) => (pow10!{27} * 10);
    (29) => (pow10!{28} * 10);
    (30) => (pow10!{27} * 1_000);
    (31) => (pow10!{30} * 10);
    (32) => (pow10!{31} * 10);
    (33) => (pow10!{30} * 1_000);
    (34) => (pow10!{33} * 10);
    (35) => (pow10!{34} * 10);
    (36) => (pow10!{33} * 1_000);
    (37) => (pow10!{36} * 10);
    (38) => (pow10!{37} * 10);
    (39) => (pow10!{36} * 1_000);
}

macro_rules! dec_n {
    ($t:ident, 0)  => (                pub const DEC_0:  $t = pow10!(0););
    ($t:ident, 1)  => (dec_n!($t, 0);  pub const DEC_1:  $t = pow10!(1););
    ($t:ident, 2)  => (dec_n!($t, 1);  pub const DEC_2:  $t = pow10!(2););
    ($t:ident, 3)  => (dec_n!($t, 2);  pub const DEC_3:  $t = pow10!(3););
    ($t:ident, 4)  => (dec_n!($t, 3);  pub const DEC_4:  $t = pow10!(4););
    ($t:ident, 5)  => (dec_n!($t, 4);  pub const DEC_5:  $t = pow10!(5););
    ($t:ident, 6)  => (dec_n!($t, 5);  pub const DEC_6:  $t = pow10!(6););
    ($t:ident, 7)  => (dec_n!($t, 6);  pub const DEC_7:  $t = pow10!(7););
    ($t:ident, 8)  => (dec_n!($t, 7);  pub const DEC_8:  $t = pow10!(8););
    ($t:ident, 9)  => (dec_n!($t, 8);  pub const DEC_9:  $t = pow10!(9););
    ($t:ident, 10) => (dec_n!($t, 9);  pub const DEC_10: $t = pow10!(10););
    ($t:ident, 11) => (dec_n!($t, 10); pub const DEC_11: $t = pow10!(11););
    ($t:ident, 12) => (dec_n!($t, 11); pub const DEC_12: $t = pow10!(12););
    ($t:ident, 13) => (dec_n!($t, 12); pub const DEC_13: $t = pow10!(13););
    ($t:ident, 14) => (dec_n!($t, 13); pub const DEC_14: $t = pow10!(14););
    ($t:ident, 15) => (dec_n!($t, 14); pub const DEC_15: $t = pow10!(15););
    ($t:ident, 16) => (dec_n!($t, 15); pub const DEC_16: $t = pow10!(16););
    ($t:ident, 17) => (dec_n!($t, 16); pub const DEC_17: $t = pow10!(17););
    ($t:ident, 18) => (dec_n!($t, 17); pub const DEC_18: $t = pow10!(18););
    ($t:ident, 19) => (dec_n!($t, 18); pub const DEC_19: $t = pow10!(19););
    ($t:ident, 20) => (dec_n!($t, 19); pub const DEC_20: $t = pow10!(20););
    ($t:ident, 21) => (dec_n!($t, 20); pub const DEC_21: $t = pow10!(21););
    ($t:ident, 22) => (dec_n!($t, 21); pub const DEC_22: $t = pow10!(22););
    ($t:ident, 23) => (dec_n!($t, 22); pub const DEC_23: $t = pow10!(23););
    ($t:ident, 24) => (dec_n!($t, 23); pub const DEC_24: $t = pow10!(24););
    ($t:ident, 25) => (dec_n!($t, 24); pub const DEC_25: $t = pow10!(25););
    ($t:ident, 26) => (dec_n!($t, 25); pub const DEC_26: $t = pow10!(26););
    ($t:ident, 27) => (dec_n!($t, 26); pub const DEC_27: $t = pow10!(27););
    ($t:ident, 28) => (dec_n!($t, 27); pub const DEC_28: $t = pow10!(28););
    ($t:ident, 29) => (dec_n!($t, 28); pub const DEC_29: $t = pow10!(29););
    ($t:ident, 30) => (dec_n!($t, 29); pub const DEC_30: $t = pow10!(30););
    ($t:ident, 31) => (dec_n!($t, 30); pub const DEC_31: $t = pow10!(31););
    ($t:ident, 32) => (dec_n!($t, 31); pub const DEC_32: $t = pow10!(32););
    ($t:ident, 33) => (dec_n!($t, 32); pub const DEC_33: $t = pow10!(33););
    ($t:ident, 34) => (dec_n!($t, 33); pub const DEC_34: $t = pow10!(34););
    ($t:ident, 35) => (dec_n!($t, 34); pub const DEC_35: $t = pow10!(35););
    ($t:ident, 36) => (dec_n!($t, 35); pub const DEC_36: $t = pow10!(36););
    ($t:ident, 37) => (dec_n!($t, 36); pub const DEC_37: $t = pow10!(37););
    ($t:ident, 38) => (dec_n!($t, 37); pub const DEC_38: $t = pow10!(38););
}

pub mod u8 {
    dec_n!(u8, 2);
    pub static DEC: [u8; 4] = [
        DEC_0, DEC_1, DEC_2, DEC_2,
    ];
}

pub mod u16 {
    dec_n!(u16, 4);
    pub static DEC: [u16; 6] = [
        DEC_0, DEC_1, DEC_2, DEC_3, DEC_4,
        DEC_4,
    ];
}

pub mod u32 {
    dec_n!(u32, 9);
    pub static DEC: [u32; 11] = [
        DEC_0, DEC_1, DEC_2, DEC_3, DEC_4,
        DEC_5, DEC_6, DEC_7, DEC_8, DEC_9,
        DEC_9,
    ];
}

pub mod u64 {
    dec_n!(u64, 19);
    pub static DEC: [u64; 21] = [
        DEC_0,  DEC_1,  DEC_2,  DEC_3,  DEC_4,
        DEC_5,  DEC_6,  DEC_7,  DEC_8,  DEC_9,
        DEC_10, DEC_11, DEC_12, DEC_13, DEC_14,
        DEC_15, DEC_16, DEC_17, DEC_18, DEC_19,
        DEC_19,
    ];
}

pub mod u128 {
    dec_n!(u128, 38);
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
