use itertools::Itertools;

use commons::eyre::{eyre, Result, WrapErr};

mod ring;

pub const TITLE: &str = "Day 9: Marble Mania";

pub fn run(raw: String) -> Result<()> {
    let mut rules = parse(&raw)?;
    println!("Part 1: The winning score is {}", winning_score(&rules));
    rules.points *= 100;
    println!("Part 2: The winning score is {}", winning_score(&rules));
    Ok(())
}

fn parse(s: &str) -> Result<Rules> {
    fn parse_int(int: &str) -> Result<usize> {
        int.parse()
            .wrap_err_with(|| format!("Could not parse number {int}"))
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
            eyre!("Expected '<PLAYERS> players; last marble is worth <POINTS> points', got {s}")
        })?;

    Ok(Rules {
        players: players?,
        points: points?,
    })
}

/// The rules of the marble game
struct Rules {
    players: usize,
    points: usize,
}

/// Play the game according to the given rules, returning the winning player score
fn winning_score(rules: &Rules) -> usize {
    ring::Ring::new(rules.points)
        .play(vec![0; rules.players], rules.points)
        .into_iter()
        .max()
        .unwrap_or_default()
}

#[cfg(test)]
mod tests;
