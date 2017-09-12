pub trait NumDigits {
    fn div_loop(&self) -> usize;
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
}