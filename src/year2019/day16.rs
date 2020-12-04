use std::convert::TryInto;
use std::str::FromStr;

use itertools::Itertools;

use crate::Problem;

const REPEAT: usize = 10000;

pub struct Day;

impl Problem for Day {
    type Input = Signal;
    type Err = std::convert::Infallible;
    const TITLE: &'static str = "Day 16: Flawed Frequency Transmission";

    fn solve(data: Self::Input) -> Result<(), Self::Err> {
        // First part
        let output = naive_fft(&data.0, 100).into_iter().take(8).join("");
        println!("The first 8 digits of the simple output are {}", output);

        // Second part
        let output = fast_second_half_fft(&data.0, 100)
            .into_iter()
            .take(8)
            .join("");
        println!(
            "The first 8 digits of the repeated {} times output are {}",
            REPEAT, output
        );

        Ok(())
    }
}

pub struct Signal(Vec<i32>);

impl FromStr for Signal {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data: Option<Vec<i32>> = s
            .chars()
            .filter(|c| c.is_numeric())
            .map(|c| c.to_digit(10).and_then(|d| d.try_into().ok()))
            .collect::<Option<_>>();

        Ok(Signal(
            data.ok_or(anyhow::anyhow!("Error parsing the input !"))?,
        ))
    }
}

/// Applies the FFT using the algorithm given. It is correct but slow.
fn naive_fft(input: &[i32], steps: usize) -> Vec<i32> {
    let mut output = input.to_vec();
    (0..steps).for_each(|_| {
        (0..input.len()).for_each(|output_idx| {
            output[output_idx] = output
                .iter()
                .enumerate()
                .map(|(idx, value)| *value * pattern_element(idx, output_idx))
                .sum::<i32>()
                .abs()
                % 10;
        })
    });

    output
}

/// The pattern for the FFT for 1) the input index 2) the output index
fn pattern_element(idx: usize, output_idx: usize) -> i32 {
    match ((idx + 1) / (output_idx + 1)) % 4 {
        0 | 2 => 0,
        1 => 1,
        _ => -1,
    }
}

/// Applies the FFT on REPEAT * the input using workaround when offset >= real_input / 2
fn fast_second_half_fft(input: &[i32], steps: usize) -> Vec<i32> {
    // The total number of elements to FFT over.
    let total = input.len() * REPEAT;
    // The offset of the 8 numbers to return at the end.
    let offset = input[0..7]
        .iter()
        .fold(0, |acc, &next| acc * 10 + next as usize);

    assert!(
        offset >= total / 2 && offset < total - 8,
        "Assumption broken : Offset is not between total / 2 and total - 8"
    );

    // Build the real signal (input * REPEAT with the useless offsets
    let mut real_input: Vec<i32> = input
        .iter()
        .cycle()
        .take(total)
        .skip(offset)
        .copied()
        .collect();

    // Apply the FFT
    (0..steps).for_each(|_| {
        // Instead of applying the full FFT we can just sum all the numbers from the end
        // Each output digit is just the first digit of the current sum
        let mut sum = 0;
        real_input.iter_mut().rev().for_each(move |value| {
            sum += *value;
            *value = sum.abs() % 10;
        });
    });

    real_input.into_iter().take(8).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_ONE: &str = "12345678";
    const TEST_TWO: &str = "80871224585914546619083218645595";
    const TEST_THREE: &str = "19617804207202209144916044189917";
    const TEST_FOUR: &str = "69317163492948606335995924319873";
    const TEST_FIVE: &str = "03036732577212944063491565474664";
    const TEST_SIX: &str = "02935109699940807407585447034323";
    const TEST_SEVEN: &str = "03081770884921959731165446850517";
    const DATA: &str = include_str!("test_resources/day16.txt");

    #[test]
    fn patterns_test() {
        assert_eq!(1, pattern_element(0, 0));
        assert_eq!(0, pattern_element(0, 1));
        assert_eq!(0, pattern_element(0, 2));

        assert_eq!(0, pattern_element(1, 0));
        assert_eq!(1, pattern_element(1, 1));
        assert_eq!(0, pattern_element(1, 2));

        let first: Vec<i32> = (0..8).map(|idx| pattern_element(idx, 0)).collect();
        assert_eq!(&[1, 0, -1, 0, 1, 0, -1, 0], &first[..]);

        let second: Vec<i32> = (0..8).map(|idx| pattern_element(idx, 1)).collect();
        assert_eq!(&[0, 1, 1, 0, 0, -1, -1, 0], &second[..]);

        let third: Vec<i32> = (0..8).map(|idx| pattern_element(idx, 2)).collect();
        assert_eq!(&[0, 0, 1, 1, 1, 0, 0, 0], &third[..]);

        let fourth: Vec<i32> = (0..8).map(|idx| pattern_element(idx, 3)).collect();
        assert_eq!(&[0, 0, 0, 1, 1, 1, 1, 0], &fourth[..]);
    }

    #[test]
    fn naive_fft_test() {
        fn assertion(data: &str, steps: usize, expected: [i32; 8]) {
            let input: Vec<i32> = Day::parse(data).unwrap().0;
            assert_eq!(&expected, &naive_fft(&input, steps)[..8])
        }

        assertion(TEST_ONE, 1, [4, 8, 2, 2, 6, 1, 5, 8]);
        assertion(TEST_ONE, 2, [3, 4, 0, 4, 0, 4, 3, 8]);
        assertion(TEST_ONE, 3, [0, 3, 4, 1, 5, 5, 1, 8]);
        assertion(TEST_ONE, 4, [0, 1, 0, 2, 9, 4, 9, 8]);
        assertion(TEST_TWO, 100, [2, 4, 1, 7, 6, 1, 7, 6]);
        assertion(TEST_THREE, 100, [7, 3, 7, 4, 5, 4, 1, 8]);
        assertion(TEST_FOUR, 100, [5, 2, 4, 3, 2, 1, 3, 3]);
        assertion(DATA, 100, [2, 9, 7, 9, 5, 5, 0, 7]);
    }

    #[test]
    fn fast_second_half_fft_test() {
        fn assertion(data: &str, steps: usize, expected: [i32; 8]) {
            let input: Vec<i32> = Day::parse(data).unwrap().0;
            assert_eq!(&expected, &fast_second_half_fft(&input, steps)[..])
        }

        assertion(TEST_FIVE, 100, [8, 4, 4, 6, 2, 0, 2, 6]);
        assertion(TEST_SIX, 100, [7, 8, 7, 2, 5, 2, 7, 0]);
        assertion(TEST_SEVEN, 100, [5, 3, 5, 5, 3, 7, 3, 1]);
        assertion(DATA, 100, [8, 9, 5, 6, 8, 5, 2, 9]);
    }
}
