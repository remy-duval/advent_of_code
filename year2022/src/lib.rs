use std::path::PathBuf;

use commons::arguments::Day;
use commons::problem::{solve_quiet, solve_verbose};
use commons::{err, Result};

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
// mod day24;
// mod day25;

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
        // Day::Day24 => solve_verbose(day24::TITLE, input, day24::run),
        // Day::Day25 => solve_verbose(day25::TITLE, input, day25::run),
        Day::All => solve_all(input),
        _ => Err(err!("unsupported day: {day:?}")),
    }
}

/// Solve all the problems for this year in a row, timing them all
pub fn solve_all(dir: PathBuf) -> Result<()> {
    fn all(dir: PathBuf) -> Result<()> {
        solve_quiet(1, dir.join("01.txt"), day01::run)?;
        solve_quiet(2, dir.join("02.txt"), day02::run)?;
        solve_quiet(3, dir.join("03.txt"), day03::run)?;
        solve_quiet(4, dir.join("04.txt"), day04::run)?;
        solve_quiet(5, dir.join("05.txt"), day05::run)?;
        solve_quiet(6, dir.join("06.txt"), day06::run)?;
        solve_quiet(7, dir.join("07.txt"), day07::run)?;
        solve_quiet(8, dir.join("08.txt"), day08::run)?;
        solve_quiet(9, dir.join("09.txt"), day09::run)?;
        solve_quiet(10, dir.join("10.txt"), day10::run)?;
        solve_quiet(11, dir.join("11.txt"), day11::run)?;
        solve_quiet(12, dir.join("12.txt"), day12::run)?;
        solve_quiet(13, dir.join("13.txt"), day13::run)?;
        solve_quiet(14, dir.join("14.txt"), day14::run)?;
        solve_quiet(15, dir.join("15.txt"), day15::run)?;
        solve_quiet(16, dir.join("16.txt"), day16::run)?;
        solve_quiet(17, dir.join("17.txt"), day17::run)?;
        solve_quiet(18, dir.join("18.txt"), day18::run)?;
        solve_quiet(19, dir.join("19.txt"), day19::run)?;
        solve_quiet(20, dir.join("20.txt"), day20::run)?;
        solve_quiet(21, dir.join("21.txt"), day21::run)?;
        solve_quiet(22, dir.join("22.txt"), day22::run)?;
        solve_quiet(23, dir.join("23.txt"), day23::run)?;
        // solve_quiet(24, dir.join("24.txt"), day24::run)?;
        // solve_quiet(25, dir.join("25.txt"), day25::run)?;
        Ok(())
    }

    let start = std::time::Instant::now();
    let result = all(dir);
    let elapsed = start.elapsed();
    println!("\n\nSolve time: {:}ms", elapsed.as_millis());

    result
}
