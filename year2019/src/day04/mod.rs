use std::str::FromStr;

use color_eyre::eyre::{eyre, Report, Result};
use itertools::Itertools;

use commons::Problem;

pub struct Day;

impl Problem for Day {
    type Input = Between;
    const TITLE: &'static str = "Day 4: Secure Container";

    fn solve(data: Self::Input) -> Result<()> {
        let Between { from, to } = data;
        println!("Range is {}..{}", from, to);

        let (first, second) = solve(from, to);
        println!("The number of valid possibilities is {}", first);
        println!(
            "The number of valid possibilities with increased strictness is {}",
            second
        );

        Ok(())
    }
}

pub struct Between {
    from: i32,
    to: i32,
}

impl FromStr for Between {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((a, b)) = s.split('-').collect_tuple() {
            let from: i32 = a.parse()?;
            let to: i32 = b.parse()?;
            if from >= to {
                Err(eyre!("{} >= {}", from, to))
            } else {
                Ok(Between { from, to })
            }
        } else {
            Err(eyre!("Didn't find the lower and higher bound in {}", s))
        }
    }
}

fn solve(start: i32, end: i32) -> (i32, i32) {
    let mut count_lenient = 0;
    let mut count_strict = 0;
    for int in start..end {
        let digits = split_digits(int);
        if check_ordered(digits) {
            let (lenient, strict) = check_pair(&digits);
            if lenient {
                count_lenient += 1;
            }
            if strict {
                count_strict += 1;
            }
        }
    }

    (count_lenient, count_strict)
}

/// Splits an integer into its 6 first digit (each is < 10)
fn split_digits(int: i32) -> [u8; 6] {
    let mut rest = int;
    let mut acc = [0u8; 6];
    for elt in acc.iter_mut().rev() {
        *elt = (rest % 10) as u8;
        rest /= 10;
    }

    acc
}

/// Checks that the given digits are ordered (first condition).
fn check_ordered(digits: [u8; 6]) -> bool {
    let mut prev = u8::MIN;
    for digit in digits.iter() {
        if *digit < prev {
            return false;
        }
        prev = *digit;
    }

    true
}

/// Checks that the given digits contains at least one group of matching following digits
/// # Returns
/// Second condition (lenient form => at least 2 same followed digits)
/// Second condition (strict form => exactly 2 same followed digits)
fn check_pair(digits: &[u8]) -> (bool, bool) {
    let mut digit_count = [0u8; 10];
    for dig in digits {
        digit_count[*dig as usize] += 1;
    }

    let strict = digit_count.iter().any(|x| *x == 2);
    if strict {
        (true, true)
    } else {
        (digit_count.iter().any(|x| *x >= 2), false)
    }
}

#[cfg(test)]
mod tests;
