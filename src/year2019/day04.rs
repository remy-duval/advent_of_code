use std::str::FromStr;

use itertools::Itertools;
use anyhow::anyhow;

pub struct Day04;

impl crate::Problem for Day04 {
    type Input = Between;
    type Err = anyhow::Error;
    const TITLE: &'static str = "Day 4: Secure Container";

    fn solve(data: Self::Input) -> Result<(), Self::Err> {
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
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((a, b)) = s.split('-').collect_tuple() {
            let from: i32 = a.parse()?;
            let to: i32 = b.parse()?;
            if from >= to {
                Err(anyhow!("{} >= {}",from, to))
            } else {
                Ok(Between { from, to })
            }
        } else {
            Err(anyhow!("Didn't find the lower and higher bound in {}", s))
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
    let mut prev = std::u8::MIN;
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
mod tests {
    use super::*;

    const DATA: &str = include_str!("test_resources/day04.txt");

    #[test]
    fn digits() {
        assert_eq!(&[1, 1, 1, 1, 1, 1], &split_digits(111_111));
        assert_eq!(&[2, 2, 3, 4, 5, 0], &split_digits(223_450));
        assert_eq!(&[1, 2, 3, 7, 8, 9], &split_digits(123_789));
    }

    #[test]
    fn ordered() {
        assert!(!check_ordered([2, 2, 3, 4, 5, 0]));
        assert!(check_ordered([1, 1, 1, 1, 1, 1]));
        assert!(check_ordered([1, 2, 3, 7, 8, 9]));
    }

    #[test]
    fn pairs() {
        assert_eq!((false, false), check_pair(&[1, 2, 3, 5, 6, 7]));
        assert_eq!((true, false), check_pair(&[1, 3, 3, 3, 6, 7]));
        assert_eq!((true, true), check_pair(&[1, 3, 3, 3, 6, 6]));
    }

    #[test]
    fn solve_test() {
        let Between { from, to } = DATA.parse().unwrap();
        let (first, second) = solve(from, to);

        assert_eq!(1_653, first);
        assert_eq!(1_133, second);
    }
}
