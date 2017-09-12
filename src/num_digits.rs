pub trait NumDigits {
    fn div_loop(&self) -> usize;
    fn mul_loop(&self) -> usize;
}

impl NumDigits for u8 {
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