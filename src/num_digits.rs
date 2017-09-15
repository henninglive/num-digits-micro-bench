extern crate itoa;

pub trait NumDigits {
    fn log(&self) -> usize;
    fn str_format(&self) -> usize;
    fn str_format_stack(&self) -> usize;
    fn str_itoa_stack(&self) -> usize;
    fn div_loop(&self) -> usize;
    fn div_unrolled(&self) -> usize;
    fn mul_loop(&self) -> usize;
    fn cmp_list(&self) -> usize;
    fn pattern_match(&self) -> usize;
    fn binary_search(&self) -> usize;
    fn most_significant_bit(&self) -> usize;
}

impl NumDigits for u8 {
    #[inline]
    fn log(&self) -> usize {
        (*self as f32).log10() as u32 as usize + 1
    }

    #[inline]
    fn str_format(&self) -> usize {
        format!("{}", *self).len()
    }

    #[inline]
    fn str_format_stack(&self) -> usize {
        use std::io::{Write, Cursor};
        let mut b = [0u8; 3];
        let mut c = Cursor::new(&mut b[..]);
        write!(c, "{}", *self).unwrap();
        c.position() as usize
    }

    #[inline]
    fn str_itoa_stack(&self) -> usize {
        use std::io::Cursor;
        let mut b = [0u8; 3];
        let mut c = Cursor::new(&mut b[..]);
        itoa::write(&mut c, *self).unwrap();
        c.position() as usize
    }

    #[inline]
    fn div_loop(&self) -> usize {
        let mut len = 1;
        let mut num = *self;
        while num >= 10 {
            num /= 10;
            len += 1;
        }
        len
    }

    #[inline]
    fn div_unrolled(&self) -> usize {
        let num = *self;
        if num < 10 {
            return 1;
        }
        if num < 100 {
            return 2;
        }
        3
    }

    #[inline]
    fn mul_loop(&self) -> usize {
        let mut len = 1;
        let mut n = 1u8;
        while len < 3 {
            n *= 10;
            if n > *self {
                return len;
            }
            len += 1;
        }
        len
    }

    #[inline]
    fn cmp_list(&self) -> usize {
        use ::decimal::dec_8::*;
        if *self >= DEC_2 {
            3
        } else if *self >= DEC_1 {
            2
        } else {
            1
        }
    }

    #[inline]
    fn pattern_match(&self) -> usize {
        use ::decimal::dec_8::*;
        use ::std::u8::MIN;
        match *self {
              MIN .. DEC_1 => 1,
            DEC_1 .. DEC_2 => 2,
            _ => 3,
        }
    }

    #[inline]
    fn binary_search(&self) -> usize {
        use ::decimal::dec_8::*;
        let num = *self;
        if num < DEC_2 {
            if num < DEC_1 {
                1
            } else {
                2
            }
        } else {
            3
        }
    }

    #[inline]
    fn most_significant_bit(&self) -> usize {
        // We know that "2^i <= num < 2^(i+1)",
        // Where i is the highest set bit of the num.
        // Use ctlz and lookup tables to resolve number of
        // didgits with a single if statment.
        use ::decimal::dec_8::*;
        use ::decimal::BIN_TO_DEC;

        let i = 8 - (*self).leading_zeros() as usize;
        let l = BIN_TO_DEC[i];
        let h = BIN_TO_DEC[i + 1];
        if *self >= DEC[(h - 1) as usize] {
            h as usize
        } else {
            l as usize
        }
    }
}

impl NumDigits for u16 {
    #[inline]
    fn log(&self) -> usize {
        (*self as f32).log10() as u32 as usize + 1
    }

    #[inline]
    fn str_format(&self) -> usize {
        format!("{}", *self).len()
    }

    #[inline]
    fn str_format_stack(&self) -> usize {
        use std::io::{Write, Cursor};
        let mut b = [0u8; 5];
        let mut c = Cursor::new(&mut b[..]);
        write!(c, "{}", *self).unwrap();
        c.position() as usize
    }

    #[inline]
    fn str_itoa_stack(&self) -> usize {
        use std::io::Cursor;
        let mut b = [0u8; 5];
        let mut c = Cursor::new(&mut b[..]);
        itoa::write(&mut c, *self).unwrap();
        c.position() as usize
    }

