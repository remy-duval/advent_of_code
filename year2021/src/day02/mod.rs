#![allow(unused)]

use color_eyre::eyre::{bail, Report, Result};
use commons::parse::LineSep;

use commons::Problem;

pub struct Day;

impl Problem for Day {
    type Input = LineSep<Command>;
    const TITLE: &'static str = "Day 2: Dive!";

    fn solve(data: Self::Input) -> Result<()> {
        println!("1. Horizontal x Depth = {}", first_part(&data.data));
        println!("2. Horizontal x Depth = {}", second_part(&data.data));

        Ok(())
    }
}

#[derive(Copy, Clone)]
pub enum Command {
    Forward(u8),
    Up(u8),
    Down(u8),
}

impl std::str::FromStr for Command {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self> {
        let command = match s.split_once(' ') {
            Some(("forward", amount)) => Command::Forward(amount.parse()?),
            Some(("up", amount)) => Command::Up(amount.parse()?),
            Some(("down", amount)) => Command::Down(amount.parse()?),
            _ => bail!("unknown command {}", s),
        };

        Ok(command)
    }
}

fn first_part(commands: &[Command]) -> u32 {
    let mut horizontal = 0;
    let mut depth = 0;
    for command in commands {
        match *command {
            Command::Forward(forward) => horizontal += forward as u32,
            Command::Up(up) => depth -= up as u32,
            Command::Down(down) => depth += down as u32,
        }
    }

    horizontal * depth
}

fn second_part(commands: &[Command]) -> u32 {
    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;
    for command in commands {
        match *command {
            Command::Forward(forward) => {
                let forward = forward as u32;
                horizontal += forward;
                depth += (forward * aim);
            }
            Command::Up(up) => aim -= up as u32,
            Command::Down(down) => aim += down as u32,
        }
    }

    horizontal * depth
}

#[cfg(test)]
mod tests;
