use commons::parse::LineSep;
use commons::Result;

mod input;
mod partition;

pub const TITLE: &str = "Day 23: Experimental Emergency Teleportation";

pub fn run(raw: String) -> Result<()> {
    let bots = parse(&raw)?;
    println!(
        "The bots with the largest radius has {} bots in range",
        first_part(&bots.data)
    );

    println!(
        "The point in range to the most bots is {} units away from the center",
        second_part(&bots.data)
    );

    Ok(())
}

fn parse(s: &str) -> Result<LineSep<input::Bot>> {
    s.parse()
}

/// Find the bot with the largest radius, compute the number of bots within its range (itself too)
fn first_part(bots: &[input::Bot]) -> usize {
    bots.iter()
        .max_by_key(|bot| bot.r)
        .map(|bot| bots.iter().filter(|&other| bot.can_reach(other)).count())
        .unwrap_or_default()
}

/// Find the point in range with the most bot and closest to the origin
fn second_part(bots: &[input::Bot]) -> input::Dimension {
    partition::partition(bots).map_or(0, |point| point.origin_distance())
}

#[cfg(test)]
mod tests;
