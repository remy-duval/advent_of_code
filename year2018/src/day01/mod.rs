use std::collections::HashSet;

use commons::parse::LineSep;
use commons::Result;

pub const TITLE: &str = "Day 1: Chronal Calibration";

pub fn run(raw: String) -> Result<()> {
    let data = parse(&raw)?;
    println!("The sum of frequencies is {}", sum(&data.data));
    println!(
        "The first repeated frequency is {}",
        first_repeated(&data.data)
    );

    Ok(())
}

fn parse(s: &str) -> Result<LineSep<i32>> {
    Ok(s.parse()?)
}

/// The sum of all frequencies
fn sum(data: &[i32]) -> i32 {
    data.iter().sum()
}

/// Find the first repeated frequency sum
fn first_repeated(data: &[i32]) -> i32 {
    let mut seen: HashSet<i32> = HashSet::with_capacity(data.len());
    seen.insert(0);
    data.iter()
        .cycle()
        .scan(0, |acc, next| {
            *acc += *next;
            Some(*acc)
        })
        .find(|&sum| !seen.insert(sum))
        .unwrap() // Since the iterator is infinite, None is impossible
}

#[cfg(test)]
mod tests;
