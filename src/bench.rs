extern crate rand;
extern crate test;

use self::test::{black_box, Bencher};
use ::num_digits::NumDigits;

const BENCH_SIZE: usize = 1_000;

trait RngVec: Sized + Copy {
    fn rand_vec() -> Vec<Self>;
}

impl RngVec for u8 {
     fn rand_vec() -> Vec<u8> {
        use self::rand::Rng;
        let mut rand = self::rand::thread_rng();
        (0..BENCH_SIZE).map(|_| {rand.gen::<u8>()}).collect::<Vec<u8>>()
     }
}

impl RngVec for u16 {
     fn rand_vec() -> Vec<u16> {
        use self::rand::Rng;
        let mut rand = self::rand::thread_rng();
        (0..BENCH_SIZE).map(|_| {rand.gen::<u16>()}).collect::<Vec<u16>>()
     }
}

impl RngVec for u32 {
     fn rand_vec() -> Vec<u32> {
        use self::rand::Rng;
        let mut rand = self::rand::thread_rng();
        (0..BENCH_SIZE).map(|_| {rand.gen::<u32>()}).collect::<Vec<u32>>()
     }
}

impl RngVec for u64 {
     fn rand_vec() -> Vec<u64> {
        use self::rand::Rng;
        let mut rand = self::rand::thread_rng();
        (0..BENCH_SIZE).map(|_| {rand.gen::<u64>()}).collect::<Vec<u64>>()
     }
}

impl RngVec for u128 {
     fn rand_vec() -> Vec<u128> {
        use self::rand::Rng;
        let mut rand = self::rand::thread_rng();
        (0..BENCH_SIZE).map(|_| {
            (rand.gen::<u64>() as u128) << 64 &
            (rand.gen::<u64>() as u128)
        }).collect::<Vec<u128>>()
     }
}

fn bench_helper<N: NumDigits + RngVec, F: Fn(N) -> usize>(f: F, b: &[N]){
    for i in black_box(b).iter() {
        black_box(f(*i));
    }
}

#[inline]
fn bench_log<N: NumDigits + RngVec>(b: &mut Bencher) {
    let v = RngVec::rand_vec();
    b.iter(|| bench_helper(|n: N| n.log(), &v[..]));
}

#[inline]
fn bench_str_format<N: NumDigits + RngVec>(b: &mut Bencher) {
    let v = RngVec::rand_vec();
    b.iter(|| bench_helper(|n: N| n.str_format(), &v[..]));
}

#[inline]
fn bench_str_format_stack<N: NumDigits + RngVec>(b: &mut Bencher) {
    let v = RngVec::rand_vec();
    b.iter(|| bench_helper(|n: N| n.str_format_stack(), &v[..]));
}

#[inline]
fn bench_str_itoa_stack<N: NumDigits + RngVec>(b: &mut Bencher) {
    let v = RngVec::rand_vec();
    b.iter(|| bench_helper(|n: N| n.str_itoa_stack(), &v[..]));
}

#[inline]
fn bench_div_loop<N: NumDigits + RngVec>(b: &mut Bencher) {
    let v = RngVec::rand_vec();
    b.iter(|| bench_helper(|n: N| n.div_loop(), &v[..]));
}

#[inline]
fn bench_div_unrolled<N: NumDigits + RngVec>(b: &mut Bencher) {
    let v = RngVec::rand_vec();
    b.iter(|| bench_helper(|n: N| n.div_unrolled(), &v[..]));
}

#[inline]
fn bench_mul_loop<N: NumDigits + RngVec>(b: &mut Bencher) {
    let v = RngVec::rand_vec();
    b.iter(|| bench_helper(|n: N| n.mul_loop(), &v[..]));
}

#[inline]
fn bench_cmp_list<N: NumDigits + RngVec>(b: &mut Bencher) {
    let v = RngVec::rand_vec();
    b.iter(|| bench_helper(|n: N| n.cmp_list(), &v[..]));
}

#[inline]
fn bench_pattern_match<N: NumDigits + RngVec>(b: &mut Bencher) {
    let v = RngVec::rand_vec();
    b.iter(|| bench_helper(|n: N| n.mul_loop(), &v[..]));
}

#[inline]
fn bench_binary_search<N: NumDigits + RngVec>(b: &mut Bencher) {
    let v = RngVec::rand_vec();
    b.iter(|| bench_helper(|n: N| n.binary_search(), &v[..]));
}

