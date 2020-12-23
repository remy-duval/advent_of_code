use std::convert::TryInto;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str::FromStr;

use itertools::Itertools;

use crate::Problem;

pub struct Day;

impl Problem for Day {
    type Input = TenCups;
    type Err = std::convert::Infallible;
    const TITLE: &'static str = "Day 23: Crab Cups";

    fn solve(cups: Self::Input) -> Result<(), Self::Err> {
        println!(
            "Part 1: The order after 100 moves is '{}'",
            first_part(&cups)
        );

        let (first, second) = second_part(&cups);
        println!(
            "Part 2: The cups to the right of 1 are {} and {}, with a product of {}",
            first,
            second,
            first * second
        );

        Ok(())
    }
}

fn first_part(cups: &TenCups) -> String {
    let mut cups = CupRing::new(cups, 9);
    cups.nth(100);
    cups.to_string()
}

fn second_part(cups: &TenCups) -> (usize, usize) {
    let mut cups = CupRing::new(cups, 1_000_000);
    cups.nth(10_000_000);
    let first = cups.storage[0];
    let second = cups.storage[first];

    // Increment by one since we store the cup indexed from 0 where the problem indexes from 1
    (first + 1, second + 1)
}

/// The input cups arrangement
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct TenCups([usize; 9]);

#[derive(Debug, thiserror::Error)]
pub enum CupParseError {
    #[error("'{0}' is not a valid base 10 digit")]
    BadDigit(char),
    #[error("Expected 9 cups, got {0} instead")]
    BadLength(usize),
}

impl FromStr for TenCups {
    type Err = CupParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let digits: [usize; 9] = s
            .trim()
            .chars()
            .map(|c| {
                c.to_digit(10)
                    .map_or_else(|| Err(CupParseError::BadDigit(c)), |ok| Ok(ok as usize - 1))
            })
            .collect::<Result<Vec<usize>, CupParseError>>()
            .and_then(|digits| {
                let digits: Result<[usize; 9], Vec<usize>> = digits.try_into();
                digits.map_err(|vec| CupParseError::BadLength(vec.len()))
            })?;

        Ok(Self(digits))
    }
}

/// The representation of the cup ring in the problem
///
/// ### Optimization
/// Instead of the linked list implementation used at first, use an array of neighbours:
/// * Each cup is stored at its label - 1 (so cup 1 is 0)
/// * The value is the index (label - 1) of its clock-wise neighbour
/// * So each move is pretty cheap (just swap three values in the storage)
/// * Retrieving the cups clockwise from 1 is pretty cheap too
///
/// Much thanks to the solution thread of r/adventofcode/ for this very nice idea
#[derive(Debug, Clone)]
struct CupRing {
    current: usize,
    storage: Vec<usize>,
}

impl CupRing {
    /// Create a new cup ring
    ///
    /// ### Arguments
    /// * `cups` - The original cups
    /// * `width` - The amount of cups in total
    fn new(cups: &TenCups, width: usize) -> Self {
        assert!(
            width >= 9,
            "The width should be at least the ten starting cups"
        );

        let mut storage = vec![0; width];
        cups.0
            .iter()
            .copied() // The 9 first cup are in the given order
            .chain(cups.0.len()..width) // Then growing from 10 to width
            .chain(std::iter::once(cups.0[0])) // Loop to start
            .tuple_windows::<(_, _)>()
            .for_each(|(from, to)| storage[from] = to);

        Self {
            current: cups.0[0],
            storage,
        }
    }

    /// Move the cup ring to the state after the n next moves (0 is NoOp)
    fn nth(&mut self, n: usize) {
        (0..n).for_each(|_| self.next());
    }

    /// Move the cup ring to the state after the next move
    fn next(&mut self) {
        // The current cup and the next three that will be moved
        let current = self.current;
        let one = self.storage[current];
        let two = self.storage[one];
        let three = self.storage[two];

        let destination = {
            let mut next = self.wrapping_decrement(current);
            while next == one || next == two || next == three {
                next = self.wrapping_decrement(next);
            }

            next
        };

        // Swap the elements around
        let next_after = self.storage[destination];
        self.storage[current] = self.storage[three];
        self.storage[destination] = one;
        self.storage[three] = next_after;

        // Set the next current cup (the one right after the current one in the ring)
        self.current = self.storage[current];
    }

    /// Decrement an index by 1 if not 0, else jump to maximum valid index
    fn wrapping_decrement(&self, index: usize) -> usize {
        if index == 0 {
            self.storage.len() - 1
        } else {
            index - 1
        }
    }
}

impl Display for CupRing {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let mut current = 0;
        for _ in 0..8 {
            current = self.storage[current];
            // +1 since we store the cup indexed from 0 where the problem indexes from 1
            write!(f, "{}", current + 1)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("test_resources/23-example.txt");
    const MAIN: &str = include_str!("test_resources/23-main.txt");

    #[test]
    fn first_part_example_a() {
        let cups = Day::parse(EXAMPLE).unwrap();
        let mut ring = CupRing::new(&cups, 9);
        ring.nth(10);
        assert_eq!(ring.to_string(), "92658374");
    }

    #[test]
    fn first_part_example_b() {
        let cups = Day::parse(EXAMPLE).unwrap();
        let result = first_part(&cups);
        assert_eq!(result, "67384529");
    }

    #[test]
    fn first_part_main() {
        let cups = Day::parse(MAIN).unwrap();
        let result = first_part(&cups);
        assert_eq!(result, "54327968");
    }

    #[test]
    fn second_part_example() {
        let cups = Day::parse(EXAMPLE).unwrap();
        let (first, second) = second_part(&cups);
        assert_eq!(first * second, 149_245_887_792);
    }

    #[test]
    fn second_part_main() {
        let cups = Day::parse(MAIN).unwrap();
        let (first, second) = second_part(&cups);
        assert_eq!(first * second, 157_410_423_276);
    }
}
