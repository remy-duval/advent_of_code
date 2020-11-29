//! The solutions to the problems of the Advent of Code of 2019

use crate::Day;
use crate::Problem;

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
// pub mod day05;
// pub mod day06;
// pub mod day07;
// pub mod day08;
// pub mod day09;
// pub mod day10;
// pub mod day11;
// pub mod day12;
// pub mod day13;
// pub mod day14;
// pub mod day15;
// pub mod day16;
// pub mod day17;
// pub mod day18;
// pub mod day19;
// pub mod day20;
// pub mod day21;
// pub mod day22;
// pub mod day23;
// pub mod day24;
// pub mod day25;
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
