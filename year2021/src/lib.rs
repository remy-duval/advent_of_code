#![deny(
    bad_style,
    const_err,
    dead_code,
    improper_ctypes,
    non_shorthand_field_patterns,
    no_mangle_generic_items,
    overflowing_literals,
    path_statements,
    patterns_in_fns_without_body,
    private_in_public,
    unconditional_recursion,
    unused,
    unused_allocation,
    unused_comparisons,
    unused_parens,
    while_true,
    missing_debug_implementations,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications
)]

use std::path::PathBuf;

use commons::eyre::{eyre, Result};
use commons::problem::solve_verbose;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
/* mod day21;
mod day22;
mod day23;
mod day24;
mod day25;
*/

/// Dispatch to the correct problem and solve it
pub fn solve_problem(day: u8, input: PathBuf) -> Result<()> {
    match day {
        1 => solve_verbose(day01::TITLE, input, day01::run),
        2 => solve_verbose(day02::TITLE, input, day02::run),
        3 => solve_verbose(day03::TITLE, input, day03::run),
        4 => solve_verbose(day04::TITLE, input, day04::run),
        5 => solve_verbose(day05::TITLE, input, day05::run),
        6 => solve_verbose(day06::TITLE, input, day06::run),
        7 => solve_verbose(day07::TITLE, input, day07::run),
        8 => solve_verbose(day08::TITLE, input, day08::run),
        9 => solve_verbose(day09::TITLE, input, day09::run),
        10 => solve_verbose(day10::TITLE, input, day10::run),
        11 => solve_verbose(day11::TITLE, input, day11::run),
        12 => solve_verbose(day12::TITLE, input, day12::run),
        13 => solve_verbose(day13::TITLE, input, day13::run),
        14 => solve_verbose(day14::TITLE, input, day14::run),
        15 => solve_verbose(day15::TITLE, input, day15::run),
        16 => solve_verbose(day16::TITLE, input, day16::run),
        17 => solve_verbose(day17::TITLE, input, day17::run),
        18 => solve_verbose(day18::TITLE, input, day18::run),
        19 => solve_verbose(day19::TITLE, input, day19::run),
        20 => solve_verbose(day20::TITLE, input, day20::run),
        /* 21 => solve_verbose(day21::TITLE, input, day21::run),
        22 => solve_verbose(day22::TITLE, input, day22::run),
        23 => solve_verbose(day23::TITLE, input, day23::run),
        24 => solve_verbose(day24::TITLE, input, day24::run),
        25 => solve_verbose(day25::TITLE, input, day25::run), */
        _ => Err(eyre!("{} is not implemented for year 2021", day)),
    }
}