#[inline]
fn bench_most_significant_bit<N: NumDigits + RngVec>(b: &mut Bencher) {
    let v = RngVec::rand_vec();
    b.iter(|| bench_helper(|n: N| n.most_significant_bit(), &v[..]));
}

#[inline]
fn bench_zero_impl<N: NumDigits + RngVec>(b: &mut Bencher) {
    let v = RngVec::rand_vec();
    b.iter(|| bench_helper(|n: N| {black_box(n); 0}, &v[..]));
}

mod u8 {
    use super::test::Bencher;

    type Uint = u8;

    #[bench]
    fn bench_log(b: &mut Bencher) {
        super::bench_log::<Uint>(b);
    }

    #[bench]
    fn bench_str_format(b: &mut Bencher) {
        super::bench_str_format::<Uint>(b);
    }

    #[bench]
    fn bench_str_format_stack(b: &mut Bencher) {
        super::bench_str_format_stack::<Uint>(b);
    }

    #[bench]
    fn bench_str_itoa_stack(b: &mut Bencher) {
        super::bench_str_itoa_stack::<Uint>(b);
    }

    #[bench]
    fn bench_div_loop(b: &mut Bencher) {
        super::bench_div_loop::<Uint>(b);
    }

    #[bench]
    fn bench_div_unrolled(b: &mut Bencher) {
        super::bench_div_unrolled::<Uint>(b);
    }

    #[bench]
    fn bench_mul_loop(b: &mut Bencher) {
        super::bench_mul_loop::<Uint>(b);
    }

    #[bench]
    fn bench_cmp_list(b: &mut Bencher) {
        super::bench_cmp_list::<Uint>(b);
    }

    #[bench]
    fn bench_pattern_match(b: &mut Bencher) {
        super::bench_pattern_match::<Uint>(b);
    }

    #[bench]
    fn bench_binary_search(b: &mut Bencher) {
        super::bench_binary_search::<Uint>(b);
    }

    #[bench]
    fn bench_most_significant_bit(b: &mut Bencher) {
        super::bench_most_significant_bit::<Uint>(b);
    }

    #[bench]
    fn bench_zero_impl(b: &mut Bencher) {
        super::bench_zero_impl::<Uint>(b);
    }
}

mod u16 {
    use super::test::Bencher;

    type Uint = u16;

    #[bench]
    fn bench_log(b: &mut Bencher) {
        super::bench_log::<Uint>(b);
    }

    #[bench]
    fn bench_str_format(b: &mut Bencher) {
        super::bench_str_format::<Uint>(b);
    }

    #[bench]
    fn bench_str_format_stack(b: &mut Bencher) {
        super::bench_str_format_stack::<Uint>(b);
    }

    #[bench]
    fn bench_str_itoa_stack(b: &mut Bencher) {
        super::bench_str_itoa_stack::<Uint>(b);
    }

    #[bench]
    fn bench_div_loop(b: &mut Bencher) {
        super::bench_div_loop::<Uint>(b);
    }

    #[bench]
    fn bench_div_unrolled(b: &mut Bencher) {
        super::bench_div_unrolled::<Uint>(b);
    }

    #[bench]
    fn bench_mul_loop(b: &mut Bencher) {
        super::bench_mul_loop::<Uint>(b);
    }

    #[bench]
    fn bench_cmp_list(b: &mut Bencher) {
        super::bench_cmp_list::<Uint>(b);
    }

    #[bench]
    fn bench_pattern_match(b: &mut Bencher) {
        super::bench_pattern_match::<Uint>(b);
    }

    #[bench]
    fn bench_binary_search(b: &mut Bencher) {
        super::bench_binary_search::<Uint>(b);
    }

    #[bench]
    fn bench_most_significant_bit(b: &mut Bencher) {
        super::bench_most_significant_bit::<Uint>(b);
    }

    #[bench]
    fn bench_zero_impl(b: &mut Bencher) {
        super::bench_zero_impl::<Uint>(b);
    }
}

mod u32 {
    use super::test::Bencher;

    type Uint = u32;

    #[bench]
    fn bench_log(b: &mut Bencher) {
        super::bench_log::<Uint>(b);
    }

    #[bench]
    fn bench_str_format(b: &mut Bencher) {
        super::bench_str_format::<Uint>(b);
    }

    #[bench]
    fn bench_str_format_stack(b: &mut Bencher) {
        super::bench_str_format_stack::<Uint>(b);
    }

    #[bench]
    fn bench_str_itoa_stack(b: &mut Bencher) {
        super::bench_str_itoa_stack::<Uint>(b);
    }

    #[bench]
    fn bench_div_loop(b: &mut Bencher) {
        super::bench_div_loop::<Uint>(b);
    }

