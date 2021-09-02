use std::str::FromStr;

use color_eyre::eyre::{bail, ensure, Report, Result};
use itertools::Itertools;

use commons::parse::LineSep;
use commons::Problem;

pub struct Day;

impl Problem for Day {
    type Input = LineSep<BoardingPass>;
    const TITLE: &'static str = "Day 5: Binary Boarding";

    fn solve(data: Self::Input) -> Result<()> {
        let max = first_part(&data.data).unwrap_or_default();
        println!("The maximum seat ID on the plane is {max}", max = max);

        let missing = second_part(data.data).unwrap_or_default();
        println!(
            "The missing seat ID on the plane is {missing}",
            missing = missing
        );

        Ok(())
    }
}

/// Find the maximum seat id on the plane
fn first_part(passes: &[BoardingPass]) -> Option<u16> {
    passes.iter().max().map(|pass| pass.seat)
}

/// Find the missing seat id on the plane
fn second_part(mut passes: Vec<BoardingPass>) -> Option<u16> {
    passes.sort();
    passes
        .into_iter()
        .tuple_windows::<(BoardingPass, BoardingPass)>()
        .find_map(|(current, next)| {
            if current.seat + 1 != next.seat {
                Some(current.seat + 1)
            } else {
                None
            }
        })
}

/// A boarding pass
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct BoardingPass {
    seat: u16,
}

impl FromStr for BoardingPass {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ensure!(
            s.len() == 10,
            "The boarding pass should have a length of 10 characters, not {}",
            s.len()
        );

        let seat = s.chars().try_fold(0, |acc, c| {
            let bit = match c {
                'F' | 'L' => 0,
                'B' | 'R' => 1,
                _ => bail!(
                    "The boarding pass should only contain be F, B, L or R, not {}",
                    c
                ),
            };

            Ok(acc * 2 + bit)
        })?;

        Ok(BoardingPass { seat })
    }
}

#[cfg(test)]
mod tests;