    #[inline]
    fn div_loop(&self) -> usize {
        let mut len = 1;
        let mut num = *self;
        while num >= 10 {
            num /= 10;
            len += 1;
        }
        len
    }

    #[inline]
    fn div_unrolled(&self) -> usize {
        let num = *self;
        if num < 10 {
            return 1;
        }
        if num < 100 {
            return 2;
        }
        if num < 1000 {
            return 3;
        }
        if num < 10000 {
            return 4;
        }
        5
    }

    #[inline]
    fn mul_loop(&self) -> usize {
        let mut len = 1;
        let mut n = 1u16;
        while len < 5 {
            n *= 10;
            if n > *self {
                return len;
            }
            len += 1;
        }
        len
    }

    #[inline]
    fn cmp_list(&self) -> usize {
        use ::decimal::dec_16::*;
        if *self >= DEC_4 {
            5
        } else if *self >= DEC_3 {
            4
        } else if *self >= DEC_2 {
            3
        } else if *self >= DEC_1 {
            2
        } else {
            1
        }
    }

    #[inline]
    fn pattern_match(&self) -> usize {
        use ::decimal::dec_16::*;
        use ::std::u16::MIN;
        match *self {
              MIN .. DEC_1 => 1,
            DEC_1 .. DEC_2 => 2,
            DEC_2 .. DEC_3 => 3,
            DEC_3 .. DEC_4 => 4,
            _ => 5,
        }
    }

    #[inline]
    fn binary_search(&self) -> usize {
        use ::decimal::dec_16::*;
        let num = *self;
        if num < DEC_4 {
            if num < DEC_2 {
                if num < DEC_1 {
                    1
                } else {
                    2
                }
            } else {
                if num < DEC_3 {
                    3
                } else {
                    4
                }
            }
        } else {
            5
        }
    }

    #[inline]
    fn most_significant_bit(&self) -> usize {
        // We know that "2^i <= num < 2^(i+1)",
        // Where i is the highest set bit of the num.
        // Use ctlz and lookup tables to resolve number of
        // didgits with a single if statment.
        use ::decimal::dec_16::*;
        use ::decimal::BIN_TO_DEC;

        let i = 16 - (*self).leading_zeros() as usize;
        let l = BIN_TO_DEC[i];
        let h = BIN_TO_DEC[i + 1];
        if *self >= DEC[(h - 1) as usize] {
            h as usize
        } else {
            l as usize
        }
    }
}

impl NumDigits for u32 {
    #[inline]
    fn log(&self) -> usize {
        (*self as f64).log10() as u32 as usize + 1
    }

    #[inline]
    fn str_format(&self) -> usize {
        format!("{}", *self).len()
    }

    #[inline]
    fn str_format_stack(&self) -> usize {
        use std::io::{Write, Cursor};
        let mut b = [0u8; 10];
        let mut c = Cursor::new(&mut b[..]);
        write!(c, "{}", *self).unwrap();
        c.position() as usize
    }

    #[inline]
    fn str_itoa_stack(&self) -> usize {
        use std::io::Cursor;
        let mut b = [0u8; 10];
        let mut c = Cursor::new(&mut b[..]);
        itoa::write(&mut c, *self).unwrap();
        c.position() as usize
    }

    #[inline]
    fn div_loop(&self) -> usize {
        let mut len = 1;
        let mut num = *self;
        while num >= 10 {
            num /= 10;
            len += 1;
        }
        len
    }

    #[inline]
    fn div_unrolled(&self) -> usize {
        let mut len = 1;
        let mut num = *self;
        loop {
            if num < 10 {
                return len;
            }
            if num < 100 {
                return len + 1;
            }
            if num < 1000 {
                return len + 2;
            }
            if num < 10000 {
                return len + 3;
            }
            num /= 10000;
            len += 4;
        }
    }

    #[inline]
    fn mul_loop(&self) -> usize {
        let mut len = 1;
        let mut n = 1u32;
        while len < 10 {
            n *= 10;
            if n > *self {
                return len;
            }
            len += 1;
        }
        len
    }

