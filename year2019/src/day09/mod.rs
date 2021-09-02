use color_eyre::eyre::{eyre, Result};

use commons::Problem;

use super::int_code::{IntCodeInput, Processor};

pub struct Day;

impl Problem for Day {
    type Input = IntCodeInput;
    const TITLE: &'static str = "Day 9: Sensor Boost";

    fn solve(data: Self::Input) -> Result<()> {
        let memory: Vec<i64> = data.data;
        let mut test_process = Processor::with_initial_inputs(&memory, &[1]);
        let mut output_count: usize = 0;
        let mut current: i64 = 0;
        test_process.run_with_callbacks(
            0,
            |_| None,
            |_, out| {
                current = out;
                output_count += 1;
                Ok(())
            },
        );
        assert_eq!(output_count, 1, "The TEST program should output once only");
        println!("The TEST program single output was {}", current);

        let mut boost_process = Processor::with_initial_inputs(&memory, &[2]);
        match boost_process.read_next() {
            Err(status) => Err(eyre!("BOOST failed ! (Status was {:?})", status)),
            Ok(coordinates) => {
                println!("The BOOST program coordinates were {}", coordinates);
                Ok(())
            }
        }
    }
}

#[cfg(test)]
mod tests;
