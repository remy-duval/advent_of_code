use super::int_code::{IntCodeInput, Processor, Status};

pub struct Day;

impl crate::Problem for Day {
    type Input = IntCodeInput;
    type Err = anyhow::Error;
    const TITLE: &'static str = "Day 5: Sunny with a Chance of Asteroids";

    fn solve(data: Self::Input) -> Result<(), Self::Err> {
        let (first, second) =
            solve(&data.data[..]).ok_or(anyhow::anyhow!("Program should not have crashed !"))?;
        println!("Input 1 produced : {}", first);
        println!("Input 5 produced : {}", second);

        Ok(())
    }
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
    use crate::Problem;

    const DATA: &str = include_str!("test_resources/day05.txt");

    #[test]
    fn solve_test() {
        let memory = Day::parse(&DATA).unwrap().data;
        let (first, second) = solve(&memory[..]).unwrap();

        assert_eq!(15_386_262, first);
        assert_eq!(10_376_124, second);
    }
}
