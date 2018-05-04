#![feature(exclusive_range_pattern)]
#![feature(i128)]
#![feature(test)]

mod num_digits;
mod decimal;

#[cfg(test)]
mod test;

#[cfg(test)]
mod bench;