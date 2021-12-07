use commons::eyre::Result;
use commons::parse::CommaSep;
use commons::Problem;

pub struct Day;

impl Problem for Day {
    type Input = CommaSep<u8>;
    const TITLE: &'static str = "Day 6: Lanternfish";

    fn solve(data: Self::Input) -> Result<()> {
        println!("1. After 80 days: {} fish", first_part(&data.data));
        println!("2. After 256 days: {} fish", second_part(&data.data));

        Ok(())
    }
}

fn first_part(initial: &[u8]) -> usize {
    simulate(initial, 80)
}

fn second_part(initial: &[u8]) -> usize {
    simulate(initial, 256)
}

/// Simulate the fish population by grouping them by reproduction timer:
///
/// Every day:
/// * Fish with a timer of 0 reset to 6 and create a new fish with a timer of 8
/// * The timer of all the other fish is reduced by 1
///
/// ### Params
/// * `initial` - The initial fish population. One entry per fish, with its initial timer
/// * `days` - The number of days to run the simulation for
///
/// ### Returns
/// The number of fish alive after the given number of days
fn simulate(initial: &[u8], days: usize) -> usize {
    // The index of the array is the remaining timer for the fish count that is the value
    let mut first = [0; 9];
    initial.iter().for_each(|i| first[*i as usize] += 1);

    (0..days)
        .fold(first, |previous, _| {
            let mut current = [0; 9];
            // 0 -> reproduce, becoming 6, with their offspring becoming 8
            current[6] += previous[0];
            current[8] += previous[0];
            // > 0 -> reduce by 1
            (1..9usize).for_each(|i| current[i - 1] += previous[i]);
            current
        })
        .into_iter()
        .sum()
}

#[cfg(test)]
mod tests;
