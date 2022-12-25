#![feature(array_windows)]
#![feature(box_into_inner)]
#![feature(byte_slice_trim_ascii)]
#![feature(generic_const_exprs)]
#![feature(get_many_mut)]
#![feature(let_chains)]
#![feature(never_type)]
#![feature(stmt_expr_attributes)]

#![allow(incomplete_features)]

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
    day15,
    day16,
    day17,
    day18,
    day19,
    day20,
    day21,
    day22,
    day23,
    day24,
    day25,
);
