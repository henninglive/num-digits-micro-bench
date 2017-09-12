#![feature(test)]
#![feature(exclusive_range_pattern)]
#![feature(i128_type)]
#![feature(i128)]

mod num_digits;
mod decimal;

#[cfg(test)]
mod test;

#[cfg(test)]
mod bench;