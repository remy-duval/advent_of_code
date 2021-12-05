#![allow(unused)]

use commons::eyre::{eyre, Result};
use commons::Problem;

pub struct Day;

impl Problem for Day {
    type Input = String;
    const TITLE: &'static str = "";

    fn solve(data: Self::Input) -> Result<()> {
        Err(eyre!("TODO"))
    }
}

#[cfg(test)]
mod tests;
