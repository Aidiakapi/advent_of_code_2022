#![feature(array_windows)]
#![feature(byte_slice_trim_ascii)]

#![feature(custom_test_frameworks)]
#![cfg_attr(feature = "criterion", test_runner(criterion::runner))]

#![allow(clippy::zero_prefixed_literal)]

mod prelude;

framework::main!(
    day01,
    day02,
    day03,
    day04,
    day05,
    day06,
    day07,
    day08,
    day09,
    day10,
    day11,
    day12,
    day13,
    day14,
);
