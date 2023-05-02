use itertools::Itertools;

use commons::parse::LineSep;
use commons::{err, Report, Result};

pub const TITLE: &str = "Day 5: Binary Boarding";

pub fn run(raw: String) -> Result<()> {
    let data = parse(&raw)?;
    let max = first_part(&data.data).unwrap_or_default();
    println!("The maximum seat ID on the plane is {max}");

    let missing = second_part(data.data).unwrap_or_default();
    println!("The missing seat ID on the plane is {missing}");

    Ok(())
}

fn parse(s: &str) -> Result<LineSep<BoardingPass>> {
    s.parse()
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
        .tuple_windows::<(_, _)>()
        .find_map(|(current, next)| {
            if current.seat + 1 != next.seat {
                Some(current.seat + 1)
            } else {
                None
            }
        })
}

/// A boarding pass
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd)]
struct BoardingPass {
    seat: u16,
}

impl std::str::FromStr for BoardingPass {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 10 {
            return Err(err!(
                "The boarding pass should have a length of 10 characters, not {}",
                s.len()
            ));
        }

        let seat = s.chars().try_fold(0, |acc, c| {
            let bit = match c {
                'F' | 'L' => 0,
                'B' | 'R' => 1,
                _ => {
                    return Err(err!(
                        "The boarding pass should only contain be F, B, L or R, not {c}"
                    ))
                }
            };

            Ok(acc * 2 + bit)
        })?;

        Ok(BoardingPass { seat })
    }
}

#[cfg(test)]
mod tests;
