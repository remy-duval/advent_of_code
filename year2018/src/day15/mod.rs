use commons::eyre::{eyre, Result};

mod data;

pub const TITLE: &str = "Day 15: Beverage Bandits";

pub fn run(raw: String) -> Result<()> {
    let fight = parse(&raw)?;
    let (remaining, hp) = fight.clone().first_part();
    println!("The fight finishes with an outcome of {}", remaining * hp);

    let (remaining, hp) = fight
        .second_part()
        .ok_or_else(|| eyre!("Didn't find an outcome where the elves won without casualties"))?;
    println!("The elves win with an outcome of {}", remaining * hp);

    Ok(())
}

fn parse(s: &str) -> Result<data::Fight> {
    Ok(s.parse()?)
}

#[cfg(test)]
mod tests;
