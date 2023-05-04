use commons::{Result, WrapErr};

use super::int_code::{IntCodeInput, Processor, Status};

pub const TITLE: &str = "Day 5: Sunny with a Chance of Asteroids";

pub fn run(raw: String) -> Result<()> {
    let data = parse(&raw)?;
    let (first, second) = solve(&data.data[..]).wrap_err("Program should not have crashed !")?;
    println!("Input 1 produced : {first}");
    println!("Input 5 produced : {second}");

    Ok(())
}

fn parse(s: &str) -> Result<IntCodeInput> {
    Ok(s.parse()?)
}

fn solve(memory: &[i64]) -> Option<(i64, i64)> {
    Some((run_program(memory, 1)?, run_program(memory, 5)?))
}

/// Runs the IntCode program with the given input and return its last output if it halted.
fn run_program(program: &[i64], input: i64) -> Option<i64> {
    let mut program: Processor = program.into();
    program.write_int(input);
    let mut output_count: usize = 0;
    let mut current: i64 = 0;
    let status = program.run_with_callbacks(
        0,
        |_| None,
        |_, out| {
            current = out;
            output_count += 1;
            Ok(())
        },
    );

    if let Status::Halted = status {
        if output_count == 0 {
            None
        } else {
            Some(current)
        }
    } else {
        println!("Input {input} produced unexpected status: {status:?}");
        None
    }
}

#[cfg(test)]
mod tests;
