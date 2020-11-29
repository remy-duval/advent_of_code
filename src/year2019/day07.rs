use std::error::Error;

use itertools::Itertools;

use aoc::generator::data_from_cli;
use aoc::int_code::{parse_int_code, Processor, Status};

const TITLE: &str = "Day 7: Amplification Circuit";
const DATA: &str = include_str!("../resources/day07.txt");

fn main() -> Result<(), Box<dyn Error>> {
    let data = data_from_cli(TITLE, DATA);
    println!("{}", TITLE);
    let memory = parse_int_code(&data)?;
    println!(
        "The best output with single cycle is {}",
        single_loop(&memory)
    );
    println!(
        "The best output with feedback loop is {}",
        feedback_loop(&memory)
    );

    Ok(())
}

/// Finds the maximum output with the single loop phases
fn single_loop(memory: &[i64]) -> i64 {
    maximum_output(&memory, &[0, 1, 2, 3, 4])
}

/// Finds the maximum output with the feedback loop phases
fn feedback_loop(memory: &[i64]) -> i64 {
    maximum_output(&memory, &[5, 6, 7, 8, 9])
}

/// Finds the maximum output of the process with the given memory and possible phases
fn maximum_output(memory: &[i64], base: &[u8]) -> i64 {
    let mut max = 0;
    let mut best: Vec<u8> = vec![];
    for phases in base.to_vec().into_iter().permutations(base.len()) {
        let result = amplifier(memory, &phases).unwrap_or(0);
        if result > max {
            max = result;
            best = phases;
        }
    }

    println!("Best permutation is ({})", best.iter().join(", "));
    max
}

/// Runs the amplifier with the given memory and phases.
fn amplifier(memory: &[i64], inputs: &[u8]) -> Option<i64> {
    let mut programs: Vec<_> = inputs
        .iter()
        .map(|phase| Processor::with_initial_inputs(memory, &[*phase as i64]))
        .collect();

    let mut done = false;
    let mut input = Some(0);
    while !done {
        for program in programs.iter_mut() {
            program.write_int(input?);
            input = program.read_next().ok();

            // Check if program is halting next (in that case we can stop)
            if let Ok(Status::Halted) = program.run() {
                done = true;
            }
        }
    }

    input
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn first_part() {
        let one = [
            3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
        ];
        let two = [
            3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23,
            99, 0, 0,
        ];
        let third = [
            3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33, 1,
            33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0,
        ];

        assert_eq!(43210, single_loop(&one));
        assert_eq!(54321, single_loop(&two));
        assert_eq!(65210, single_loop(&third));
    }

    #[test]
    fn second_part() {
        let one = [
            3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28, -1,
            28, 1005, 28, 6, 99, 0, 0, 5,
        ];
        let two = [
            3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26, 1001, 54,
            -5, 54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55, 2, 53, 55, 53, 4,
            53, 1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10,
        ];

        assert_eq!(139_629_729, feedback_loop(&one));
        assert_eq!(18216, feedback_loop(&two));
    }

    #[test]
    fn solve_test() -> Result<(), Box<dyn Error>> {
        let memory = parse_int_code(&DATA)?;
        assert_eq!(11828, single_loop(&memory));
        assert_eq!(1_714_298, feedback_loop(&memory));

        Ok(())
    }
}
