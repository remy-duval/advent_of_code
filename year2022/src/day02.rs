use std::str::FromStr;

use commons::err;
use commons::error::Result;

pub const TITLE: &str = "Day 2: Rock Paper Scissors";

pub fn run(raw: String) -> Result<()> {
    let data = parse(raw.into())?;
    let first = first_part(&data);
    println!("1. Total score when interpreting second column as a shape is {first}");
    let second = second_part(&data);
    println!("2. Total score when interpreting second column as a result is {second}");

    Ok(())
}

fn first_part(input: &[(Shape, Game)]) -> u64 {
    input
        .iter()
        .map(|&(opponent, game)| {
            let player = match game {
                Game::Lose => Shape::Rock,
                Game::Draw => Shape::Paper,
                Game::Win => Shape::Scissors,
            };

            let result = if opponent.defeats() == player {
                Game::Lose
            } else if opponent.defeated() == player {
                Game::Win
            } else {
                Game::Draw
            };

            player.score(result)
        })
        .sum()
}

fn second_part(input: &[(Shape, Game)]) -> u64 {
    input
        .iter()
        .map(|&(opponent, result)| {
            let player = match result {
                Game::Draw => opponent,
                Game::Lose => opponent.defeats(),
                Game::Win => opponent.defeated(),
            };
            player.score(result)
        })
        .sum()
}

#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(u8)]
enum Shape {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl From<u8> for Shape {
    fn from(value: u8) -> Self {
        match value {
            1 => Shape::Rock,
            2 => Shape::Paper,
            _ => Shape::Scissors,
        }
    }
}

impl Shape {
    fn defeats(self) -> Self {
        ((self as u8 + 2) % 3).into()
    }

    fn defeated(self) -> Self {
        ((self as u8 + 1) % 3).into()
    }

    fn score(self, game: Game) -> u64 {
        self as u64 + game as u64
    }
}

#[derive(Copy, Clone)]
#[repr(u8)]
enum Game {
    Lose = 0,
    Draw = 3,
    Win = 6,
}

impl FromStr for Shape {
    type Err = commons::error::Report;

    fn from_str(s: &str) -> Result<Self> {
        match s.chars().next() {
            Some('A') => Ok(Self::Rock),
            Some('B') => Ok(Self::Paper),
            Some('C') => Ok(Self::Scissors),
            _ => Err(err!("unknown first value {s}")),
        }
    }
}

impl FromStr for Game {
    type Err = commons::error::Report;

    fn from_str(s: &str) -> Result<Self> {
        match s.chars().next() {
            Some('X') => Ok(Self::Lose),
            Some('Y') => Ok(Self::Draw),
            Some('Z') => Ok(Self::Win),
            _ => Err(err!("unknown second value {s}")),
        }
    }
}

fn parse(s: std::borrow::Cow<'static, str>) -> Result<Vec<(Shape, Game)>> {
    s.lines()
        .filter_map(|line| line.trim().split_once(' '))
        .map(|(a, b)| -> Result<(Shape, Game)> { Ok((a.parse()?, b.parse()?)) })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../examples/02.txt");
    const MAIN: &str = include_str!("../inputs/02.txt");

    #[test]
    fn first_part_example() {
        let data = parse(EXAMPLE.into()).unwrap();
        assert_eq!(first_part(&data), 15);
    }

    #[test]
    fn first_part_main() {
        let data = parse(MAIN.into()).unwrap();
        assert_eq!(first_part(&data), 14827);
    }

    #[test]
    fn second_part_example() {
        let data = parse(EXAMPLE.into()).unwrap();
        assert_eq!(second_part(&data), 12);
    }

    #[test]
    fn second_part_main() {
        let data = parse(MAIN.into()).unwrap();
        assert_eq!(second_part(&data), 13889);
    }
}
