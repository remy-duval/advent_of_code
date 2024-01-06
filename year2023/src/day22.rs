use std::ops::Range;

use itertools::Itertools;

use commons::error::Result;
use commons::{err, Report, WrapErr};

pub const TITLE: &str = "Day 22: Sand Slabs";

pub fn run(raw: String) -> Result<()> {
    let data = parse(raw.into())?;
    let first = first_part(&data);
    println!("1. The number of bricks that can be safely disintegrated is {first}");
    let second = second_part(&data);
    println!("2. The number of bricks that would fall would be {second}");

    Ok(())
}

fn first_part(relations: &[Relations]) -> usize {
    relations
        .iter()
        .filter(|r| {
            r.supports.iter().all(|&id| {
                relations
                    .get(id as usize)
                    .is_some_and(|supported| supported.supported_by > 1)
            })
        })
        .count()
}

fn second_part(relations: &[Relations]) -> usize {
    let mut total: usize = 0;
    let mut stack: Vec<u16> = vec![];
    let mut remains: Vec<usize> = vec![0; relations.len()];
    relations.iter().enumerate().for_each(|(i, r)| {
        remains.clear();
        remains.extend(relations[i..].iter().map(|r| r.supported_by));
        stack.extend_from_slice(&r.supports);
        while let Some(next) = stack.pop() {
            if let Some(supported_by) = remains.get_mut(next as usize - i) {
                *supported_by -= 1;
                if *supported_by == 0 {
                    total += 1;
                    if let Some(r) = relations.get(next as usize) {
                        stack.extend_from_slice(&r.supports);
                    }
                }
            }
        }
    });

    total
}

#[derive(Debug)]
struct Relations {
    supported_by: usize,
    supports: Vec<u16>,
}

#[derive(Debug)]
struct Brick {
    position: Position,
    extent: Extent,
}

#[derive(Debug, Copy, Clone)]
enum Extent {
    X(usize),
    Y(usize),
    Z(usize),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Position {
    x: usize,
    y: usize,
    z: usize,
}

impl Brick {
    fn new(mut position: Position, mut second: Position) -> Result<Self> {
        let extent = if position.x < second.x {
            Extent::X(std::mem::replace(&mut second.x, position.x))
        } else if position.x > second.x {
            Extent::X(std::mem::replace(&mut position.x, second.x))
        } else if position.y < second.y {
            Extent::Y(std::mem::replace(&mut second.y, position.y))
        } else if position.y > second.y {
            Extent::Y(std::mem::replace(&mut position.y, second.y))
        } else if position.z < second.z {
            Extent::Z(std::mem::replace(&mut second.z, position.z))
        } else {
            Extent::Z(std::mem::replace(&mut position.z, second.z))
        };
        if position == second {
            Ok(Self { position, extent })
        } else {
            Err(err!("multiple coordinates are different"))
        }
    }

    fn x(&self) -> Range<usize> {
        let start = self.position.x;
        match self.extent {
            Extent::X(x) => start..(x + 1),
            _ => start..(start + 1),
        }
    }

    fn y(&self) -> Range<usize> {
        let start = self.position.y;
        match self.extent {
            Extent::Y(y) => start..(y + 1),
            _ => start..(start + 1),
        }
    }

    fn z(&self) -> Range<usize> {
        let start = self.position.z;
        match self.extent {
            Extent::Z(z) => start..(z + 1),
            _ => start..(start + 1),
        }
    }
}

impl std::str::FromStr for Position {
    type Err = Report;
    fn from_str(s: &str) -> Result<Self> {
        let mut coordinates = s.trim().split(',');
        coordinates
            .next()
            .and_then(|x| {
                let y = coordinates.next()?;
                let z = coordinates.next()?;
                let position = match (x.parse(), y.parse(), z.parse()) {
                    (Ok(x), Ok(y), Ok(z)) => Ok(Position { x, y, z }),
                    (Err(e), _, _) | (_, Err(e), _) | (_, _, Err(e)) => Err(e),
                };
                Some(position)
            })
            .wrap_err("expected format: X,Y,Z")
            .and_then(|res| res.wrap_err("coordinate is not a number"))
            .wrap_err_with(|| format!("for {s:?}"))
    }
}

fn parse(s: std::borrow::Cow<'static, str>) -> Result<Vec<Relations>> {
    s.lines()
        .map(|line| -> Result<Brick> {
            match line.split_once('~') {
                Some((start, end)) => Brick::new(start.parse()?, end.parse()?),
                None => Err(err!("missing '~' separator")),
            }
        })
        .collect::<Result<Vec<Brick>>>()
        .map(create_graph)
}

fn create_graph(mut bricks: Vec<Brick>) -> Vec<Relations> {
    bricks.sort_unstable_by_key(|brick| brick.position.z);
    let (max_x, max_y, max_z) = bricks.iter().fold((1, 1, 1), |(x, y, z), b| {
        (x.max(b.x().end), y.max(b.y().end), z.max(b.z().end))
    });
    const AIR: u16 = u16::MAX;
    let mut grid = vec![AIR; max_z * max_y * max_x];
    let idx = |x: usize, y: usize, z: usize| x + (y + (z - 1) * max_y) * max_x;
    let mut relations: Vec<Relations> = Vec::with_capacity(bricks.len());
    for (brick, id) in bricks.into_iter().zip(0u16..) {
        let mut supported_by = 0;
        let mut z = brick.position.z;
        let mut below = brick.position.z.saturating_sub(1);
        loop {
            if below == 0 {
                supported_by += 1;
                break;
            }
            // Find which bricks are supporting this one currently
            brick
                .x()
                .flat_map(|x| brick.y().map(move |y| (x, y)))
                .filter_map(|(x, y)| grid.get(idx(x, y, below)))
                .filter(|&&support| support != AIR)
                .unique()
                .for_each(|&support| {
                    supported_by += 1;
                    if let Some(support) = relations.get_mut(support as usize) {
                        support.supports.push(id);
                    }
                });
            // If there is at least one, update it to keep track of that
            if supported_by != 0 {
                break;
            }
            // Otherwise drop by one and continue
            z -= 1;
            below -= 1;
        }
        relations.push(Relations {
            supported_by,
            supports: vec![],
        });
        // Update the grid for the next bricks
        brick
            .x()
            .flat_map(|x| brick.y().map(move |y| (x, y)))
            .flat_map(|a| {
                let max_z = match brick.extent {
                    Extent::Z(end_z) => z + end_z - brick.position.z,
                    _ => z,
                };
                (z..(max_z + 1)).map(move |z| (a, z))
            })
            .for_each(|((x, y), z)| {
                if let Some(pos) = grid.get_mut(idx(x, y, z)) {
                    *pos = id;
                }
            });
    }

    relations
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../examples/22.txt");
    const MAIN: &str = include_str!("../inputs/22.txt");

    #[test]
    fn first_part_example() {
        let data = parse(EXAMPLE.into()).unwrap();
        assert_eq!(first_part(&data), 5);
    }

    #[test]
    fn first_part_main() {
        let data = parse(MAIN.into()).unwrap();
        assert_eq!(first_part(&data), 411);
    }

    #[test]
    fn second_part_example() {
        let data = parse(EXAMPLE.into()).unwrap();
        assert_eq!(second_part(&data), 7);
    }

    #[test]
    fn second_part_main() {
        let data = parse(MAIN.into()).unwrap();
        assert_eq!(second_part(&data), 47_671);
    }
}