    #[inline]
    fn cmp_list(&self) -> usize {
        use ::decimal::dec_32::*;
        if *self >= DEC_9 {
            10
        } else if *self >= DEC_8 {
            9
        } else if *self >= DEC_7 {
            8
        } else if *self >= DEC_6 {
            7
        } else if *self >= DEC_5 {
            6
        } else if *self >= DEC_4 {
            5
        } else if *self >= DEC_3 {
            4
        } else if *self >= DEC_2 {
            3
        } else if *self >= DEC_1 {
            2
        } else {
            1
        }
    }

    #[inline]
    fn pattern_match(&self) -> usize {
        use ::decimal::dec_32::*;
        use ::std::u32::MIN;
        match *self {
              MIN  .. DEC_1  => 1,
            DEC_1  .. DEC_2  => 2,
            DEC_2  .. DEC_3  => 3,
            DEC_3  .. DEC_4  => 4,
            DEC_4  .. DEC_5  => 5,
            DEC_5  .. DEC_6  => 6,
            DEC_6  .. DEC_7  => 7,
            DEC_7  .. DEC_8  => 8,
            DEC_8  .. DEC_9  => 9,
            _ => 10,
        }
    }

    #[inline]
    fn binary_search(&self) -> usize {
        use ::decimal::dec_32::*;
        let num = *self;
        if num < DEC_8 {
            if num < DEC_4 {
                if num < DEC_2 {
                    if num < DEC_1 {
                        1
                    } else {
                        2
                    }
                } else {
                    if num < DEC_3 {
                        3
                    } else {
                        4
                    }
                }
            } else {
                if num < DEC_6 {
                    if num < DEC_5 {
                        5
                    }
                    else {
                        6
                    }
                } else {
                    if num < DEC_7 {
                        7
                    } else {
                        8
                    }
                }
            }
        } else {
            if num < DEC_9 {
                9
            } else {
                10
            }
        }
    }

    #[inline]
    fn most_significant_bit(&self) -> usize {
        // We know that "2^i <= num < 2^(i+1)",
        // Where i is the highest set bit of the num.
        // Use ctlz and lookup tables to resolve number of
        // didgits with a single if statment.
        use ::decimal::dec_32::*;
        use ::decimal::BIN_TO_DEC;

        let i = 32 - (*self).leading_zeros() as usize;
        let l = BIN_TO_DEC[i];
        let h = BIN_TO_DEC[i + 1];
        if *self >= DEC[(h - 1) as usize] {
            h as usize
        } else {
            l as usize
        }
    }
}

impl NumDigits for u64 {
    #[inline]
    fn log(&self) -> usize {
        (*self as f64).log10() as u32 as usize + 1
    }

    #[inline]
    fn str_format(&self) -> usize {
        format!("{}", *self).len()
    }

    #[inline]
    fn str_format_stack(&self) -> usize {
        use std::io::{Write, Cursor};
        let mut b = [0u8; 20];
        let mut c = Cursor::new(&mut b[..]);
        write!(c, "{}", *self).unwrap();
        c.position() as usize
    }

    #[inline]
    fn str_itoa_stack(&self) -> usize {
        use std::io::Cursor;
        let mut b = [0u8; 20];
        let mut c = Cursor::new(&mut b[..]);
        itoa::write(&mut c, *self).unwrap();
        c.position() as usize
    }

    #[inline]
    fn div_loop(&self) -> usize {
        let mut len = 1;
        let mut num = *self;
        while num >= 10 {
            num /= 10;
            len += 1;
        }
        len
    }

    #[inline]
    fn div_unrolled(&self) -> usize {
        let mut len = 1;
        let mut num = *self;
        loop {
            if num < 10 {
                return len;
            }
            if num < 100 {
                return len + 1;
            }
            if num < 1000 {
                return len + 2;
            }
            if num < 10000 {
                return len + 3;
            }
            num /= 10000;
            len += 4;
        }
    }

    #[inline]
    fn mul_loop(&self) -> usize {
        let mut len = 1;
        let mut n = 1u64;
        while len < 20 {
            n *= 10;
            if n > *self {
                return len;
            }
            len += 1;
        }
        len
    }

