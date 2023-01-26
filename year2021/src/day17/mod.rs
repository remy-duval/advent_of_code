use commons::eyre::{eyre, Result, WrapErr};

pub const TITLE: &str = "Day 17: Trick Shot";

pub fn run(raw: String) -> Result<()> {
    let data = parse(&raw)?;
    println!("1. Max shot height: {}", first_part(&data));
    println!("2. Possible shots: {}", second_part(data));

    Ok(())
}

struct Target {
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
}

fn parse(s: &str) -> Result<Target> {
    let ((min_x, max_x), (min_y, max_y)) = s
        .trim()
        .strip_prefix("target area: x=")
        .and_then(|s| {
            let (x, y) = s.split_once(", y=")?;
            Some((x.split_once("..")?, y.split_once("..")?))
        })
        .ok_or_else(|| eyre!("Can't find target string known delimiters '{}'", s))?;

    Ok(Target {
        min_x: min_x.parse().wrap_err_with(|| format!("For '{min_x}'"))?,
        max_x: max_x.parse().wrap_err_with(|| format!("For '{max_x}'"))?,
        min_y: min_y.parse().wrap_err_with(|| format!("For '{min_y}'"))?,
        max_y: max_y.parse().wrap_err_with(|| format!("For '{max_y}'"))?,
    })
}

/// Find the highest height that can be reached while reaching the target at the end
fn first_part(target: &Target) -> i32 {
    // We need an upper bound to limit the search on Y
    let max_y = target.max_y.abs().max(target.min_y.abs()) + 10;

    // The maximum height will be reached on a case where the target is reached vertically
    // This means we can ignore X completely for this part
    (1..max_y)
        .filter_map(|y| {
            if valid_y(y, (y, i32::MAX), target.min_y, target.max_y) {
                Some(((y + 1) * y) / 2)
            } else {
                None
            }
        })
        .max()
        .unwrap_or_default()
}

/// Count all possible X, Y velocities that will reach the target
fn second_part(target: Target) -> usize {
    // We need upper and lower bound to limit the search on Y
    let min_y = target.max_y.min(target.min_y);
    let max_y = target.max_y.abs().max(target.min_y.abs()) + 10;

    // For X velocity that can reach the target, find Y velocities that reach it at the same time
    (1..=target.max_x)
        .filter_map(|x| valid_x_range(x, target.min_x, target.max_x))
        .map(|range| {
            (min_y..max_y)
                .filter(|&y| valid_y(y, range, target.min_y, target.max_y))
                .count()
        })
        .sum()
}

/// Find the steps range at which the given initial X velocity is in the target
fn valid_x_range(x: i32, min: i32, max: i32) -> Option<(i32, i32)> {
    let mut current = (x * (x + 1)) / 2;
    let mut min_max = None;
    let mut steps = x;
    let mut backwards = 0;
    while current >= min {
        if current <= max {
            min_max = match min_max {
                Some((_, max)) => Some((steps, max)),
                None => Some((steps, if steps == x { i32::MAX } else { steps })),
            };
        }

        backwards -= 1;
        steps -= 1;
        current += backwards;
    }

    min_max
}

/// Check whether this initial Y velocity can reach the target in the given steps
fn valid_y(y: i32, steps: (i32, i32), min: i32, max: i32) -> bool {
    // Compute the Y value at the start of the range using gauss formula
    let mut step = steps.0;
    let mut current = (step * (2 * y - step + 1)) / 2;
    let mut velocity = y - step;
    // Then check if there is any point in the range where it can fit in the target
    loop {
        if current < min || step > steps.1 {
            return false;
        } else if current <= max {
            return true;
        }

        current += velocity;
        velocity -= 1;
        step += 1;
    }
}

#[cfg(test)]
mod tests;
