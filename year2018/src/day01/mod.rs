use hashbrown::HashSet;

use commons::parse::LineSep;
use commons::Problem;

pub struct Day;

impl Problem for Day {
    type Input = LineSep<i32>;
    type Err = std::convert::Infallible;
    const TITLE: &'static str = "Day 1: Chronal Calibration";

    fn solve(data: Self::Input) -> Result<(), Self::Err> {
        println!("The sum of frequencies is {}", sum(&data.data));
        println!(
            "The first repeated frequency is {}",
            first_repeated(&data.data)
        );

        Ok(())
    }
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
