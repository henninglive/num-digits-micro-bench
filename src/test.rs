mod u8 {
    use ::num_digits::NumDigits;

    type Uint = u8;

    fn test_helper<F: Fn(u8) -> usize>(f: F) {
        assert_eq!(f(0), 1);
        assert_eq!(f(::std::u8::MAX), 3);
        for i in 0..3 {
            let n = 10u8.pow(i as u32);
            assert_eq!(f(n), i + 1);
            if i == 0 {
                assert_eq!(f(0), 1);
            } else {
                assert_eq!(f(n - 1), i);
                assert_eq!(f(n / 2), i);
            }
        }
    }

    #[test]
    fn test_log() {
        test_helper(|n: Uint| n.log());
    }

    #[test]
    fn test_str_format() {
        test_helper(|n: Uint| n.str_format());
    }

    #[test]
    fn test_str_format_stack() {
        test_helper(|n: Uint| n.str_format_stack());
    }

    #[test]
    fn test_str_itoa_stack() {
        test_helper(|n: Uint| n.str_itoa_stack());
    }

    #[test]
    fn test_div_loop() {
        test_helper(|n: Uint| n.div_loop());
    }

    #[test]
    fn test_div_unrolled() {
        test_helper(|n: Uint| n.div_unrolled());
    }

    #[test]
    fn test_mul() {
        test_helper(|n: Uint| n.mul_loop());
    }

    #[test]
    fn test_cmp_list() {
        test_helper(|n: Uint| n.cmp_list());
    }

    #[test]
    fn test_pattern_match() {
        test_helper(|n: Uint| n.pattern_match());
    }

    #[test]
    fn test_binary_search() {
        test_helper(|n: Uint| n.binary_search());
    }

    #[test]
    fn test_most_significant_bit() {
        test_helper(|n: Uint| n.most_significant_bit());
    }
}

mod u16 {
    use ::num_digits::NumDigits;

    type Uint = u16;

    fn test_helper<F: Fn(u16) -> usize>(f: F){
        assert_eq!(f(0), 1);
        assert_eq!(f(::std::u16::MAX), 5);
        for i in 0..5 {
            let n = 10u16.pow(i as u32);
            assert_eq!(f(n), i + 1);
            if i == 0 {
                assert_eq!(f(0), 1);
            } else {
                assert_eq!(f(n - 1), i);
                assert_eq!(f(n / 2), i);
            }
        }
    }

    #[test]
    fn test_log() {
        test_helper(|n: Uint| n.log());
    }

    #[test]
    fn test_str_format() {
        test_helper(|n: Uint| n.str_format());
    }

    #[test]
    fn test_str_format_stack() {
        test_helper(|n: Uint| n.str_format_stack());
    }

    #[test]
    fn test_str_itoa_stack() {
        test_helper(|n: Uint| n.str_itoa_stack());
    }

    #[test]
    fn test_div_loop() {
        test_helper(|n: Uint| n.div_loop());
    }

    #[test]
    fn test_div_unrolled() {
        test_helper(|n: Uint| n.div_unrolled());
    }

    #[test]
    fn test_mul() {
        test_helper(|n: Uint| n.mul_loop());
    }

    #[test]
    fn test_cmp_list() {
        test_helper(|n: Uint| n.cmp_list());
    }

    #[test]
    fn test_pattern_match() {
        test_helper(|n: Uint| n.pattern_match());
    }

    #[test]
    fn test_binary_search() {
        test_helper(|n: Uint| n.binary_search());
    }

    #[test]
    fn test_most_significant_bit() {
        test_helper(|n: Uint| n.most_significant_bit());
    }
}

mod u32 {
    use ::num_digits::NumDigits;

    type Uint = u32;

    fn test_helper<F: Fn(u32) -> usize>(f: F){
        assert_eq!(f(0), 1);
        assert_eq!(f(::std::u32::MAX), 10);
        for i in 0..10 {
            let n = 10u32.pow(i as u32);
            assert_eq!(f(n), i + 1);
            if i == 0 {
                assert_eq!(f(0), 1);
            } else {
                assert_eq!(f(n - 1), i);
                assert_eq!(f(n / 2), i);
            }
        }
    }

    #[test]
    fn test_log() {
        test_helper(|n: Uint| n.log());
    }

    #[test]
    fn test_str_format() {
        test_helper(|n: Uint| n.str_format());
    }

    #[test]
    fn test_str_format_stack() {
        test_helper(|n: Uint| n.str_format_stack());
    }

    #[test]
    fn test_str_itoa_stack() {
        test_helper(|n: Uint| n.str_itoa_stack());
    }

    #[test]
    fn test_div_loop() {
        test_helper(|n: Uint| n.div_loop());
    }

    #[test]
    fn test_div_unrolled() {
        test_helper(|n: Uint| n.div_unrolled());
    }

    #[test]
    fn test_mul() {
        test_helper(|n: Uint| n.mul_loop());
    }

    #[test]
    fn test_cmp_list() {
        test_helper(|n: Uint| n.cmp_list());
    }

