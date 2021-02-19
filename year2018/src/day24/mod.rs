use commons::Problem;

mod data;

pub struct Day;

impl Problem for Day {
    type Input = data::Battle;
    type Err = std::convert::Infallible;
    const TITLE: &'static str = "Day 24: Immune System Simulator 20XX";

    fn solve(data: Self::Input) -> Result<(), Self::Err> {
        println!(
            "The winning army ends with {} units",
            first_part(data.clone())
        );
        println!(
            "The least amount of units the immune system has after winning is {}",
            second_part(data)
        );
        Ok(())
    }
}

/// First part: execute the full battle with no changes
fn first_part(mut battle: data::Battle) -> data::Int {
    battle.fight()
}

/// Second part: boost the immune system in battle to find the smallest boost to get it to win
fn second_part(battle: data::Battle) -> data::Int {
    let immune_system_remaining = |boost: data::Int| {
        let mut battle = battle.with_boosted_immune_system(boost);
        let remaining = battle.fight();
        if battle.has_immune_system_won() {
            remaining
        } else {
            0 // Can be a tie or a defeat
        }
    };

    // Binary search for the minimum non-zero amount of immune system units remaining
    let mut base = 0;
    let mut size = battle.infection_max_hp(); // Killing any enemy unit in 1 hit is a good max
    let mut min = data::Int::MAX;
    while size > 1 {
        let half = size / 2;
        size -= half;

        let current = immune_system_remaining(base + half);
        if current > 0 {
            min = min.min(current);
        } else {
            base += half;
        }
    }

    min
}

#[cfg(test)]
mod tests;
