use std::cmp::Ordering;
use std::collections::BinaryHeap;

use commons::error::Result;
use commons::grid::{Direction, Grid, Point};
use commons::{err, WrapErr};

pub const TITLE: &str = "Day 17: Clumsy Crucible";

pub fn run(raw: String) -> Result<()> {
    let data = parse(raw.into())?;
    let first = first_part(&data)?;
    println!("1. The minimum heat loss for the crucible is {first}");
    let second = second_part(&data)?;
    println!("1. The minimum heat loss for the ultra-crucible is {second}");

    Ok(())
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Path {
    cost: u32,
    point: Point<isize>,
    direction: Direction,
}

impl Ord for Path {
    fn cmp(&self, rhs: &Self) -> Ordering {
        // Reversed cost comparison to select the minimum (rust BinaryHeap is a Max heap)
        rhs.cost.cmp(&self.cost).then_with(|| {
            // Split ties on the point closest to the end in absolute distance
            let a = self.point.manhattan_distance();
            let b = rhs.point.manhattan_distance();
            a.cmp(&b).then_with(|| self.direction.cmp(&rhs.direction))
        })
    }
}

impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn first_part(grid: &Grid<u8>) -> Result<u32> {
    shortest_path(grid, 0, 3)
}

fn second_part(grid: &Grid<u8>) -> Result<u32> {
    shortest_path(grid, 4, 10)
}

fn shortest_path(grid: &Grid<u8>, min_line: u16, max_line: u16) -> Result<u32> {
    let mut seen_from = Grid::fill(grid.width(), grid.height(), 0u8);
    let mut queue = BinaryHeap::new();
    queue.push(Path {
        cost: 0,
        point: Point::new(0, 0),
        direction: Direction::East,
    });
    queue.push(Path {
        cost: 0,
        point: Point::new(0, 0),
        direction: Direction::South,
    });

    let dest = Point::new(grid.width() as isize - 1, grid.height() as isize - 1);
    while let Some(path) = queue.pop() {
        let dir_mask = match path.direction {
            Direction::North => 1,
            Direction::South => 2,
            Direction::East => 4,
            Direction::West => 8,
        };

        match seen_from.get_mut(path.point.tupled()) {
            Some(bits) if (*bits & dir_mask) == 0 => *bits |= dir_mask,
            _ => continue,
        };
        if path.point == dest {
            return Ok(path.cost);
        }

        let mut cost = path.cost;
        let mut point = path.point;
        for i in 1..(max_line + 1) {
            point = point.moved(path.direction);
            let p = point.tupled();
            if let Some(c) = grid.get(p) {
                cost += *c as u32;
                if i < min_line {
                    continue;
                }

                queue.push(Path {
                    cost,
                    point,
                    direction: path.direction.right(),
                });
                queue.push(Path {
                    cost,
                    point,
                    direction: path.direction.left(),
                });
            } else {
                break;
            }
        }
    }

    Err(err!("path to exit was not found"))
}

fn parse(s: std::borrow::Cow<'static, str>) -> Result<Grid<u8>> {
    let width = s.lines().next().wrap_err("empty input")?.len();
    let tiles = s
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.trim()
                .chars()
                .enumerate()
                .map(move |(x, c)| match c.to_digit(10) {
                    Some(d) => Ok(d as u8),
                    None => Err(err!("bad tile at ({x},{y}) {c}")),
                })
        })
        .collect::<Result<Vec<u8>>>()?;

    Ok(Grid::from_vec(width, tiles))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../examples/17.txt");
    const EXAMPLE_2: &str = include_str!("../examples/17_2.txt");
    const MAIN: &str = include_str!("../inputs/17.txt");

    #[test]
    fn first_part_example() {
        let data = parse(EXAMPLE.into()).unwrap();
        assert_eq!(first_part(&data).unwrap(), 102);
    }

    #[test]
    fn first_part_main() {
        let data = parse(MAIN.into()).unwrap();
        assert_eq!(first_part(&data).unwrap(), 797);
    }

    #[test]
    fn second_part_example_1() {
        let data = parse(EXAMPLE.into()).unwrap();
        assert_eq!(second_part(&data).unwrap(), 94);
    }

    #[test]
    fn second_part_example_2() {
        let data = parse(EXAMPLE_2.into()).unwrap();
        assert_eq!(second_part(&data).unwrap(), 71);
    }

    #[test]
    fn second_part_main() {
        let data = parse(MAIN.into()).unwrap();
        assert_eq!(second_part(&data).unwrap(), 914);
    }
}
