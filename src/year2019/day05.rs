use std::error::Error;

use aoc::generator::data_from_cli;
use aoc::int_code::{parse_int_code, IntCodeError::Other, Processor, Status};

const TITLE: &str = "Day 5: Sunny with a Chance of Asteroids";
const DATA: &str = include_str!("../resources/day05.txt");

fn main() -> Result<(), Box<dyn Error>> {
    let data = data_from_cli(TITLE, DATA);
    println!("{}", TITLE);
    let memory = parse_int_code(&data)?;

    let (first, second) = solve(&memory[..])
        .ok_or_else(|| Box::new(Other("Program should not have crashed !".into())))?;
    println!("Input 1 produced : {}", first);
    println!("Input 5 produced : {}", second);

    Ok(())
}

fn solve(memory: &[i64]) -> Option<(i64, i64)> {
    Some((run(memory, 1)?, run(memory, 5)?))
}

/// Runs the IntCode program with the given input and return its last output if it halted.
fn run(program: &[i64], input: i64) -> Option<i64> {
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
        println!("Input {} produced unexpected status: {:?}", input, status);
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_test() -> Result<(), Box<dyn Error>> {
        let memory = parse_int_code(&DATA)?;
        let (first, second) = solve(&memory[..])
            .ok_or_else(|| Box::new(Other("Program should not have crashed !".into())))?;

        assert_eq!(15_386_262, first);
        assert_eq!(10_376_124, second);

        Ok(())
    }
}
