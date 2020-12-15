//! The solutions to the problems of the Advent of Code of 2020

use std::path::PathBuf;

use crate::Day;
use crate::parse_and_solve;

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

/// Solve a problem in the year 2020
pub fn solve(day: Day, input: PathBuf) -> anyhow::Result<()> {
    match day.0 {
        1 => parse_and_solve::<day01::Day>(input)?,
        2 => parse_and_solve::<day02::Day>(input)?,
        3 => parse_and_solve::<day03::Day>(input)?,
        4 => parse_and_solve::<day04::Day>(input)?,
        5 => parse_and_solve::<day05::Day>(input)?,
        6 => parse_and_solve::<day06::Day>(input)?,
        7 => parse_and_solve::<day07::Day>(input)?,
        8 => parse_and_solve::<day08::Day>(input)?,
        9 => parse_and_solve::<day09::Day>(input)?,
        10 => parse_and_solve::<day10::Day>(input)?,
        11 => parse_and_solve::<day11::Day>(input)?,
        12 => parse_and_solve::<day12::Day>(input)?,
        13 => parse_and_solve::<day13::Day>(input)?,
        14 => parse_and_solve::<day14::Day>(input)?,
        15 => parse_and_solve::<day15::Day>(input)?,
        16 => parse_and_solve::<day16::Day>(input)?,
        _ => return Err(anyhow::anyhow!("{} is not implemented", day)),
    }
    Ok(())
}
