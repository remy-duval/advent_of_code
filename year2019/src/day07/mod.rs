use commons::eyre::Result;
use itertools::Itertools;

use commons::Problem;

use super::int_code::{IntCodeInput, Processor, Status};

pub struct Day;

impl Problem for Day {
    type Input = IntCodeInput;
    const TITLE: &'static str = "Day 7: Amplification Circuit";

    fn solve(data: Self::Input) -> Result<()> {
        println!(
            "The best output with single cycle is {}",
            single_loop(&data.data)
        );
        println!(
            "The best output with feedback loop is {}",
            feedback_loop(&data.data)
        );

        Ok(())
    }
}

/// Finds the maximum output with the single loop phases
fn single_loop(memory: &[i64]) -> i64 {
    maximum_output(memory, &[0, 1, 2, 3, 4])
}

/// Finds the maximum output with the feedback loop phases
fn feedback_loop(memory: &[i64]) -> i64 {
    maximum_output(memory, &[5, 6, 7, 8, 9])
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
mod tests;
