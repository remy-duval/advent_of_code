use itertools::{process_results, Itertools};
use std::collections::HashSet;

use commons::eyre::{eyre, Result, WrapErr};

pub const TITLE: &str = "Day 19: Beacon Scanner";

pub fn run(raw: String) -> Result<()> {
    let (scanners, points) = scan(parse(&raw)?);
    println!("1. Number of beacons: {}", points.len());
    println!("2. Max distance is {}", max_distance(&scanners));

    Ok(())
}

type Point3d = [i16; 3];

/// A scanner that detects surrounding beacons
struct Scanner {
    /// The offset of this scanner to the first one
    offset: Point3d,
    /// All the beacons this scanners knows about
    beacons: Vec<Point3d>,
    /// The distance set of each beacon to all the other beacons
    /// Two scanners will overlap if at least 12 points' distance sets overlap
    distances: Vec<HashSet<u32>>,
}

/// Compute the max manhattan distance between all the points
fn max_distance(positions: &[Point3d]) -> i16 {
    let mut max = 0;
    for i in 0..positions.len() {
        for j in (0..positions.len()).filter(|&j| j != i) {
            let [x, y, z] = sub(positions[i], positions[j]);
            max = max.max(x.abs() + y.abs() + z.abs());
        }
    }

    max
}

/// Go through all the scanners, aligning them so they use the coordinate system of the first
///
/// ### Returns
/// `(scanners positions, beacons)`:
/// The positions of all the scanners and the set of all the beacons
fn scan(mut scanners: Vec<Scanner>) -> (Vec<Point3d>, HashSet<Point3d>) {
    let mut points = HashSet::new();

    // The first scanner is correctly aligned by definition
    if let Some(s) = scanners.first() {
        points.extend(s.beacons.as_slice());
    }

    // Go through correcting the scanners and moving the corrected ones at the head of the array
    let mut first_incorrect = 1;
    for correct in 0..scanners.len() {
        assert!(correct < first_incorrect, "Couldn't correct the scanners");
        let mut i = first_incorrect;
        while i < scanners.len() {
            let over = overlaps(&scanners[correct], &scanners[i]);
            if let Some((id, offset)) = find_transform(over) {
                points.reserve(scanners[i].beacons.len());
                scanners[i].offset = offset;
                scanners[i].beacons.iter_mut().for_each(|point| {
                    *point = add(offset, transform(*point, id));
                    points.insert(*point);
                });

                scanners.swap(first_incorrect, i);
                first_incorrect += 1;
            }

            i += 1;
        }
    }

    (scanners.into_iter().map(|s| s.offset).collect(), points)
}

/// Find the points that overlap between the two scanners
fn overlaps(first: &Scanner, second: &Scanner) -> Vec<(Point3d, Point3d)> {
    first
        .distances
        .iter()
        .enumerate()
        .filter_map(|(i, dist)| {
            // Assume two points are the same if they have the similar distances to other points
            // The intersection size should be at least 11 to form a 12-points match
            let j = second
                .distances
                .iter()
                .position(|d| dist.intersection(d).count() >= 11)?;
            Some((first.beacons[i], second.beacons[j]))
        })
        .collect()
}

/// Find the correct transform and offset to make the overlapping points have the same coordinates
fn find_transform(overlaps: Vec<(Point3d, Point3d)>) -> Option<(u8, Point3d)> {
    // If there are less than 12 overlapping points, we shouldn't try to match them
    if overlaps.len() < 12 {
        return None;
    }

    // Go through every available transform, returning the first that is coherent
    let mut id = 0;
    while id < 24 {
        // A transform is coherent if it makes every overlapping point differ by the same offset
        // We can then use that offset and transform to realign every point
        let offset = overlaps
            .iter()
            .map(|(a, b)| sub(*a, transform(*b, id)))
            .dedup()
            .exactly_one();

        match offset {
            Ok(offset) => return Some((id, offset)),
            Err(_) => id += 1,
        }
    }

    None
}

/// Add two points together
fn add([x1, y1, z1]: Point3d, [x2, y2, z2]: Point3d) -> Point3d {
    [x1 + x2, y1 + y2, z1 + z2]
}

/// Subtracts the second point from the first
fn sub([x1, y1, z1]: Point3d, [x2, y2, z2]: Point3d) -> Point3d {
    [x1 - x2, y1 - y2, z1 - z2]
}

/// Apply one of the rotation (0 to 23) to the point
fn transform([x, y, z]: Point3d, id: u8) -> Point3d {
    match id {
        0 => [x, y, z],
        1 => [-y, x, z],
        2 => [-x, -y, z],
        3 => [y, -x, z],
        4 => [-x, y, -z],
        5 => [y, x, -z],
        6 => [x, -y, -z],
        7 => [-y, -x, -z],
        8 => [-z, y, x],
        9 => [-z, x, -y],
        10 => [-z, -y, -x],
        11 => [-z, -x, y],
        12 => [z, y, -x],
        13 => [z, x, y],
        14 => [z, -y, x],
        15 => [z, -x, -y],
        16 => [x, -z, y],
        17 => [-y, -z, x],
        18 => [-x, -z, -y],
        19 => [y, -z, -x],
        20 => [x, z, -y],
        21 => [-y, z, -x],
        22 => [-x, z, y],
        _ => [y, z, x],
    }
}

/// Parses all the scanners
fn parse(s: &str) -> Result<Vec<Scanner>> {
    commons::parse::sep_by_empty_lines(s)
        .map(|s| -> Result<Scanner> {
            let beacons: Vec<Point3d> = s
                .lines()
                .dropping(1) // Skip the --- scanner N --- line
                .map(|p| {
                    process_results(p.splitn(3, ',').map(|n| n.parse::<i16>()), |mut r| {
                        Some([r.next()?, r.next()?, r.next()?])
                    })
                    .wrap_err_with(|| format!("Failed to parse coordinate in '{p}'"))
                    .and_then(|r| r.ok_or_else(|| eyre!("Not enough coordinates in '{p}'")))
                })
                .collect::<Result<_>>()
                .wrap_err_with(|| format!("For {s}"))?;

            let distances = (0..beacons.len())
                .map(|i| {
                    (0..beacons.len())
                        .filter(|&j| j != i)
                        .map(|j| {
                            let [x, y, z] = sub(beacons[j], beacons[i]);
                            let x = x.unsigned_abs() as u32;
                            let y = y.unsigned_abs() as u32;
                            let z = z.unsigned_abs() as u32;
                            x * x + y * y + z * z
                        })
                        .collect()
                })
                .collect();

            Ok(Scanner {
                offset: [0, 0, 0],
                beacons,
                distances,
            })
        })
        .collect()
}

#[cfg(test)]
mod tests;
