use aoc::generator::data_from_cli;
use aoc::int_code::{parse_int_code, Processor};
use std::error::Error;

const TITLE: &str = "Day 9: Sensor Boost";
const DATA: &str = include_str!("../resources/day09.txt");

fn main() -> Result<(), Box<dyn Error>> {
    let data = data_from_cli(TITLE, DATA);
    println!("{}", TITLE);
    let memory = parse_int_code(&data)?;

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
    let coordinates = boost_process.read_next().expect("BOOST failed !");
    println!("The BOOST program coordinates were {}", coordinates);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_test() -> Result<(), Box<dyn Error>> {
        let memory = parse_int_code(&DATA)?;

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
        assert_eq!(2_752_191_671, current);

        let mut boost_process = Processor::with_initial_inputs(&memory, &[2]);
        let coordinates = boost_process.read_next().expect("BOOST failed !");
        assert_eq!(87_571, coordinates);

        Ok(())
    }
}