    #[bench]
    fn bench_div_unrolled(b: &mut Bencher) {
        super::bench_div_unrolled::<Uint>(b);
    }

    #[bench]
    fn bench_mul_loop(b: &mut Bencher) {
        super::bench_mul_loop::<Uint>(b);
    }

    #[bench]
    fn bench_cmp_list(b: &mut Bencher) {
        super::bench_cmp_list::<Uint>(b);
    }

    #[bench]
    fn bench_pattern_match(b: &mut Bencher) {
        super::bench_pattern_match::<Uint>(b);
    }

    #[bench]
    fn bench_binary_search(b: &mut Bencher) {
        super::bench_binary_search::<Uint>(b);
    }

    #[bench]
    fn bench_most_significant_bit(b: &mut Bencher) {
        super::bench_most_significant_bit::<Uint>(b);
    }

    #[bench]
    fn bench_zero_impl(b: &mut Bencher) {
        super::bench_zero_impl::<Uint>(b);
    }
}

mod u64 {
    use super::test::Bencher;

    type Uint = u64;

    #[bench]
    fn bench_log(b: &mut Bencher) {
        super::bench_log::<Uint>(b);
    }

    #[bench]
    fn bench_str_format(b: &mut Bencher) {
        super::bench_str_format::<Uint>(b);
    }

    #[bench]
    fn bench_str_format_stack(b: &mut Bencher) {
        super::bench_str_format_stack::<Uint>(b);
    }

    #[bench]
    fn bench_str_itoa_stack(b: &mut Bencher) {
        super::bench_str_itoa_stack::<Uint>(b);
    }

    #[bench]
    fn bench_div_loop(b: &mut Bencher) {
        super::bench_div_loop::<Uint>(b);
    }

    #[bench]
    fn bench_div_unrolled(b: &mut Bencher) {
        super::bench_div_unrolled::<Uint>(b);
    }

    #[bench]
    fn bench_mul_loop(b: &mut Bencher) {
        super::bench_mul_loop::<Uint>(b);
    }

    #[bench]
    fn bench_cmp_list(b: &mut Bencher) {
        super::bench_cmp_list::<Uint>(b);
    }

    #[bench]
    fn bench_pattern_match(b: &mut Bencher) {
        super::bench_pattern_match::<Uint>(b);
    }

    #[bench]
    fn bench_binary_search(b: &mut Bencher) {
        super::bench_binary_search::<Uint>(b);
    }

    #[bench]
    fn bench_most_significant_bit(b: &mut Bencher) {
        super::bench_most_significant_bit::<Uint>(b);
    }

    #[bench]
    fn bench_zero_impl(b: &mut Bencher) {
        super::bench_zero_impl::<Uint>(b);
    }
}

mod u128 {
    use super::test::Bencher;

    type Uint = u128;

    #[bench]
    fn bench_log(b: &mut Bencher) {
        super::bench_log::<Uint>(b);
    }

    #[bench]
    fn bench_str_format(b: &mut Bencher) {
        super::bench_str_format::<Uint>(b);
    }

    #[bench]
    fn bench_str_format_stack(b: &mut Bencher) {
        super::bench_str_format_stack::<Uint>(b);
    }

    #[bench]
    #[ignore]
    fn bench_str_itoa_stack(b: &mut Bencher) {
        super::bench_str_itoa_stack::<Uint>(b);
    }

    #[bench]
    fn bench_div_loop(b: &mut Bencher) {
        super::bench_div_loop::<Uint>(b);
    }

    #[bench]
    fn bench_div_unrolled(b: &mut Bencher) {
        super::bench_div_unrolled::<Uint>(b);
    }

    #[bench]
    fn bench_mul_loop(b: &mut Bencher) {
        super::bench_mul_loop::<Uint>(b);
    }

    #[bench]
    fn bench_cmp_list(b: &mut Bencher) {
        super::bench_cmp_list::<Uint>(b);
    }

    #[bench]
    fn bench_pattern_match(b: &mut Bencher) {
        super::bench_pattern_match::<Uint>(b);
    }

    #[bench]
    fn bench_binary_search(b: &mut Bencher) {
        super::bench_binary_search::<Uint>(b);
    }

    #[bench]
    fn bench_most_significant_bit(b: &mut Bencher) {
        super::bench_most_significant_bit::<Uint>(b);
    }

    #[bench]
    fn bench_zero_impl(b: &mut Bencher) {
        super::bench_zero_impl::<Uint>(b);
    }
}
