use color_eyre::Result;

use commons::parse::CommaSep;
use commons::Problem;

const FIRST_TURNS: u32 = 2020;
const SECOND_TURNS: u32 = 30000000;

/// This day is based on the [Van Eck sequence](https://www.numberphile.com/videos/van-eck-sequence)
///
/// The definition of the Van Eck sequence is:
/// * `VanEck(0)` = `0`
/// * `VanEck(n+1)` =
///   * if `VanEck(n)` exists in `VanEck(0..n)` `n - prev_occ`
///   * else `0`
///
/// In this our the base case is overridden with the given `prefix`:
/// Such that `VanEck(0..prefix.len())` = `prefix`
pub struct Day;

impl Problem for Day {
    type Input = CommaSep<u32>;
    const TITLE: &'static str = "Day 15: Rambunctious Recitation";

    fn solve(data: Self::Input) -> Result<()> {
        let first = nth_spoken_number(&data.data, FIRST_TURNS);
        println!("The {}th spoken is {}", FIRST_TURNS, first);

        let second = nth_spoken_number(&data.data, SECOND_TURNS);
        println!("The {}th spoken is {}", SECOND_TURNS, second);

        Ok(())
    }
}

/// Compute the nth term using a Vec as the backing memoization:
///
/// Use a Vec of fixed length max of (prefix values, turns) + 1
/// For any `number`, the value at the index `number` means:
/// 0 => number was never spoken,
/// t => number was spoken on turn n
fn nth_spoken_number(prefix: &[u32], n: u32) -> u32 {
    let size = prefix.iter().copied().max().unwrap_or_default().max(n);
    let mut spoken: Vec<u32> = vec![0; (size as usize) + 1];

    let mut current = 0;
    let mut index = 0;
    prefix.iter().for_each(|&number| {
        index += 1;
        spoken[number as usize] = index;
        current = number;
    });

    (index..n).for_each(|turn| {
        if let Some(last) = spoken.get_mut(current as usize) {
            let spoken_at = *last;
            current = if spoken_at != 0 { turn - spoken_at } else { 0 };
            *last = turn;
        }
    });

    current
}

#[cfg(test)]
mod tests;
