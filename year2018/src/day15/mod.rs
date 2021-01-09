#![allow(unused)]

use commons::Problem;

use crate::points::Point;

mod data;

pub struct Day;

impl Problem for Day {
    type Input = data::Fight;
    type Err = anyhow::Error;
    const TITLE: &'static str = "Day 15: Beverage Bandits";

    fn solve(fight: Self::Input) -> Result<(), Self::Err> {
        let (remaining, hp) = fight.clone().first_part();
        println!("The fight finishes with an outcome of {}", remaining * hp);

        let (remaining, hp) = fight.second_part().ok_or_else(|| {
            anyhow::anyhow!("Didn't find an outcome where the elves won without casualties")
        })?;
        println!("The elves win with an outcome of {}", remaining * hp);

        Ok(())
    }
}

#[cfg(test)]
mod tests;
