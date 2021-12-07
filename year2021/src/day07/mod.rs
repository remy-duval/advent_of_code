use commons::eyre::{ensure, Result};
use commons::parse::CommaSep;
use commons::Problem;

pub struct Day;

impl Problem for Day {
    type Input = CommaSep<i32>;
    const TITLE: &'static str = "Day 7: The Treachery of Whales";

    fn solve(mut data: Self::Input) -> Result<()> {
        println!("1. Linear fuel cost {}", first_part(&mut data.data)?);
        println!("1. Quadratic fuel cost {}", second_part(&mut data.data)?);

        Ok(())
    }
}

/// Find the minimum sum of distances to a central point where the distance is linear
fn first_part(points: &mut [i32]) -> Result<i32> {
    ensure!(!points.is_empty(), "Empty points for computing the mean");

    // Find the median point, which minimizes the distance of all points to it
    points.sort_unstable();
    let middle = points.len() / 2;
    let median = if points.len() % 2 == 1 {
        (points[middle / 2] + points[middle + 1]) / 2
    } else {
        points[middle]
    };

    // Then compute said sum of distance to the mean point
    Ok(points.iter().map(|&p| (p - median).abs()).sum())
}

/// Find the minimum sum of distance to a central point where the distance is (n + 1) * n / 2
fn second_part(points: &[i32]) -> Result<i32> {
    // Total fuel cost for a move from the given points to the center
    fn cost(center: i32, points: &[i32]) -> i32 {
        points
            .iter()
            .map(|p| {
                let distance = (*p - center).abs();
                distance * (distance + 1) / 2
            })
            .sum()
    }

    // Search for the minimum cost from the given point
    // Stop as soon as the costs are not diminishing
    // This should work since the cost function is an upside-down parabola
    fn search(mut current: i32, step: i32, points: &[i32]) -> i32 {
        let mut min = i32::MAX;
        loop {
            let tentative = cost(current, points);
            if min < tentative {
                break min;
            }
            min = tentative;
            current += step;
        }
    }

    ensure!(!points.is_empty(), "Empty points for computing the mean");
    let mean = points.iter().sum::<i32>() / points.len() as i32;
    Ok(search(mean, 1, points).min(search(mean - 1, -1, points)))
}

#[cfg(test)]
mod tests;
