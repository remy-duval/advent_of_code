use std::path::PathBuf;

use commons::arguments::Day;
use commons::problem::solve_verbose;
use commons::{err, error::Result};

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
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;
pub mod int_code;

/// Dispatch to the correct problem and solve it
pub fn solve_problem(day: Day, input: PathBuf) -> Result<()> {
    match day {
        Day::Day1 => solve_verbose(day01::TITLE, input, day01::run),
        Day::Day2 => solve_verbose(day02::TITLE, input, day02::run),
        Day::Day3 => solve_verbose(day03::TITLE, input, day03::run),
        Day::Day4 => solve_verbose(day04::TITLE, input, day04::run),
        Day::Day5 => solve_verbose(day05::TITLE, input, day05::run),
        Day::Day6 => solve_verbose(day06::TITLE, input, day06::run),
        Day::Day7 => solve_verbose(day07::TITLE, input, day07::run),
        Day::Day8 => solve_verbose(day08::TITLE, input, day08::run),
        Day::Day9 => solve_verbose(day09::TITLE, input, day09::run),
        Day::Day10 => solve_verbose(day10::TITLE, input, day10::run),
        Day::Day11 => solve_verbose(day11::TITLE, input, day11::run),
        Day::Day12 => solve_verbose(day12::TITLE, input, day12::run),
        Day::Day13 => solve_verbose(day13::TITLE, input, day13::run),
        Day::Day14 => solve_verbose(day14::TITLE, input, day14::run),
        Day::Day15 => solve_verbose(day15::TITLE, input, day15::run),
        Day::Day16 => solve_verbose(day16::TITLE, input, day16::run),
        Day::Day17 => solve_verbose(day17::TITLE, input, day17::run),
        Day::Day18 => solve_verbose(day18::TITLE, input, day18::run),
        Day::Day19 => solve_verbose(day19::TITLE, input, day19::run),
        Day::Day20 => solve_verbose(day20::TITLE, input, day20::run),
        Day::Day21 => solve_verbose(day21::TITLE, input, day21::run),
        Day::Day22 => solve_verbose(day22::TITLE, input, day22::run),
        Day::Day23 => solve_verbose(day23::TITLE, input, day23::run),
        Day::Day24 => solve_verbose(day24::TITLE, input, day24::run),
        Day::Day25 => solve_verbose(day25::TITLE, input, day25::run),
        Day::All => Err(err!("'all' is not implemented for year 2019")),
    }
}
