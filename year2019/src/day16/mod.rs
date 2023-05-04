use itertools::Itertools;

use commons::{Result, WrapErr};

pub const TITLE: &str = "Day 16: Flawed Frequency Transmission";
const REPEAT: usize = 10000;

pub fn run(raw: String) -> Result<()> {
    let signal = parse(&raw)?;
    // First part
    let output = naive_fft(&signal, 100).into_iter().take(8).join("");
    println!("The first 8 digits of the simple output are {output}");

    // Second part
    let output = fast_second_half_fft(&signal, 100)
        .into_iter()
        .take(8)
        .join("");
    println!("The first 8 digits of the repeated {REPEAT} times output are {output}");

    Ok(())
}

fn parse(s: &str) -> Result<Vec<i32>> {
    let data: Option<Vec<i32>> = s
        .chars()
        .filter(|c| c.is_numeric())
        .map(|c| c.to_digit(10).and_then(|d| d.try_into().ok()))
        .collect::<Option<_>>();

    data.wrap_err("Error parsing the input !")
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
mod tests;