    #[inline]
    fn cmp_list(&self) -> usize {
        use ::decimal::dec_64::*;
        if *self >= DEC_19 {
            20
        } else if *self >= DEC_18 {
            19
        } else if *self >= DEC_17 {
            18
        } else if *self >= DEC_16 {
            17
        } else if *self >= DEC_15 {
            16
        } else if *self >= DEC_14 {
            15
        } else if *self >= DEC_13 {
            14
        } else if *self >= DEC_12 {
            13
        } else if *self >= DEC_11 {
            12
        } else if *self >= DEC_10 {
            11
        } else if *self >= DEC_9 {
            10
        } else if *self >= DEC_8 {
            9
        } else if *self >= DEC_7 {
            8
        } else if *self >= DEC_6 {
            7
        } else if *self >= DEC_5 {
            6
        } else if *self >= DEC_4 {
            5
        } else if *self >= DEC_3 {
            4
        } else if *self >= DEC_2 {
            3
        } else if *self >= DEC_1 {
            2
        } else {
            1
        }
    }

    #[inline]
    fn pattern_match(&self) -> usize {
        use ::decimal::dec_64::*;
        use ::std::u64::MIN;
        match *self {
              MIN  .. DEC_1  => 1,
            DEC_1  .. DEC_2  => 2,
            DEC_2  .. DEC_3  => 3,
            DEC_3  .. DEC_4  => 4,
            DEC_4  .. DEC_5  => 5,
            DEC_5  .. DEC_6  => 6,
            DEC_6  .. DEC_7  => 7,
            DEC_7  .. DEC_8  => 8,
            DEC_8  .. DEC_9  => 9,
            DEC_9  .. DEC_10 => 10,
            DEC_10 .. DEC_11 => 11,
            DEC_11 .. DEC_12 => 12,
            DEC_12 .. DEC_13 => 13,
            DEC_13 .. DEC_14 => 14,
            DEC_14 .. DEC_15 => 15,
            DEC_15 .. DEC_16 => 16,
            DEC_16 .. DEC_17 => 17,
            DEC_17 .. DEC_18 => 18,
            DEC_18 .. DEC_19 => 19,
            _ => 20,
        }
    }

    #[inline]
    fn binary_search(&self) -> usize {
        use ::decimal::dec_64::*;
        let num = *self;
        if num < DEC_16 {
            if num < DEC_8 {
                if num < DEC_4 {
                    if num < DEC_2 {
                        if num < DEC_1 {
                            1
                        } else {
                            2
                        }
                    } else {
                        if num < DEC_3 {
                            3
                        } else {
                            4
                        }
                    }
                } else {
                    if num < DEC_6 {
                        if num < DEC_5 {
                            5
                        }
                        else {
                            6
                        }
                    } else {
                        if num < DEC_7 {
                            7
                        } else {
                            8
                        }
                    }
                }
            } else {
               if num < DEC_12 {
                    if num < DEC_10 {
                        if num < DEC_9 {
                            9
                        } else {
                            10
                        }
                    } else {
                        if num < DEC_11 {
                            11
                        } else {
                            12
                        }
                    }
                } else {
                    if num < DEC_14 {
                        if num < DEC_13 {
                            13
                        }
                        else {
                            14
                        }
                    } else {
                        if num < DEC_15 {
                            15
                        } else {
                            16
                        }
                    }
                }
            }
        } else {
            if num < DEC_18 {
                if num < DEC_17 {
                    17
                } else {
                    18
                }
            } else {
                if num < DEC_19 {
                    19
                } else {
                    20
                }
            }
        }
    }

    #[inline]
    fn most_significant_bit(&self) -> usize {
        // We know that "2^i <= num < 2^(i+1)",
        // Where i is the highest set bit of the num.
        // Use ctlz and lookup tables to resolve number of
        // didgits with a single if statment.
        use ::decimal::dec_64::*;
        use ::decimal::BIN_TO_DEC;

        let i = 64 - (*self).leading_zeros() as usize;
        let l = BIN_TO_DEC[i];
        let h = BIN_TO_DEC[i + 1];
        if *self >= DEC[(h - 1) as usize] {
            h as usize
        } else {
            l as usize
        }
    }
}

impl NumDigits for u128 {
    #[inline]
    fn log(&self) -> usize {
        (*self as f64).log10() as u32 as usize + 1
    }

    #[inline]
    fn str_format(&self) -> usize {
        format!("{}", *self).len()
    }

