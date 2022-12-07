#![warn(clippy::all)]
#![feature(allocator_api)]
#![feature(iter_collect_into)]
// #![no_std]

extern crate alloc;

pub const NUM_DAYS: usize = 7;

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;

pub(crate) mod bitset;
pub(crate) mod hash;