    #[test]
    fn test_pattern_match() {
        test_helper(|n: Uint| n.pattern_match());
    }

    #[test]
    fn test_binary_search() {
        test_helper(|n: Uint| n.binary_search());
    }

    #[test]
    fn test_most_significant_bit() {
        test_helper(|n: Uint| n.most_significant_bit());
    }
}

mod u64 {
    use ::num_digits::NumDigits;

    type Uint = u64;

    fn test_helper<F: Fn(u64) -> usize>(f: F){
        assert_eq!(f(0), 1);
        assert_eq!(f(::std::u64::MAX), 20);
        for i in 0..20 {
            let n = 10u64.pow(i as u32);
            assert_eq!(f(n), i + 1);
            if i == 0 {
                assert_eq!(f(0), 1);
            } else {
                assert_eq!(f(n - 1), i);
                assert_eq!(f(n / 2), i);
            }
        }
    }

    #[test]
    fn test_log() {
        let f = |n: u64| n.log();
        assert_eq!(f(0), 1);
        assert_eq!(f(::std::u64::MAX), 20);
        for i in 0..20 {
            let n = 10u64.pow(i as u32);
            if i == 0 {
                assert_eq!(f(0), 1);
            } else if i < 15  {
                assert_eq!(f(n), i + 1);
                assert_eq!(f(n - 1), i);
                assert_eq!(f(n / 2), i);
            } else {
                //log10 is inaccurate for large numbers
                assert!((i <= f(n)) && (f(n) < ::std::cmp::max(i + 2, 20)));
            }
        }
    }

    #[test]
    fn test_str_format() {
        test_helper(|n: Uint| n.str_format());
    }

    #[test]
    fn test_str_format_stack() {
        test_helper(|n: Uint| n.str_format_stack());
    }

    #[test]
    fn test_str_itoa_stack() {
        test_helper(|n: Uint| n.str_itoa_stack());
    }

    #[test]
    fn test_div_loop() {
        test_helper(|n: Uint| n.div_loop());
    }

    #[test]
    fn test_div_unrolled() {
        test_helper(|n: Uint| n.div_unrolled());
    }

    #[test]
    fn test_mul() {
        test_helper(|n: Uint| n.mul_loop());
    }

    #[test]
    fn test_cmp_list() {
        test_helper(|n: Uint| n.cmp_list());
    }

    #[test]
    fn test_pattern_match() {
        test_helper(|n: Uint| n.pattern_match());
    }

    #[test]
    fn test_binary_search() {
        test_helper(|n: Uint| n.binary_search());
    }

    #[test]
    fn test_most_significant_bit() {
        test_helper(|n: Uint| n.most_significant_bit());
    }
}

mod u128 {
    use ::num_digits::NumDigits;

    type Uint = u128;

    fn test_helper<F: Fn(u128) -> usize>(f: F){
        assert_eq!(f(0), 1);
        assert_eq!(f(::std::u128::MAX), 39);
        for i in 0..39 {
            let n = 10u128.pow(i as u32);
            assert_eq!(f(n), i + 1);
            if i == 0 {
                assert_eq!(f(0), 1);
            } else {
                assert_eq!(f(n - 1), i);
                assert_eq!(f(n / 2), i);
            }
        }
    }

    #[test]
    fn test_log() {
        let f = |n: u128| n.log();
        assert_eq!(f(0), 1);
        assert_eq!(f(::std::u128::MAX), 39);
        for i in 0..39 {
            let n = 10u128.pow(i as u32);
            if i == 0 {
                assert_eq!(f(0), 1);
            } else if i < 15  {
                assert_eq!(f(n), i + 1);
                assert_eq!(f(n - 1), i);
                assert_eq!(f(n / 2), i);
            } else {
                //log10 is inaccurate for large numbers
                assert!((i <= f(n)) && (f(n) < ::std::cmp::max(i + 2, 39)));
            }
        }
    }

    #[test]
    fn test_str_format() {
        test_helper(|n: Uint| n.str_format());
    }

    #[test]
    fn test_str_format_stack() {
        test_helper(|n: Uint| n.str_format_stack());
    }

    #[test]
    #[should_panic]
    fn test_str_itoa_stack() {
        test_helper(|n: Uint| n.str_itoa_stack());
    }

    #[test]
    fn test_div() {
        test_helper(|n: Uint| n.div_loop());
    }

    #[test]
    fn test_div_unrolled() {
        test_helper(|n: Uint| n.div_unrolled());
    }

    #[test]
    fn test_mul() {
        test_helper(|n: Uint| n.mul_loop());
    }

    #[test]
    fn test_cmp_list() {
        test_helper(|n: Uint| n.cmp_list());
    }

    #[test]
    fn test_pattern_match() {
        test_helper(|n: Uint| n.pattern_match());
    }

    #[test]
    fn test_binary_search() {
        test_helper(|n: Uint| n.binary_search());
    }

    #[test]
    fn test_most_significant_bit() {
        test_helper(|n: Uint| n.most_significant_bit());
    }
}