    #[inline]
    fn str_format_stack(&self) -> usize {
        use std::io::{Write, Cursor};
        let mut b = [0u8; 39];
        let mut c = Cursor::new(&mut b[..]);
        write!(c, "{}", *self).unwrap();
        c.position() as usize
    }

    #[inline]
    fn str_itoa_stack(&self) -> usize {
        unimplemented!();
    }

    #[inline]
    fn div_loop(&self) -> usize {
        let mut len = 1;
        let mut num = *self;
        while num >= 10 {
            num /= 10;
            len += 1;
        }
        len
    }

    #[inline]
    fn div_unrolled(&self) -> usize {
        let mut len = 1;
        let mut num = *self;
        loop {
            if num < 10 {
                return len;
            }
            if num < 100 {
                return len + 1;
            }
            if num < 1000 {
                return len + 2;
            }
            if num < 10000 {
                return len + 3;
            }
            num /= 10000;
            len += 4;
        }
    }

    #[inline]
    fn mul_loop(&self) -> usize {
        let mut len = 1;
        let mut n = 1u128;
        while len < 39 {
            n *= 10;
            if n > *self {
                return len;
            }
            len += 1;
        }
        len
    }

    #[inline]
    fn cmp_list(&self) -> usize {
        use ::decimal::dec_128::*;
        if *self >= DEC_38 {
            39
        } else if *self >= DEC_37 {
            38
        } else if *self >= DEC_36 {
            37
        } else if *self >= DEC_35 {
            36
        } else if *self >= DEC_34 {
            35
        } else if *self >= DEC_33 {
            34
        } else if *self >= DEC_32 {
            33
        } else if *self >= DEC_31 {
            32
        } else if *self >= DEC_30 {
            31
        } else if *self >= DEC_29 {
            30
        } else if *self >= DEC_28 {
            29
        } else if *self >= DEC_27 {
            28
        } else if *self >= DEC_26 {
            27
        } else if *self >= DEC_25 {
            26
        } else if *self >= DEC_24 {
            25
        } else if *self >= DEC_23 {
            24
        } else if *self >= DEC_22 {
            23
        } else if *self >= DEC_21 {
            22
        } else if *self >= DEC_20 {
            21
        } else if *self >= DEC_19 {
            20
        } else if *self >= DEC_18 {
            19
        } else if *self >= DEC_17 {
            18
        } else if *self >= DEC_16 {
            17
        } else if *self >= DEC_15 {
            16
        } else if *self >= DEC_14 {
            15
        } else if *self >= DEC_13 {
            14
        } else if *self >= DEC_12 {
            13
        } else if *self >= DEC_11 {
            12
        } else if *self >= DEC_10 {
            11
        } else if *self >= DEC_9 {
            10
        } else if *self >= DEC_8 {
            9
        } else if *self >= DEC_7 {
            8
        } else if *self >= DEC_6 {
            7
        } else if *self >= DEC_5 {
            6
        } else if *self >= DEC_4 {
            5
        } else if *self >= DEC_3 {
            4
        } else if *self >= DEC_2 {
            3
        } else if *self >= DEC_1 {
            2
        } else {
            1
        }
    }

    #[inline]
    fn pattern_match(&self) -> usize {
        use ::decimal::dec_128::*;
        use ::std::u128::MIN;
        match *self {
              MIN  .. DEC_1  => 1,
            DEC_1  .. DEC_2  => 2,
            DEC_2  .. DEC_3  => 3,
            DEC_3  .. DEC_4  => 4,
            DEC_4  .. DEC_5  => 5,
            DEC_5  .. DEC_6  => 6,
            DEC_6  .. DEC_7  => 7,
            DEC_7  .. DEC_8  => 8,
            DEC_8  .. DEC_9  => 9,
            DEC_9  .. DEC_10 => 10,
            DEC_10 .. DEC_11 => 11,
            DEC_11 .. DEC_12 => 12,
            DEC_12 .. DEC_13 => 13,
            DEC_13 .. DEC_14 => 14,
            DEC_14 .. DEC_15 => 15,
            DEC_15 .. DEC_16 => 16,
            DEC_16 .. DEC_17 => 17,
            DEC_17 .. DEC_18 => 18,
            DEC_18 .. DEC_19 => 19,
            DEC_19 .. DEC_20 => 20,
            DEC_20 .. DEC_21 => 21,
            DEC_21 .. DEC_22 => 22,
            DEC_22 .. DEC_23 => 23,
            DEC_23 .. DEC_24 => 24,
            DEC_24 .. DEC_25 => 25,
            DEC_25 .. DEC_26 => 26,
            DEC_26 .. DEC_27 => 27,
            DEC_27 .. DEC_28 => 28,
            DEC_28 .. DEC_29 => 29,
            DEC_29 .. DEC_30 => 30,
            DEC_30 .. DEC_31 => 31,
            DEC_31 .. DEC_32 => 32,
            DEC_32 .. DEC_33 => 33,
            DEC_33 .. DEC_34 => 34,
            DEC_34 .. DEC_35 => 35,
            DEC_35 .. DEC_36 => 36,
            DEC_36 .. DEC_37 => 37,
            DEC_37 .. DEC_38 => 38,
            _ => 39,
        }
    }

