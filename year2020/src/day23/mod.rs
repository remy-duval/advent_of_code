use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str::FromStr;

use itertools::Itertools;

use commons::Problem;

pub struct Day;

impl Problem for Day {
    type Input = TenCups;
    type Err = std::convert::Infallible;
    const TITLE: &'static str = "Day 23: Crab Cups";

    fn solve(cups: Self::Input) -> Result<(), Self::Err> {
        println!(
            "Part 1: The order after 100 moves is '{}'",
            first_part(&cups.0)
        );

        let (first, second) = second_part(&cups.0);
        println!(
            "Part 2: The cups to the right of 1 are {} and {}, with a product of {}",
            first,
            second,
            first * second
        );

        Ok(())
    }
}

fn first_part(cups: &[usize]) -> String {
    let mut cups = CupRing::new(cups, 9);
    cups.nth(100);
    cups.to_string()
}

fn second_part(cups: &[usize]) -> (usize, usize) {
    let mut cups = CupRing::new(cups, 1_000_000);
    cups.nth(10_000_000);
    let first = cups.storage[0];
    let second = cups.storage[first];

    // Increment by one since we store the cup indexed from 0 where the problem indexes from 1
    (first + 1, second + 1)
}

/// The input cups arrangement
#[derive(Debug, Clone)]
pub struct TenCups(Vec<usize>);

/// An error when parsing cups labels
#[derive(Debug, thiserror::Error)]
#[error("'{0}' is not a valid base 10 digit")]
pub struct CupParseError(char);

impl FromStr for TenCups {
    type Err = CupParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.trim()
                .chars()
                .map(|c| {
                    c.to_digit(10)
                        .map_or_else(|| Err(CupParseError(c)), |ok| Ok(ok as usize - 1))
                })
                .collect::<Result<Vec<usize>, CupParseError>>()?,
        ))
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
    fn new(cups: &[usize], width: usize) -> Self {
        assert!(!cups.is_empty(), "There should be at least one cup");
        assert!(
            width >= cups.len(),
            "The width should be at least the ten starting cups"
        );

        let mut storage = vec![0; width];
        cups.iter()
            .copied() // The 9 first cup are in the given order
            .chain(cups.len()..width) // Then growing from 10 to width
            .chain(std::iter::once(cups[0])) // Loop to start
            .tuple_windows::<(_, _)>()
            .for_each(|(from, to)| storage[from] = to);

        Self {
            current: cups[0],
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
        self.storage[current] = self.storage[three];
        self.storage[three] = self.storage[destination];
        self.storage[destination] = one;

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

/// Display the 8 cups after the Cup 1 (for part 1)
impl Display for CupRing {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let mut current = 0;
        (0..8).try_for_each(|_| {
            current = self.storage[current];
            // +1 since we store the cup indexed from 0 where the problem indexes from 1
            write!(f, "{}", current + 1)
        })
    }
}

#[cfg(test)]
mod tests;
