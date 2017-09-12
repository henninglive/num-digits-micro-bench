pub trait NumDigits {
    fn str_format(&self) -> usize;
    fn str_format_stack(&self) -> usize;
    fn div_loop(&self) -> usize;
    fn div_unrolled(&self) -> usize;
    fn mul_loop(&self) -> usize;
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
}