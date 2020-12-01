//! The solutions to the problems of the Advent of Code of 2019

use crate::Day;
use crate::Problem;

mod day01;
mod day02;
mod day03;
mod day04;
// mod day05;
// mod day06;
// mod day07;
// mod day08;
// mod day09;
// mod day10;
// mod day11;
// mod day12;
// mod day13;
// mod day14;
// mod day15;
// mod day16;
// mod day17;
// mod day18;
// mod day19;
// mod day20;
// mod day21;
// mod day22;
// mod day23;
// mod day24;
// mod day25;
pub mod int_code;

/// Solve a problem in the year 2019
pub fn solve(day: Day, input: String) -> anyhow::Result<()> {
    match day.0 {
        1 => day01::Day01::parse_and_solve(input)?,
        2 => day02::Day02::parse_and_solve(input)?,
        3 => day03::Day03::parse_and_solve(input)?,
        4 => day04::Day04::parse_and_solve(input)?,
        _ => return Err(anyhow::anyhow!("{} is not implemented", day)),
    }
    Ok(())
}
