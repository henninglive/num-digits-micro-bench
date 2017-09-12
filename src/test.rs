mod u8 {
    use ::num_digits::NumDigits;

    fn test_helper<F: Fn(u8) -> usize>(f: F) {
        assert_eq!(f(0), 1);
        assert_eq!(f(::std::u8::MAX), 3);

        let mut n = 1;
        assert_eq!(f(n), 1);
        for i in 1..3 {
            n *= 10;
            assert_eq!(f(n), i + 1);
            assert_eq!(f(n - 1), i);
            assert_eq!(f(n / 2), i);
        }
    }

    #[test]
    fn test_log() {
        test_helper(|n: u8| n.log());
    }

    #[test]
    fn test_str_format() {
        test_helper(|n: u8| n.str_format());
    }

    #[test]
    fn test_str_format_stack() {
        test_helper(|n: u8| n.str_format_stack());
    }

    #[test]
    fn test_str_itoa_stack() {
        test_helper(|n: u8| n.str_itoa_stack());
    }

    #[test]
    fn test_div_loop() {
        test_helper(|n: u8| n.div_loop());
    }

    #[test]
    fn test_div_unrolled() {
        test_helper(|n: u8| n.div_unrolled());
    }

    #[test]
    fn test_mul() {
        test_helper(|n: u8| n.mul_loop());
    }

    #[test]
    fn test_pattern_match() {
        test_helper(|n: u8| n.pattern_match());
    }

    #[test]
    fn test_binary_search() {
        test_helper(|n: u8| n.binary_search());
    }

    #[test]
    fn test_most_significant_bit() {
        test_helper(|n: u8| n.most_significant_bit());
    }
}

mod u16 {
    use ::num_digits::NumDigits;

    fn test_helper<F: Fn(u16) -> usize>(f: F){
        assert_eq!(f(0), 1);
        assert_eq!(f(::std::u16::MAX), 5);

        let mut n = 1;
        assert_eq!(f(n), 1);
        for i in 1..5 {
            n *= 10;
            assert_eq!(f(n), i + 1);
            assert_eq!(f(n - 1), i);
            assert_eq!(f(n / 2), i);
        }
    }

    #[test]
    fn test_log() {
        test_helper(|n: u16| n.log());
    }

    #[test]
    fn test_str_format() {
        test_helper(|n: u16| n.str_format());
    }

    #[test]
    fn test_str_format_stack() {
        test_helper(|n: u16| n.str_format_stack());
    }

    #[test]
    fn test_str_itoa_stack() {
        test_helper(|n: u16| n.str_itoa_stack());
    }

    #[test]
    fn test_div_loop() {
        test_helper(|n: u16| n.div_loop());
    }

    #[test]
    fn test_div_unrolled() {
        test_helper(|n: u16| n.div_unrolled());
    }

    #[test]
    fn test_mul() {
        test_helper(|n: u16| n.mul_loop());
    }

    #[test]
    fn test_pattern_match() {
        test_helper(|n: u16| n.pattern_match());
    }

    #[test]
    fn test_binary_search() {
        test_helper(|n: u16| n.binary_search());
    }

    #[test]
    fn test_most_significant_bit() {
        test_helper(|n: u16| n.most_significant_bit());
    }
}

mod u32 {
    use ::num_digits::NumDigits;

    fn test_helper<F: Fn(u32) -> usize>(f: F){
        assert_eq!(f(0), 1);
        assert_eq!(f(::std::u32::MAX), 10);

        let mut n = 1;
        assert_eq!(f(n), 1);
        for i in 1..10 {
            n *= 10;
            assert_eq!(f(n), i + 1);
            assert_eq!(f(n - 1), i);
            assert_eq!(f(n / 2), i);
        }
    }

    #[test]
    fn test_log() {
        test_helper(|n: u32| n.log());
    }

    #[test]
    fn test_str_format() {
        test_helper(|n: u32| n.str_format());
    }

    #[test]
    fn test_str_format_stack() {
        test_helper(|n: u32| n.str_format_stack());
    }

    #[test]
    fn test_str_itoa_stack() {
        test_helper(|n: u32| n.str_itoa_stack());
    }

    #[test]
    fn test_div_loop() {
        test_helper(|n: u32| n.div_loop());
    }

    #[test]
    fn test_div_unrolled() {
        test_helper(|n: u32| n.div_unrolled());
    }

    #[test]
    fn test_mul() {
        test_helper(|n: u32| n.mul_loop());
    }

    #[test]
    fn test_pattern_match() {
        test_helper(|n: u32| n.pattern_match());
    }

    #[test]
    fn test_binary_search() {
        test_helper(|n: u32| n.binary_search());
    }

    #[test]
    fn test_most_significant_bit() {
        test_helper(|n: u32| n.most_significant_bit());
    }
}

mod u64 {
    use ::num_digits::NumDigits;

    fn test_helper<F: Fn(u64) -> usize>(f: F){
        assert_eq!(f(0), 1);
        assert_eq!(f(::std::u64::MAX), 20);

        let mut n = 1;
        assert_eq!(f(n), 1);
        for i in 1..20 {
            n *= 10;
            assert_eq!(f(n), i + 1);
            assert_eq!(f(n - 1), i);
            assert_eq!(f(n / 2), i);
        }
    }

    #[test]
    fn test_log() {
        use ::decimal::dec_64::*;
        let f = |n: u64| n.log();

        assert_eq!(f(DEC_0), 1);
        assert_eq!(f(DEC_1), 2);
        assert_eq!(f(DEC_2), 3);
        assert_eq!(f(DEC_3), 4);
        assert_eq!(f(DEC_4), 5);
        assert_eq!(f(DEC_5), 6);
        assert_eq!(f(DEC_6), 7);
        assert_eq!(f(DEC_7), 8);
        assert_eq!(f(DEC_8), 9);
        assert_eq!(f(DEC_9), 10);
        assert_eq!(f(DEC_10), 11);
        assert_eq!(f(DEC_11), 12);
        assert_eq!(f(DEC_12), 13);
        assert_eq!(f(DEC_13), 14);
        assert_eq!(f(DEC_14), 15);

        //log10 is inaccurate for large numbers
        assert!((15 <= f(DEC_15)) && (f(DEC_15) <= 17));
        assert!((16 <= f(DEC_16)) && (f(DEC_16) <= 18));
        assert!((17 <= f(DEC_17)) && (f(DEC_17) <= 19));
        assert!((18 <= f(DEC_18)) && (f(DEC_18) <= 20));
        assert!((19 <= f(DEC_19)) && (f(DEC_19) <= 20));

        assert_eq!(f(::std::u64::MAX), 20);
    }

    #[test]
    fn test_str_format() {
        test_helper(|n: u64| n.str_format());
    }

    #[test]
    fn test_str_format_stack() {
        test_helper(|n: u64| n.str_format_stack());
    }

    #[test]
    fn test_str_itoa_stack() {
        test_helper(|n: u64| n.str_itoa_stack());
    }

    #[test]
    fn test_div_loop() {
        test_helper(|n: u64| n.div_loop());
    }

    #[test]
    fn test_div_unrolled() {
        test_helper(|n: u64| n.div_unrolled());
    }

    #[test]
    fn test_mul() {
        test_helper(|n: u64| n.mul_loop());
    }

    #[test]
    fn test_pattern_match() {
        test_helper(|n: u64| n.pattern_match());
    }

    #[test]
    fn test_binary_search() {
        test_helper(|n: u64| n.binary_search());
    }

    #[test]
    fn test_most_significant_bit() {
        test_helper(|n: u64| n.most_significant_bit());
    }
}
