use itertools::Itertools;

use commons::error::Result;
use commons::grid::{Direction, Point};
use commons::WrapErr;

pub const TITLE: &str = "Day 18: Lavaduct Lagoon";

pub fn run(raw: String) -> Result<()> {
    let data = parse(raw.into())?;
    let first = first_part(&data);
    println!("1. The area of the trench is {first}");
    let second = second_part(&data);
    println!("2. The area of the second trench is {second}");

    Ok(())
}

struct Dig {
    direction: Direction,
    count: u8,
    color: u32,
}

fn first_part(digs: &[Dig]) -> i64 {
    compute_lac_volume(digs, |dig| (dig.direction, dig.count as i64))
}

fn second_part(digs: &[Dig]) -> i64 {
    compute_lac_volume(digs, |dig| {
        let direction = match dig.color % 4 {
            0 => Direction::East,
            1 => Direction::South,
            2 => Direction::West,
            _ => Direction::North,
        };
        (direction, (dig.color / 16) as i64)
    })
}

fn compute_lac_volume(digs: &[Dig], to_step: impl Fn(&Dig) -> (Direction, i64)) -> i64 {
    let mut boundary = 1;
    let mut current = Point::new(0, 0);

    // Get the Area of the polygon using https://en.wikipedia.org/wiki/Shoelace_formula
    // 2 * Area = Sum (xi * yi+1) - (xi+1 - yi)
    let twice_area = digs.iter().fold(0, |acc, dig| {
        let prev = current;
        let (direction, steps) = to_step(dig);
        current += direction.offset().multiply(steps);
        boundary += steps;
        acc + prev.x * current.y - current.x * prev.y
    });

    // Complete the boundary of the polygon if it was not closed
    if current != Point::new(0, 0) {
        boundary += current.manhattan_distance();
    }

    // The volume is the number of points (cubic meters) in the polygon interior + boundary
    // Rearranging the formula from https://en.wikipedia.org/wiki/Pick%27s_theorem
    // Area = Interior + Boundary / 2 - 1
    // Interior = Area - Boundary / 2 + 1
    // Interior + Boundary = Area + Boundary / 2 + 1
    // Interior + Boundary = (2 * Area + Boundary) / 2 + 1
    (twice_area + boundary) / 2 + 1
}

fn parse(s: std::borrow::Cow<'static, str>) -> Result<Vec<Dig>> {
    s.lines()
        .map(|dig| {
            dig.split_whitespace()
                .collect_tuple::<(_, _, _)>()
                .and_then(|(dir, count, color)| {
                    let dir = match dir {
                        "L" => Direction::West,
                        "R" => Direction::East,
                        "U" => Direction::North,
                        "D" => Direction::South,
                        _ => return None,
                    };
                    let color = color.strip_prefix("(#")?.strip_suffix(')')?;
                    Some((dir, count, color))
                })
                .wrap_err("not the expected 'R|D|L|U <number> (#<hex>)' format")
                .and_then(|(direction, count, color)| {
                    let count = count.parse().wrap_err("count is not a valid u8")?;
                    let color = u32::from_str_radix(color, 16)
                        .wrap_err("color is not a valid hexadecimal")?;
                    Ok(Dig {
                        direction,
                        count,
                        color,
                    })
                })
                .wrap_err_with(|| format!("for line {dig:?}"))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../examples/18.txt");
    const MAIN: &str = include_str!("../inputs/18.txt");

    #[test]
    fn first_part_example() {
        let data = parse(EXAMPLE.into()).unwrap();
        assert_eq!(first_part(&data), 62);
    }

    #[test]
    fn first_part_main() {
        let data = parse(MAIN.into()).unwrap();
        assert_eq!(first_part(&data), 47_527);
    }

    #[test]
    fn second_part_example() {
        let data = parse(EXAMPLE.into()).unwrap();
        assert_eq!(second_part(&data), 952_408_144_115);
    }

    #[test]
    fn second_part_main() {
        let data = parse(MAIN.into()).unwrap();
        assert_eq!(second_part(&data), 52_240_187_443_190);
    }
}
