mod u8 {
    use ::num_digits::NumDigits;

    type Uint = u8;

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
        use ::decimal::u64::*;
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

        let mut n = 1;
        assert_eq!(f(n), 1);
        for i in 1..39 {
            n *= 10;
            assert_eq!(f(n), i + 1);
            assert_eq!(f(n - 1), i);
            assert_eq!(f(n / 2), i);
        }
    }

    #[test]
    fn test_log() {
        use ::decimal::u128::*;
        let f = |n: u128| n.log();

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
        assert!((19 <= f(DEC_19)) && (f(DEC_19) <= 21));
        assert!((20 <= f(DEC_20)) && (f(DEC_20) <= 22));
        assert!((21 <= f(DEC_21)) && (f(DEC_21) <= 23));
        assert!((22 <= f(DEC_22)) && (f(DEC_22) <= 24));
        assert!((23 <= f(DEC_23)) && (f(DEC_23) <= 25));
        assert!((24 <= f(DEC_24)) && (f(DEC_24) <= 26));
        assert!((25 <= f(DEC_25)) && (f(DEC_25) <= 27));
        assert!((26 <= f(DEC_26)) && (f(DEC_26) <= 28));
        assert!((27 <= f(DEC_27)) && (f(DEC_27) <= 29));
        assert!((28 <= f(DEC_28)) && (f(DEC_28) <= 30));
        assert!((29 <= f(DEC_29)) && (f(DEC_29) <= 31));
        assert!((30 <= f(DEC_30)) && (f(DEC_30) <= 32));
        assert!((31 <= f(DEC_31)) && (f(DEC_31) <= 33));
        assert!((32 <= f(DEC_32)) && (f(DEC_32) <= 34));
        assert!((33 <= f(DEC_33)) && (f(DEC_33) <= 35));
        assert!((34 <= f(DEC_34)) && (f(DEC_34) <= 36));
        assert!((35 <= f(DEC_35)) && (f(DEC_35) <= 37));
        assert!((36 <= f(DEC_36)) && (f(DEC_36) <= 38));
        assert!((37 <= f(DEC_37)) && (f(DEC_37) <= 39));
        assert!((38 <= f(DEC_38)) && (f(DEC_38) <= 39));
        assert_eq!(f(::std::u128::MAX), 39);
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
