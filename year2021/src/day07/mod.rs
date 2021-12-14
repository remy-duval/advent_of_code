use commons::eyre::Result;
use commons::parse::CommaSep;

pub const TITLE: &str = "Day 7: The Treachery of Whales";

pub fn run(raw: String) -> Result<()> {
    let mut data = parse(&raw)?;
    println!("1. Linear fuel cost {}", first_part(&mut data.data));
    println!("2. Quadratic fuel cost {}", second_part(&data.data));
    Ok(())
}

fn parse(s: &str) -> Result<CommaSep<i32>> {
    Ok(s.parse()?)
}

/// Find the minimum sum of distances to a central point where the distance is linear
fn first_part(points: &mut [i32]) -> i32 {
    if points.is_empty() {
        return 0;
    }

    // Find the approximation of the median point, which minimizes the distance of all points to it
    points.sort_unstable();
    let middle = points.len() / 2;
    let median = if points.len() % 2 == 1 {
        (points[middle / 2] + points[middle + 1]) / 2
    } else {
        points[middle]
    };

    improve_min_cost(median, |center| {
        points.iter().fold(0, |acc, p| acc + (center - *p).abs())
    })
}

/// Find the minimum sum of distance to a central point where the distance is (n + 1) * n / 2
fn second_part(points: &[i32]) -> i32 {
    if points.is_empty() {
        return 0;
    }

    // The mean is a good starting approximation
    let mean = points.iter().sum::<i32>() / points.len() as i32;
    improve_min_cost(mean, |center| {
        points.iter().fold(0, |acc, p| {
            let distance = (center - *p).abs();
            acc + distance * (distance + 1) / 2
        })
    })
}

/// Improve the given approximation of the min cost by looking around it
fn improve_min_cost<Cost: Fn(i32) -> i32>(approximation: i32, cost: Cost) -> i32 {
    // Search in the given direction to minimize the cost
    // Stop as soon the the cost is no longer decreasing (assuming the local min is the global one)
    let search = |mut current, step| {
        let mut min = i32::MAX;
        loop {
            let tentative = cost(current);
            if min < tentative {
                break min;
            }
            min = tentative;
            current += step;
        }
    };

    // Find the minimum in both directions around the central point
    search(approximation, 1).min(search(approximation - 1, -1))
}

#[cfg(test)]
mod tests;
