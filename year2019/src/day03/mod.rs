use hashbrown::HashMap;

use commons::eyre::{eyre, Result};
use commons::grid::{Direction, Point};

pub const TITLE: &str = "Day 3: Crossed Wires";

pub fn run(raw: String) -> Result<()> {
    let crossed = parse(&raw);
    let closest = closest(&crossed[..]).ok_or_else(|| eyre!("Could not find closest !"))?;
    let (shortest, length) =
        shortest(&crossed[..]).ok_or_else(|| eyre!("Could not find shortest !"))?;

    let distance = closest.manhattan_distance();
    println!("Closest cross to origin : {closest} with distance {distance}");
    println!("Shortest cross to origin : {shortest} with length {length}");

    Ok(())
}

fn parse(s: &str) -> Vec<(Point, i64)> {
    fn parse_cable(line: &str) -> Option<HashMap<Point, i64>> {
        let mut current = Point::default();
        let mut from_origin = 0;
        let mut acc: HashMap<Point, i64> = HashMap::new();
        for movement in line.split(',') {
            let mut chars = movement.trim().chars();
            let direction = match chars.next() {
                Some('R') => Direction::East,
                Some('U') => Direction::North,
                Some('D') => Direction::South,
                Some('L') => Direction::West,
                _ => return None,
            };
            let end = chars.collect::<String>().parse::<i64>().ok()?;
            for _ in 0..end {
                current = current.moved(direction);
                from_origin += 1;
                acc.insert(current, from_origin);
            }
        }
        Some(acc)
    }

    // The values are : (number of overlapping cables, combined distance from origin)
    let cable_number = 2;
    let mut result: HashMap<Point, (i8, i64)> = HashMap::new();
    for raw_cable in s.lines() {
        if let Some(cable) = parse_cable(raw_cable) {
            for (point, from_origin) in cable {
                result
                    .entry(point)
                    .and_modify(|x| {
                        x.0 += 1;
                        x.1 += from_origin;
                    })
                    .or_insert((1, from_origin));
            }
        };
    }
    let crossed: Vec<(Point, i64)> = result
        .iter()
        .filter_map(|item| {
            let (point, (overlap, distance)) = item;
            if *overlap >= cable_number {
                Some((*point, *distance))
            } else {
                None
            }
        })
        .collect();

    crossed
}

/// Return the crossing point closest to the origin (according to manhattan distance)
fn closest(crossed: &[(Point, i64)]) -> Option<Point> {
    let mut min: Option<Point> = None;
    let mut distance = i64::MAX;
    for (point, _) in crossed.iter() {
        let current = point.manhattan_distance();
        if current < distance {
            min = Some(*point);
            distance = current;
        }
    }
    min
}

/// Return the crossing point with the shortest length (second member of the tuple)
fn shortest(crossed: &[(Point, i64)]) -> Option<(Point, i64)> {
    let mut min: Option<Point> = None;
    let mut distance = i64::MAX;
    for (point, length) in crossed.iter() {
        if *length < distance {
            min = Some(*point);
            distance = *length;
        }
    }
    min.map(move |x| (x, distance))
}

#[cfg(test)]
mod tests;
