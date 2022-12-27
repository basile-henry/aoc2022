#![warn(clippy::all)]
#![feature(allocator_api)]
#![feature(iter_collect_into)]
// #![no_std]

extern crate alloc;

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day18;
pub mod day24;
pub mod day25;

#[allow(dead_code)]
pub(crate) mod bitset;
#[allow(dead_code)]
pub(crate) mod hash;
#[allow(dead_code)]
pub(crate) mod nom_extra;
