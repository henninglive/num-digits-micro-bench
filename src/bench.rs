extern crate rand;
extern crate test;

macro_rules! bench_setup {
    (u128) => {
        fn bench_setup() -> Vec<u128> {
            use super::rand::Rng;
            let mut rand = super::rand::thread_rng();
            (0..BENCH_SIZE).map(|_| {
                (rand.gen::<u64>() as u128) << 64 &
                (rand.gen::<u64>() as u128)
            }).collect::<Vec<u128>>()
        }
    };
    ($t:ident) => {
        fn bench_setup() -> Vec<$t> {
            use super::rand::Rng;
            let mut rand = super::rand::thread_rng();
            (0..BENCH_SIZE)
                .map(|_| {rand.gen::<$t>()}).collect::<Vec<$t>>()
        }
    };
}

macro_rules! bench_methode {
    ($t:ident, zero_impl) => {
        #[bench]
        fn zero_impl(b: &mut Bencher) {
            let v = bench_setup();
            b.iter(|| {
                bench_helper(|n: $t| {black_box(n); 0}, &v[..])
            });
        }
    };
    ($t:ident, $name:ident) => {
        #[bench]
        fn $name(b: &mut Bencher) {
            let v = bench_setup();
            b.iter(|| bench_helper(|n: $t| n.$name(), &v[..]));
        }
    };
}

macro_rules! bench_type {
    ( $( $t:ident ),* ) => {
        $(
            mod $t {
                use super::test::{Bencher, black_box};
                use ::{Digits, DidgitsFmt, DidgitsItoa};

                const BENCH_SIZE: usize = 1_000;

                bench_setup!($t);

                fn bench_helper<N: Digits + Sized + Copy, F: Fn(N) -> usize>(f: F, b: &[N]){
                    for i in black_box(b).iter() {
                        black_box(f(*i));
                    }
                }

                bench_methode!($t, str_format);
                bench_methode!($t, str_format_stack);
                bench_methode!($t, str_itoa_stack);
                bench_methode!($t, log);
                bench_methode!($t, div_loop);
                bench_methode!($t, div_unrolled);
                bench_methode!($t, mul_loop);
                bench_methode!($t, cmp_list);
                bench_methode!($t, pattern_match);
                bench_methode!($t, binary_search);
                bench_methode!($t, most_significant_bit);
                bench_methode!($t, zero_impl);
            }
        )*
    };
}

bench_type!(u8, u16, u32, u64, u128);
