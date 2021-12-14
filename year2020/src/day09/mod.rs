use itertools::Itertools;

use commons::eyre::{eyre, Result};
use commons::parse::LineSep;

pub const TITLE: &str = "Day 9: Encoding Error";

pub fn run(raw: String) -> Result<()> {
    let data = parse(&raw)?;
    let wanted = first_not_sum(&data.data, PREAMBLE)
        .ok_or_else(|| eyre!("Did not find the first element that is not a sum"))?;

    println!(
        "The first element that is not a sum of the previous ones is {first}",
        first = wanted
    );

    let (min, max) = second_part(&data.data, wanted)
        .ok_or_else(|| eyre!("Did not find the slice that can be summed to {}", wanted))?;

    println!(
        "The slice between {min} and {max} (sum is {sum}) will sum up to it",
        min = min,
        max = max,
        sum = min + max
    );

    Ok(())
}

fn parse(s: &str) -> Result<LineSep<u64>> {
    Ok(s.parse()?)
}

const PREAMBLE: usize = 25;

/// Find the first element in `data` that is not a sum of the `from_previous` previous elements
fn first_not_sum(data: &[u64], from_previous: usize) -> Option<u64> {
    (from_previous..data.len()).find_map(|end| {
        let start = end - from_previous;
        let value = data[end];
        if !is_sum(&data[start..end], value) {
            Some(value)
        } else {
            None
        }
    })
}

/// Find the first contiguous set that sum up to `wanted` in `data` and get its min and max
fn second_part(data: &[u64], wanted: u64) -> Option<(u64, u64)> {
    contiguous_set(data, wanted)?
        .iter()
        .copied()
        .minmax()
        .into_option()
}

/// Find the first contiguous set of at least 2 elements in `data` that sum up to `wanted`
fn contiguous_set(data: &[u64], wanted: u64) -> Option<&[u64]> {
    if data.is_empty() {
        return None;
    }

    let mut start = 0;
    let mut sum = data[0];
    (1..data.len()).find_map(|end| {
        sum += data[end];
        while sum > wanted && start < (end - 1) {
            sum -= data[start];
            start += 1;
        }
        if sum == wanted {
            Some(&data[start..(end + 1)])
        } else {
            None
        }
    })
}

/// Tell if the `wanted` number is a sum of any two numbers in `inside`
fn is_sum(inside: &[u64], wanted: u64) -> bool {
    inside.iter().enumerate().any(|(i, &first)| {
        if first <= wanted {
            let to_find = wanted - first;
            inside
                .iter()
                .enumerate()
                .any(|(j, &second)| i != j && second == to_find)
        } else {
            false
        }
    })
}

#[cfg(test)]
mod tests;
