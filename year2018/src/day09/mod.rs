use std::str::FromStr;

use itertools::Itertools;

use commons::Problem;

mod ring;

pub struct Day;

impl Problem for Day {
    type Input = Rules;
    type Err = std::convert::Infallible;
    const TITLE: &'static str = "Day 9: Marble Mania";

    fn solve(mut rules: Self::Input) -> Result<(), Self::Err> {
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

/// An error that happens when parsing the rules of the game
#[derive(Debug, thiserror::Error)]
pub enum RulesParseError {
    #[error("Expected '<PLAYERS> players; last marble is worth <POINTS> points', got {0}")]
    BadFormat(Box<str>),
    #[error("Could not parse number {0} ({1})")]
    ParseIntError(Box<str>, std::num::ParseIntError),
}

impl FromStr for Rules {
    type Err = RulesParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_int(int: &str) -> Result<usize, RulesParseError> {
            int.parse()
                .map_err(|e| RulesParseError::ParseIntError(int.into(), e))
        }

        let (players, points) = s
            .trim()
            .strip_suffix("points")
            .and_then(|s| {
                s.splitn(2, "players; last marble is worth")
                    .map(|part| parse_int(part.trim()))
                    .collect_tuple::<(_, _)>()
            })
            .ok_or_else(|| RulesParseError::BadFormat(s.into()))?;

        Ok(Self {
            players: players?,
            points: points?,
        })
    }
}

#[cfg(test)]
mod tests;
