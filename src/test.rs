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
    fn test_div_loop() {
        test_helper(|n: u8| n.div_loop());
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
    fn test_div_loop() {
        test_helper(|n: u16| n.div_loop());
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
    fn test_div_loop() {
        test_helper(|n: u32| n.div_loop());
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
    fn test_div_loop() {
        test_helper(|n: u64| n.div_loop());
    }
}