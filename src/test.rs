macro_rules! test_methode {
    (u64, log) => {
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
    };
    (u128, log) => {
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
    };
    ($t:ident, $name:ident) => {
        #[test]
        fn $name() {
            test_helper(|n: $t| n.$name());
        }
    };
}

macro_rules! test_type {
    ( $( $t:ident($max:expr) ),* ) => {
        $(
            mod $t {
                use ::{Digits, DidgitsFmt, DidgitsItoa};

                fn test_helper<F: Fn($t) -> usize>(f: F) {
                    assert_eq!(f(0), 1);
                    assert_eq!(f(::std::$t::MAX), $max);
                    for i in 0..$max {
                        let n = (10 as $t).pow(i as u32);
                        assert_eq!(f(n), i + 1);
                        if i == 0 {
                            assert_eq!(f(0), 1);
                        } else {
                            assert_eq!(f(n - 1), i);
                            assert_eq!(f(n / 2), i);
                        }
                    }
                }

                test_methode!($t, str_format);
                test_methode!($t, str_format_stack);
                test_methode!($t, str_itoa_stack);
                test_methode!($t, log);
                test_methode!($t, div_loop);
                test_methode!($t, div_unrolled);
                test_methode!($t, mul_loop);
                test_methode!($t, cmp_list);
                test_methode!($t, pattern_match);
                test_methode!($t, binary_search);
                test_methode!($t, most_significant_bit);
            }
        )*
    };
}

test_type!(u8(3), u16(5), u32(10), u64(20), u128(39));
