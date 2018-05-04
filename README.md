# num-digits-micro-bench
Rust micro benchmark of different algorithms for finding the number of decimal digits of integers of various sizes.
The results are not very meaningful, I mostly created this to test and learn `cargo bench` and rust macros.

## Results

| Algorithm                                     | u8(ns)               | u16(ns)              | u32(ns)              | u64(ns)              | u128(ns)            |
| --------------------------------------------- | -------------------- | -------------------- | -------------------- | -------------------- | ------------------- |
| [div_loop](#div_loop)                         | 3,632 (+/- 388)      | 5,111 (+/- 626)      | 12,036 (+/- 4,035)   | 33,732 (+/- 2,946)   | 2,440 (+/- 563)     |
| [div_unrolled](#div_unrolled)                 | 1,238 (+/- 156)      | 1,929 (+/- 186)      | 5,693 (+/- 1,689)    | 11,512 (+/- 1,080)   | 1,187 (+/- 114)     |
| [mul_loop](#mul_loop)                         | 1,262 (+/- 250)      | 1,861 (+/- 146)      | 8,957 (+/- 767)      | 17,628 (+/- 2,019)   | 1,180 (+/- 115)     |
| [binary_search](#binary_search)               | 1,165 (+/- 106)      | 2,600 (+/- 233)      | 1,343 (+/- 127)      | 2,643 (+/- 245)      | 3,766 (+/- 1,459)   |
| [pattern_match](#pattern_match)               | 1,252 (+/- 119)      | 1,761 (+/- 169)      | 8,316 (+/- 324)      | 16,233 (+/- 641)     | 1,178 (+/- 77)      |
| [cmp_list](#cmp_list)                         | 1,685 (+/- 149)      | 2,138 (+/- 607)      | 2,314 (+/- 412)      | 2,113 (+/- 213)      | 39,225 (+/- 3,145)  |
| [most_significant_bit](#most_significant_bit) | 3,834 (+/- 745)      | 5,420 (+/- 697)      | 5,368 (+/- 845)      | 5,484 (+/- 890)      | 6,495 (+/- 2,633)   |
| [log](#log)                                   | 22,575 (+/- 7,807)   | 22,980 (+/- 4,983)   | 20,825 (+/- 9,213)   | 21,194 (+/- 1,976)   | 51,657 (+/- 5,479)  |
| [str_format](#str_format)                     | 125,692 (+/- 66,182) | 122,269 (+/- 45,914) | 125,254 (+/- 10,093) | 139,665 (+/- 47,381) | 113,735 (+/- 9,299) |
| [str_format_stack](#str_format_stack)         | 64,424 (+/- 32,997)  | 62,855 (+/- 41,842)  | 68,488 (+/- 7,809)   | 80,726 (+/- 10,432)  | 56,130 (+/- 4,633)  |
| [str_itoa_stack](#str_itoa_stack)             | 26,174 (+/- 12,323)  | 25,838 (+/- 10,821)  | 23,846 (+/- 2,852)   | 38,519 (+/- 4,106)   | 19,233 (+/- 10,256) |

Benchmarked on an Intel® Core™ i5-3337U Processor, an old laptop cpu. Using rust `nightly-x86_64-pc-windows-gnu` with `rustc 1.27.0-nightly`.

## Build and benchmark

1. Ensure you have the nightly version of cargo and Rust installed
2. Clone the project `$ git clone https://github.com/henninglive/num-digits-micro-bench && cd num-digits-micro-bench`
3. Build then run benchmarks `$ cargo bench`

## Algorithms

### pow10-macro
Integer literal power of 10, generated recursively

```rust
macro_rules! pow10 {
    (0)  => (1);
    (1)  => (10);
    (2)  => (100);
    (3)  => (1_000);
    (4)  => (10_000);
    (5)  => (100_000);
    (6)  => (1_000_000);
    (7)  => (10_000_000);
    (8)  => (100_000_000);
    (9)  => (1_000_000_000);
    (10) => (10_000_000_000);
    (11) => (100_000_000_000);
    (12) => (1_000_000_000_000);
    (13) => (pow10!{12} * 10);
    (14) => (pow10!{12} * 100);
    (15) => (pow10!{12} * 1_000);
    (16) => (pow10!{15} * 10);
    (17) => (pow10!{15} * 100);
    (18) => (pow10!{15} * 1_000);
    (19) => (pow10!{18} * 10);
    ...
}
```

### div_loop
Naive implementation, divide by 10 and count.

```rust
fn div_loop(mut n: u64) -> usize {
    let mut len = 1;
    while n >= 10 {
        n /= 10;
        len += 1;
    }
    len
}
```

### div_unrolled
Divide by 10, loop unrolled.

```rust
fn div_unrolled(mut n: u64) -> usize {
    let mut len = 1;
    loop {
        if n < 10 {
            return len;
        }
        if n < 100 {
            return len + 1;
        }
        if n < 1000 {
            return len + 2;
        }
        if n < 10000 {
            return len + 3;
        }
        n /= 10000;
        len += 4;
    }
}
```

### mul_loop
Multiplication is usually faster than division.

```rust
fn mul_loop(n: u64) -> usize {
    let mut len = 1;
    let mut m = 1u64;
    while len < 20 {
        m *= 10;
        if m > n {
            return len;
        }
        len += 1;
    }
    len
}
```

### binary_search
Map into a range with binary search. Powers of 10 using [macro](#pow10-macro).

```rust
fn binary_search(n: u64) -> usize {
    if n < pow10!(16) {
        if n < pow10!(8) {
            if n < pow10!(4) {
                if n < pow10!(2) {
                    if n < pow10!(1) {
                        1
                    } else {
                        2
                    }
                } else {
                    if n < pow10!(3) {
                        3
                    } else {
                        4
                    }
                }
            } else {
                if n < pow10!(6) {
                    if n < pow10!(5) {
                        5
                    }
                    else {
                        6
                    }
                } else {
                    if n < pow10!(7) {
                        7
                    } else {
                        8
                    }
                }
            }
        } else {
           if n < pow10!(12) {
                if n < pow10!(10) {
                    if n < pow10!(9) {
                        9
                    } else {
                        10
                    }
                } else {
                    if n < pow10!(11) {
                        11
                    } else {
                        12
                    }
                }
            } else {
                if n < pow10!(14) {
                    if n < pow10!(13) {
                        13
                    }
                    else {
                        14
                    }
                } else {
                    if n < pow10!(15) {
                        15
                    } else {
                        16
                    }
                }
            }
        }
    } else {
        if n < pow10!(18) {
            if n < pow10!(17) {
                17
            } else {
                18
            }
        } else {
            if n < pow10!(19) {
                19
            } else {
                20
            }
        }
    }
}
```

### pattern_match
Map into a range with pattern matching. We need to bind ranges to constants since patterns
can’t match against constant expression. Powers of 10 using [macro](#pow10-macro).

```rust
fn pattern_match(n: u64) -> usize {
    use ::std::u64::MIN;

    const POW10_1:  u64 = pow10!(1);
    const POW10_2:  u64 = pow10!(2);
    const POW10_3:  u64 = pow10!(3);
    const POW10_4:  u64 = pow10!(4);
    const POW10_5:  u64 = pow10!(5);
    const POW10_6:  u64 = pow10!(6);
    const POW10_7:  u64 = pow10!(7);
    const POW10_8:  u64 = pow10!(8);
    const POW10_9:  u64 = pow10!(9);
    const POW10_10: u64 = pow10!(10);
    const POW10_11: u64 = pow10!(11);
    const POW10_12: u64 = pow10!(12);
    const POW10_13: u64 = pow10!(13);
    const POW10_14: u64 = pow10!(14);
    const POW10_15: u64 = pow10!(15);
    const POW10_16: u64 = pow10!(16);
    const POW10_17: u64 = pow10!(17);
    const POW10_18: u64 = pow10!(18);
    const POW10_19: u64 = pow10!(19);

    const POW10_1_M:  u64 = POW10_1 - 1;
    const POW10_2_M:  u64 = POW10_2 - 1;
    const POW10_3_M:  u64 = POW10_3 - 1;
    const POW10_4_M:  u64 = POW10_4 - 1;
    const POW10_5_M:  u64 = POW10_5 - 1;
    const POW10_6_M:  u64 = POW10_6 - 1;
    const POW10_7_M:  u64 = POW10_7 - 1;
    const POW10_8_M:  u64 = POW10_8 - 1;
    const POW10_9_M:  u64 = POW10_9 - 1;
    const POW10_10_M: u64 = POW10_10 - 1;
    const POW10_11_M: u64 = POW10_11 - 1;
    const POW10_12_M: u64 = POW10_12 - 1;
    const POW10_13_M: u64 = POW10_13 - 1;
    const POW10_14_M: u64 = POW10_14 - 1;
    const POW10_15_M: u64 = POW10_15 - 1;
    const POW10_16_M: u64 = POW10_16 - 1;
    const POW10_17_M: u64 = POW10_17 - 1;
    const POW10_18_M: u64 = POW10_18 - 1;
    const POW10_19_M: u64 = POW10_19 - 1;

    match n {
        MIN      ... POW10_1_M  => 1,
        POW10_1  ... POW10_2_M  => 2,
        POW10_2  ... POW10_3_M  => 3,
        POW10_3  ... POW10_4_M  => 4,
        POW10_4  ... POW10_5_M  => 5,
        POW10_5  ... POW10_6_M  => 6,
        POW10_6  ... POW10_7_M  => 7,
        POW10_7  ... POW10_8_M  => 8,
        POW10_8  ... POW10_9_M  => 9,
        POW10_9  ... POW10_10_M => 10,
        POW10_10 ... POW10_11_M => 11,
        POW10_11 ... POW10_12_M => 12,
        POW10_12 ... POW10_13_M => 13,
        POW10_13 ... POW10_14_M => 14,
        POW10_14 ... POW10_15_M => 15,
        POW10_15 ... POW10_16_M => 16,
        POW10_16 ... POW10_17_M => 17,
        POW10_17 ... POW10_18_M => 18,
        POW10_18 ... POW10_19_M => 19,
        _ => 20,
    }
}
```

### cmp_list
Compare digit by digit. Powers of 10 using [macro](#pow10-macro).

```rust
fn cmp_list(n) -> usize {
    if n >= pow10!(19) {
        20
    } else if n >= pow10!(18) {
        19
    } else if n >= pow10!(17) {
        18
    } else if n >= pow10!(16) {
        17
    } else if n >= pow10!(15) {
        16
    } else if n >= pow10!(14) {
        15
    } else if n >= pow10!(13) {
        14
    } else if n >= pow10!(12) {
        13
    } else if n >= pow10!(11) {
        12
    } else if n >= pow10!(10) {
        11
    } else if n >= pow10!(9) {
        10
    } else if n >= pow10!(8) {
        9
    } else if n >= pow10!(7) {
        8
    } else if n >= pow10!(6) {
        7
    } else if n >= pow10!(5) {
        6
    } else if n >= pow10!(4) {
        5
    } else if n >= pow10!(3) {
        4
    } else if n >= pow10!(2) {
        3
    } else if n >= pow10!(1) {
        2
    } else {
        1
    }
}
```

### most_significant_bit
We know that `2^i <= num < 2^(i+1)` where i is the highest set bit of the num.
Use ctlz and lookup tables to resolve number of didgits with a single if statment.
Powers of 10 using [macro](#pow10-macro).

```rust
pub static DEC: [u64; 21] = [
    pow10!(0),  pow10!(1),  pow10!(2),  pow10!(3),  pow10!(4),
    pow10!(5),  pow10!(6),  pow10!(7),  pow10!(8),  pow10!(9),
    pow10!(10), pow10!(11), pow10!(12), pow10!(13), pow10!(14),
    pow10!(15), pow10!(16), pow10!(17), pow10!(18), pow10!(19),
    pow10!(19),
];

pub static BIN_TO_DEC: [u8; 130] = [
     1,  1,  1,  1,  1,  2,  2,  2,  3,  3,  3,  4,
     4,  4,  4,  5,  5,  5,  6,  6,  6,  7,  7,  7,
     7,  8,  8,  8,  9,  9,  9, 10, 10, 10, 10, 11,
    11, 11, 12, 12, 12, 13, 13, 13, 13, 14, 14, 14,
    15, 15, 15, 16, 16, 16, 16, 17, 17, 17, 18, 18,
    18, 19, 19, 19, 19, 20, 20, 20, 21, 21, 21, 22,
    22, 22, 22, 23, 23, 23, 24, 24, 24, 25, 25, 25,
    25, 26, 26, 26, 27, 27, 27, 28, 28, 28, 28, 29,
    29, 29, 30, 30, 30, 31, 31, 31, 32, 32, 32, 32,
    33, 33, 33, 34, 34, 34, 35, 35, 35, 35, 36, 36,
    36, 37, 37, 37, 38, 38, 38, 38, 39, 39,
];

fn most_significant_bit(n: u64) -> usize {
    let i = 64 - n.leading_zeros() as usize;
    let l = BIN_TO_DEC[i];
    let h = BIN_TO_DEC[i + 1];
    if *self >= DEC[(h - 1) as usize] {
        h as usize
    } else {
        l as usize
    }
}
```

### log
Cast to double then call log10, this is going to be inaccurate for large numbers.

```rust
fn log(&self) -> usize {
    (*self as f64).log10() as u32 as usize + 1
}
```


### str_format
Use the standard library format macro to format as a string then measure how long the string is. 

```rust
fn str_format(n: u64) -> usize {
    format!("{}", n).len()
}
```

### str_format
The format macro will allocate a new String on the heap, we can instead format into a stack allocated array.

```rust
use std::io::{Write, Cursor};

fn str_format_stack(n: u64) -> usize {
    let mut b = [0u8; 20];
    let mut c = Cursor::new(b.as_mut());
    write!(c, "{}", n).unwrap();
    c.position() as usize
}
```

### str_itoa_stack
The built in formatting code in the standard library is not the fastest, we instead format into the stack using the
[itoa crate](https://github.com/dtolnay/itoa).

```rust
use std::io::Cursor;

fn str_itoa_stack(n: u64) -> usize {
    use std::io::Cursor;
    let mut b = [0u8; 20];
    let mut c = Cursor::new(b.as_mut());
    itoa::write(&mut c, n).unwrap();
    c.position() as usize
}
```
