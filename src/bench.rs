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

macro_rules! bench {
    ( $( $t:ident ),* ) => {
        $(
            mod $t {
                use super::test::{Bencher, black_box};
                use ::num_digits::NumDigits;

                const BENCH_SIZE: usize = 1_000;

                bench_setup!($t);

                fn bench_helper<N: NumDigits + Sized + Copy, F: Fn(N) -> usize>(f: F, b: &[N]){
                    for i in black_box(b).iter() {
                        black_box(f(*i));
                    }
                }

                #[bench]
                fn bench_log(b: &mut Bencher) {
                    let v = bench_setup();
                    b.iter(|| bench_helper(|n: $t| n.log(), &v[..]));
                }

                #[bench]
                fn bench_str_format(b: &mut Bencher) {
                    let v = bench_setup();
                    b.iter(|| bench_helper(|n: $t| n.str_format(), &v[..]));
                }

                #[bench]
                fn bench_str_format_stack(b: &mut Bencher) {
                    let v = bench_setup();
                    b.iter(|| bench_helper(|n: $t| n.str_format_stack(), &v[..]));
                }

                #[bench]
                fn bench_str_itoa_stack(b: &mut Bencher) {
                    let v = bench_setup();
                    b.iter(|| bench_helper(|n: $t| n.str_itoa_stack(), &v[..]));
                }

                #[bench]
                fn bench_div_loop(b: &mut Bencher) {
                    let v = bench_setup();
                    b.iter(|| bench_helper(|n: $t| n.div_loop(), &v[..]));
                }

                #[bench]
                fn bench_div_unrolled(b: &mut Bencher) {
                    let v = bench_setup();
                    b.iter(|| bench_helper(|n: $t| n.div_unrolled(), &v[..]));
                }

                #[bench]
                fn bench_mul_loop(b: &mut Bencher) {
                    let v = bench_setup();
                    b.iter(|| bench_helper(|n: $t| n.mul_loop(), &v[..]));
                }

                #[bench]
                fn bench_cmp_list(b: &mut Bencher) {
                    let v = bench_setup();
                    b.iter(|| bench_helper(|n: $t| n.cmp_list(), &v[..]));
                }

                #[bench]
                fn bench_pattern_match(b: &mut Bencher) {
                    let v = bench_setup();
                    b.iter(|| bench_helper(|n: $t| n.pattern_match(), &v[..]));
                }

                #[bench]
                fn bench_binary_search(b: &mut Bencher) {
                    let v = bench_setup();
                    b.iter(|| bench_helper(|n: $t| n.binary_search(), &v[..]));
                }

                #[bench]
                fn bench_most_significant_bit(b: &mut Bencher) {
                    let v = bench_setup();
                    b.iter(|| bench_helper(|n: $t| n.most_significant_bit(), &v[..]));
                }

                #[bench]
                fn bench_zero_impl(b: &mut Bencher) {
                    let v = bench_setup();
                    b.iter(|| bench_helper(|n: $t| {black_box(n); 0}, &v[..]));
                }
            }
        )*
    };
}

bench!(u8, u16, u32, u64, u128);
