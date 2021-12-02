use color_eyre::eyre::Result;

use commons::parse::LineSep;
use commons::Problem;

pub struct Day;

impl Problem for Day {
    type Input = LineSep<i32>;
    const TITLE: &'static str = "Day 1: Sonar Sweep";

    fn solve(data: Self::Input) -> Result<()> {
        println!(
            "1. {} measurements larger than the previous one",
            first_part(&data.data)
        );

        println!(
            "2. {} three sums measurements larger than the previous one",
            second_part(&data.data)
        );

        Ok(())
    }
}

fn first_part(measurements: &[i32]) -> usize {
    increases(measurements.iter().copied())
}

fn second_part(measurements: &[i32]) -> usize {
    increases(measurements.windows(3).map(|w| w.iter().sum()))
}

fn increases(mut measurements: impl Iterator<Item = i32>) -> usize {
    if let Some(mut previous) = measurements.next() {
        let mut count = 0usize;
        for next in measurements {
            if previous < next {
                count += 1;
            }
            previous = next;
        }
        count
    } else {
        0
    }
}

#[cfg(test)]
mod tests;
