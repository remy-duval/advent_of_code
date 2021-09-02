use std::str::FromStr;

use commons::parse::LineSep;
use commons::Problem;

pub struct Day;

impl Problem for Day {
    type Input = SortedAdapters;
    const TITLE: &'static str = "Day 10: Adapter Array";

    fn solve(data: Self::Input) -> color_eyre::Result<()> {
        let (ones, threes) = first_part(&data);

        println!(
            "The full adapter chain has {ones} (1V difference) * {threes} (3V difference) = {sum}",
            ones = ones,
            threes = threes,
            sum = ones * threes
        );

        println!(
            "The number of arrangements of adapters for reaching the wanted value is: {total}",
            total = second_part(data)
        );

        Ok(())
    }
}

/// Contains the adapters for the problems, already sorted and including the last one (max + 3)
pub struct SortedAdapters(Vec<usize>);

impl FromStr for SortedAdapters {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut elements = s.parse::<LineSep<usize>>()?.data;
        elements.sort_unstable();
        Ok(SortedAdapters(elements))
    }
}

/// Find a chain of all the adapters that connect 0 to max + 3
/// Then return the number of 1V differences and the number of 3V differences in it
///
/// ### Arguments
/// * `adapters` - All the adapters that are considered (must be sorted)
///
/// ### Returns
/// Maybe (ones, threes), the counts of 1V diff and 3V diff in the chain of adapters found
fn first_part(adapters: &SortedAdapters) -> (usize, usize) {
    let (ones, threes, _) = {
        // The adapters are sorted, so this is already the full chain !
        // The only thing that remains is to count the number of 1s and 3s differences
        // Also the last adapter in the problem is MAX + 3V, so the 'threes' counter starts at 1
        adapters
            .0
            .iter()
            .fold((0, 1, 0), |(ones, threes, current), &next| {
                let difference = next - current;
                let ones = if difference == 1 { ones + 1 } else { ones };
                let threes = if difference == 3 { threes + 1 } else { threes };
                (ones, threes, next)
            })
    };

    (ones, threes)
}

/// Find the possibilities for arranging adapters to get to the maximum value in V
///
/// ### Arguments
/// * `adapters` - All the adapters that are considered (must be sorted)
///
/// ### Returns
/// The number of arrangement of adapters that can chain to reach their maximum V value
fn second_part(adapters: SortedAdapters) -> usize {
    if let Some(maximum) = adapters.0.last() {
        let mut memoized = vec![0; maximum + 1];
        memoized[0] = 1;
        adapters.0.into_iter().for_each(|voltage| {
            memoized[voltage] = memoized[voltage.saturating_sub(3)..voltage].iter().sum();
        });

        memoized.last().copied().unwrap_or_default()
    } else {
        0
    }
}

#[cfg(test)]
mod tests;
