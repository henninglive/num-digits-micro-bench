pub trait NumDigits {
    fn str_format(&self) -> usize;
    fn str_format_stack(&self) -> usize;
    fn div_loop(&self) -> usize;
    fn div_unrolled(&self) -> usize;
    fn mul_loop(&self) -> usize;
    fn pattern_match(&self) -> usize;
}

impl NumDigits for u8 {
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
    fn pattern_match(&self) -> usize {
        use ::decimal::dec_8::*;
        match *self {
            DEC_0 .. DEC_1 => 1,
            DEC_1 .. DEC_2 => 2,
            _ => 3,
        }
    }
}

impl NumDigits for u16 {
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
    fn pattern_match(&self) -> usize {
        use ::decimal::dec_16::*;
        match *self {
            DEC_0 .. DEC_1 => 1,
            DEC_1 .. DEC_2 => 2,
            DEC_2 .. DEC_3 => 3,
            DEC_3 .. DEC_4 => 4,
            _ => 5,
        }
    }
}

impl NumDigits for u32 {
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
    fn pattern_match(&self) -> usize {
        use ::decimal::dec_32::*;
        match *self {
            DEC_0  .. DEC_1  => 1,
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
}

impl NumDigits for u64 {
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
    fn pattern_match(&self) -> usize {
        use ::decimal::dec_64::*;
        match *self {
            DEC_0  .. DEC_1  => 1,
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
}