    #[inline]
    fn binary_search(&self) -> usize {
        use ::decimal::dec_128::*;
        let num = *self;
        if num < DEC_32 {
            if num < DEC_16 {
                if num < DEC_8 {
                    if num < DEC_4 {
                        if num < DEC_2 {
                            if num < DEC_1 {
                                1
                            } else {
                                2
                            }
                        } else {
                            if num < DEC_3 {
                                3
                            } else {
                                4
                            }
                        }
                    } else {
                        if num < DEC_6 {
                            if num < DEC_5 {
                                5
                            }
                            else {
                                6
                            }
                        } else {
                            if num < DEC_7 {
                                7
                            } else {
                                8
                            }
                        }
                    }
                } else {
                   if num < DEC_12 {
                        if num < DEC_10 {
                            if num < DEC_9 {
                                9
                            } else {
                                10
                            }
                        } else {
                            if num < DEC_11 {
                                11
                            } else {
                                12
                            }
                        }
                    } else {
                        if num < DEC_14 {
                            if num < DEC_13 {
                                13
                            }
                            else {
                                14
                            }
                        } else {
                            if num < DEC_15 {
                                15
                            } else {
                                16
                            }
                        }
                    }
                }
            } else {
                if num < DEC_24 {
                    if num < DEC_20 {
                        if num < DEC_18 {
                            if num < DEC_17 {
                                17
                            } else {
                                18
                            }
                        } else {
                            if num < DEC_19 {
                                19
                            } else {
                                20
                            }
                        }
                    } else {
                        if num < DEC_22 {
                            if num < DEC_21 {
                                21
                            }
                            else {
                                22
                            }
                        } else {
                            if num < DEC_23 {
                                23
                            } else {
                                24
                            }
                        }
                    }
                } else {
                   if num < DEC_28 {
                        if num < DEC_26 {
                            if num < DEC_25 {
                                25
                            } else {
                                26
                            }
                        } else {
                            if num < DEC_27 {
                                27
                            } else {
                                28
                            }
                        }
                    } else {
                        if num < DEC_30 {
                            if num < DEC_29 {
                                29
                            }
                            else {
                                30
                            }
                        } else {
                            if num < DEC_31 {
                                31
                            } else {
                                32
                            }
                        }
                    }
                }
            }
        } else {
            if num < DEC_36 {
                if num < DEC_34 {
                    if num < DEC_33 {
                        33
                    } else {
                        34
                    }
                } else {
                    if num < DEC_35 {
                        35
                    } else {
                        36
                    }
                }
            } else {
                if num < DEC_38 {
                    if num < DEC_37 {
                        37
                    }
                    else {
                        38
                    }
                } else {
                    39
                }
            }
        }
    }

    #[inline]
    fn most_significant_bit(&self) -> usize {
        // We know that "2^i <= num < 2^(i+1)",
        // Where i is the highest set bit of the num.
        // Use ctlz and lookup tables to resolve number of
        // didgits with a single if statment.
        use ::decimal::dec_128::*;
        use ::decimal::BIN_TO_DEC;

        let i = 128 - (*self).leading_zeros() as usize;
        let l = BIN_TO_DEC[i];
        let h = BIN_TO_DEC[i + 1];
        if *self >= DEC[(h - 1) as usize] {
            h as usize
        } else {
            l as usize
        }
    }
}
