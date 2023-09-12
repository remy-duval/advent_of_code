use std::str::FromStr;

use itertools::Itertools;

use commons::error::Result;
use commons::parse::LineSep;
use commons::{Report, WrapErr};

pub const TITLE: &str = "Day 18: Boiling Boulders";

pub fn run(raw: String) -> Result<()> {
    let data = parse(raw.into())?;
    let first = first_part(&data);
    println!("1. The surface area is {first}");
    let second = second_part(&data);
    println!("2. The surface area excluding the interior is {second}");

    Ok(())
}

fn first_part(points: &[Point3D]) -> usize {
    let width = points
        .iter()
        .map(|p| p.x.max(p.y).max(p.z))
        .max()
        .map_or(1, |m| m as usize + 1);

    // Contains lines of length width each containing the points with 2 equal coordinates
    let mut projections = vec![false; 3 * width * width * width];
    for point in points {
        let x = point.x as usize;
        let y = point.y as usize;
        let z = point.z as usize;
        // Project to lines of equal Z + Y
        projections[(z * width + y) * width + x] = true;
        // Project to lines of equal Z + X
        projections[((width + z) * width + x) * width + y] = true;
        // Project to lines of equal X + Y
        projections[((2 * width + x) * width + y) * width + z] = true;
    }

    // Now go through every line of same two coordinates
    // On a single line, deduplicate consecutive points * 2 sides to obtain the count
    (0..(3 * width * width * width))
        .step_by(width)
        .filter_map(|start| projections.get(start..(start + width)))
        .map(|line| line.iter().dedup().filter(|x| **x).count() * 2)
        .sum()
}

fn second_part(points: &[Point3D]) -> usize {
    // Remove the internal surface by filling in the solid, then just naively run the first part
    first_part(&fill_interior(points))
}

/// Fill in internal points by building the grid and then propagating water around it
fn fill_interior(points: &[Point3D]) -> Vec<Point3D> {
    #[repr(u8)]
    #[derive(Copy, Clone)]
    enum Tile {
        Empty = 0,
        Point = 1,
        Water = 2,
    }

    let max = points
        .iter()
        .map(|p| p.x.max(p.y).max(p.z))
        .max()
        .map_or(2, |m| m + 2);

    // Build a grid containing the points (offset by 1 to allow water to spawn there)
    let width = max as usize + 1;
    let mut grid = vec![Tile::Empty; width * width * width];
    let idx = |x: u8, y: u8, z: u8| (z as usize * width + y as usize) * width + x as usize;
    for point in points {
        grid[idx(point.x + 1, point.y + 1, point.z + 1)] = Tile::Point;
    }

    // Insert water in the border of the grid, and let it flow to the nearest empty tiles
    let mut stack = vec![(0, 0, 0)];
    while let Some((x, y, z)) = stack.pop() {
        if let Some(t @ Tile::Empty) = grid.get_mut(idx(x, y, z)) {
            *t = Tile::Water;
            if x != 0 {
                stack.push((x - 1, y, z));
            }
            if x < max - 1 {
                stack.push((x + 1, y, z));
            }
            if y != 0 {
                stack.push((x, y - 1, z));
            }
            if y < max - 1 {
                stack.push((x, y + 1, z));
            }
            if z != 0 {
                stack.push((x, y, z - 1));
            }
            if y < max - 1 {
                stack.push((x, y, z + 1));
            }
        }
    }

    // After the water has stopped flowing, any tile that is not water is internal
    (0..max)
        .cartesian_product(0..max)
        .cartesian_product(0..max)
        .filter(|&((x, y), z)| matches!(grid[idx(x, y, z)], Tile::Point | Tile::Empty))
        .map(|((x, y), z)| Point3D {
            x: x - 1,
            y: y - 1,
            z: z - 1,
        })
        .collect()
}

#[derive(Debug)]
struct Point3D {
    x: u8,
    y: u8,
    z: u8,
}

impl FromStr for Point3D {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self> {
        let (x, y, z) = s
            .splitn(3, ',')
            .collect_tuple::<(_, _, _)>()
            .wrap_err("expected 3 coordinates")?;
        let x = x.trim().parse().wrap_err("bad x coordinate")?;
        let y = y.trim().parse().wrap_err("bad y coordinate")?;
        let z = z.trim().parse().wrap_err("bad z coordinate")?;
        Ok(Point3D { x, y, z })
    }
}

fn parse(s: std::borrow::Cow<'static, str>) -> Result<Vec<Point3D>> {
    let split: LineSep<Point3D> = s.parse()?;
    Ok(split.data)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../examples/18.txt");
    const MAIN: &str = include_str!("../inputs/18.txt");

    #[test]
    fn first_part_example() {
        let data = parse(EXAMPLE.into()).unwrap();
        assert_eq!(first_part(&data), 64);
    }

    #[test]
    fn first_part_main() {
        let data = parse(MAIN.into()).unwrap();
        assert_eq!(first_part(&data), 3466);
    }

    #[test]
    fn second_part_example() {
        let data = parse(EXAMPLE.into()).unwrap();
        assert_eq!(second_part(&data), 58);
    }

    #[test]
    fn second_part_main() {
        let data = parse(MAIN.into()).unwrap();
        assert_eq!(second_part(&data), 2012);
    }
}
