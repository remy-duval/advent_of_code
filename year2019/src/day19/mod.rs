use commons::eyre::Result;
use commons::grid::Point;

use super::int_code::{IntCodeInput, Processor};

pub const TITLE: &str = "Day 19: Tractor Beam";

pub fn run(raw: String) -> Result<()> {
    let memory = parse(&raw)?.data;
    // First part
    let affected = count_pulled(&memory, Point::new(0, 0), Point::new(50, 50));
    println!("{affected} tiles are affected by the beam in the (0,0) (49, 49) square");

    // Second part
    let first = find_first_square(&memory, 100);
    println!(
        "The first point for the square is {first}, with code {code}",
        code = first.x * 10_000 + first.y
    );

    Ok(())
}

fn parse(s: &str) -> Result<IntCodeInput> {
    Ok(s.parse()?)
}

/// Check a range of positive Points to get the number of affected tiles.
fn count_pulled(drone: &[i64], top: Point, bottom: Point) -> usize {
    let mut display: String = String::with_capacity((bottom - top).manhattan_distance() as usize);
    let affected: usize = (top.y..bottom.y)
        .map(|y| {
            display.push('\n');
            (top.x..bottom.x)
                .filter(|x| {
                    let point = Point::new(*x, y);
                    if check_position(drone, point) {
                        display.push('#');
                        true
                    } else {
                        display.push('.');
                        false
                    }
                })
                .count()
        })
        .sum();

    println!("{display}");
    affected
}

/// Find the first square with the given dimension that is fully affected by the beam
fn find_first_square(drone: &[i64], size: i64) -> Point {
    let diff: i64 = size - 1; // If the size is n, the difference in coordinates are n - 1
    let mut start = Point::new(0, size);

    loop {
        // Realign the point with the beam if it exited it.
        while !check_position(drone, start) {
            start = start + Point::new(1, 0);
        }

        // Check if we have a valid square (our position is OK, just need to check the top)
        if check_position(drone, start + Point::new(diff, -diff)) {
            return start + Point::new(0, -diff);
        }
        start = start + Point::new(0, 1);
    }
}

/// Check a position to get if it is currently in the beam or not
fn check_position(drone: &[i64], point: Point) -> bool {
    let mut drone: Processor = drone.into();
    drone.write_int(point.x.max(0));
    drone.write_int(point.y.max(0));

    match drone.read_next() {
        Ok(result) => result == 1,
        Err(status) => {
            println!("The drone should not stop, but we got status {status:?}");
            false
        }
    }
}

#[cfg(test)]
mod tests;
