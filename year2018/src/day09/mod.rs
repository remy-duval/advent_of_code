use std::str::FromStr;

use color_eyre::eyre::{eyre, Report, Result, WrapErr};
use itertools::Itertools;

use commons::Problem;

mod ring;

pub struct Day;

impl Problem for Day {
    type Input = Rules;
    const TITLE: &'static str = "Day 9: Marble Mania";

    fn solve(mut rules: Self::Input) -> Result<()> {
        println!("Part 1: The winning score is {}", winning_score(&rules));
        rules.points *= 100;
        println!("Part 2: The winning score is {}", winning_score(&rules));

        Ok(())
    }
}

/// Play the game according to the given rules, returning the winning player score
fn winning_score(rules: &Rules) -> usize {
    ring::Ring::new(rules.points)
        .play(vec![0; rules.players], rules.points)
        .into_iter()
        .max()
        .unwrap_or_default()
}

/// The rules of the marble game
#[derive(Debug, Clone)]
pub struct Rules {
    players: usize,
    points: usize,
}

impl FromStr for Rules {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self> {
        fn parse_int(int: &str) -> Result<usize> {
            int.parse()
                .wrap_err_with(|| format!("Could not parse number {}", int))
        }

        let (players, points) = s
            .trim()
            .strip_suffix("points")
            .and_then(|s| {
                s.splitn(2, "players; last marble is worth")
                    .map(|part| parse_int(part.trim()))
                    .collect_tuple::<(_, _)>()
            })
            .ok_or_else(|| {
                eyre!(
                    "Expected '<PLAYERS> players; last marble is worth <POINTS> points', got {}",
                    s
                )
            })?;

        Ok(Self {
            players: players?,
            points: points?,
        })
    }
}

#[cfg(test)]
mod tests;
