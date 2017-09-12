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

fn bench_helper<N: NumDigits + RngVec, F: Fn(N) -> usize>(f: F, b: &[N]){
    for i in black_box(b).iter() {
        black_box(f(*i));
    }
}

#[inline]
fn bench_div_loop<N: NumDigits + RngVec>(b: &mut Bencher) {
    let v = RngVec::rand_vec();
    b.iter(|| bench_helper(|n: N| n.div_loop(), &v[..]));
}

#[inline]
fn bench_mul_loop<N: NumDigits + RngVec>(b: &mut Bencher) {
    let v = RngVec::rand_vec();
    b.iter(|| bench_helper(|n: N| n.mul_loop(), &v[..]));
}

#[inline]
fn bench_zero_impl<N: NumDigits + RngVec>(b: &mut Bencher) {
    let v = RngVec::rand_vec();
    b.iter(|| bench_helper(|n: N| {black_box(n); 0}, &v[..]));
}

mod u8 {
    use super::test::Bencher;

    #[bench]
    fn bench_div_loop(b: &mut Bencher) {
        super::bench_div_loop::<u8>(b);
    }

    #[bench]
    fn bench_mul_loop(b: &mut Bencher) {
        super::bench_mul_loop::<u8>(b);
    }

    #[bench]
    fn bench_zero_impl(b: &mut Bencher) {
        super::bench_zero_impl::<u8>(b);
    }
}

mod u16 {
    use super::test::Bencher;

    #[bench]
    fn bench_div_loop(b: &mut Bencher) {
        super::bench_div_loop::<u16>(b);
    }

    #[bench]
    fn bench_mul_loop(b: &mut Bencher) {
        super::bench_mul_loop::<u16>(b);
    }

    #[bench]
    fn bench_zero_impl(b: &mut Bencher) {
        super::bench_zero_impl::<u16>(b);
    }
}

mod u32 {
    use super::test::Bencher;

    #[bench]
    fn bench_div_loop(b: &mut Bencher) {
        super::bench_div_loop::<u32>(b);
    }

    #[bench]
    fn bench_mul_loop(b: &mut Bencher) {
        super::bench_mul_loop::<u32>(b);
    }

    #[bench]
    fn bench_zero_impl(b: &mut Bencher) {
        super::bench_zero_impl::<u32>(b);
    }
}

mod u64 {
    use super::test::Bencher;

    #[bench]
    fn bench_div_loop(b: &mut Bencher) {
        super::bench_div_loop::<u64>(b);
    }

    #[bench]
    fn bench_mul_loop(b: &mut Bencher) {
        super::bench_mul_loop::<u64>(b);
    }

    #[bench]
    fn bench_zero_impl(b: &mut Bencher) {
        super::bench_zero_impl::<u64>(b);
    }
}
