use commons::error::Result;
use commons::grid::{Grid, Point};
use commons::{err, WrapErr};

pub const TITLE: &str = "Day 14: Regolith Reservoir";

pub fn run(raw: String) -> Result<()> {
    let mut data = parse(raw.into())?;
    let first = first_part(&mut data);
    println!("1. {first} units of sand were poured before the bottom was reached");
    let second = second_part(data);
    println!("2. {second} units of sand were poured before the source was blocked");

    Ok(())
}

fn first_part(cave: &mut Cave) -> usize {
    while let Outcome::AtRest = cave.pour_sand() {}
    cave.sand_units - 1 // Last unit is on the bottom and not at rest
}

fn second_part(mut cave: Cave) -> usize {
    while let Outcome::AtRest | Outcome::AtBottom = cave.pour_sand() {}
    cave.sand_units
}

#[derive(Clone)]
struct Cave {
    sand_source: Point<isize>,
    sand_units: usize,
    grid: Grid<Tile>,
}

enum Outcome {
    AtRest,
    AtBottom,
    BlockedSource,
}

#[derive(Copy, Clone)]
enum Tile {
    Empty,
    Wall,
    Sand,
    Source,
}

impl Tile {
    fn to_char(self) -> char {
        match self {
            Self::Empty => '.',
            Self::Wall => '#',
            Self::Sand => 'o',
            Self::Source => '+',
        }
    }
}

impl Cave {
    fn pour_sand(&mut self) -> Outcome {
        let mut x = self.sand_source.x;
        let mut y = self.sand_source.y;
        let outcome = loop {
            let below = self.grid.get((x, y + 1));
            if below.is_none() {
                break Outcome::AtBottom;
            } else if matches!(below, Some(Tile::Empty)) {
                y += 1
            } else if matches!(self.grid.get((x - 1, y + 1)), Some(Tile::Empty)) {
                y += 1;
                x -= 1;
            } else if matches!(self.grid.get((x + 1, y + 1)), Some(Tile::Empty)) {
                y += 1;
                x += 1;
            } else {
                break Outcome::AtRest;
            }
        };

        self.sand_units += 1;
        if let Some(tile @ Tile::Empty) = self.grid.get_mut((x, y)) {
            *tile = Tile::Sand;
            outcome
        } else {
            Outcome::BlockedSource
        }
    }
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use std::fmt::Write;
        f.write_char(self.to_char())
    }
}

impl std::fmt::Display for Cave {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.grid)
    }
}

fn parse(s: std::borrow::Cow<'static, str>) -> Result<Cave> {
    let mut points = vec![];
    for line in s.lines() {
        let edges = line.split(" -> ").map(|p| match p.split_once(',') {
            Some((x, y)) => x
                .parse()
                .and_then(|x| y.parse().map(|y| Point::<i16>::new(x, y)))
                .wrap_err_with(|| format!("number in {p:?}")),
            None => Err(err!("format of {p:?}")),
        });

        itertools::process_results(edges, |mut edges| {
            if let Some(mut from) = edges.next() {
                for to in edges {
                    let diff = to - from;
                    let step = diff.divide(commons::math::gcd(diff.x, diff.y));
                    while from != to {
                        points.push(from);
                        from += step;
                    }
                }
                points.push(from);
            }
        })
        .wrap_err_with(|| format!("for line {line:?}"))?
    }

    let mut sand_source: Point<i16> = Point::new(500, 0);
    let (min, span) = {
        let mut min_x = points.iter().map(|p| p.x).min().unwrap_or_default();
        let mut max_x = points.iter().map(|p| p.x).max().unwrap_or_default();
        let max_y = points.iter().map(|p| p.y).max().unwrap_or_default() + 2;
        min_x = min_x.min(sand_source.x - max_y);
        max_x = max_x.max(sand_source.x + max_y + 1);
        sand_source.x -= min_x;
        (Point::new(min_x, 0), Point::new(max_x - min_x, max_y))
    };

    let mut grid = Grid::fill(span.x as usize, span.y as usize, Tile::Empty);
    for point in points {
        grid[((point.x - min.x) as isize, (point.y - min.y) as isize)] = Tile::Wall;
    }

    let sand_source: Point<isize> = Point::new(sand_source.x as isize, sand_source.y as isize);
    grid[(sand_source.x, sand_source.y)] = Tile::Source;
    Ok(Cave {
        sand_source,
        sand_units: 0,
        grid,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../examples/14.txt");
    const MAIN: &str = include_str!("../inputs/14.txt");

    #[test]
    fn first_part_example() {
        let mut data = parse(EXAMPLE.into()).unwrap();
        assert_eq!(first_part(&mut data), 24);
    }

    #[test]
    fn first_part_main() {
        let mut data = parse(MAIN.into()).unwrap();
        assert_eq!(first_part(&mut data), 774);
    }

    #[test]
    fn second_part_example() {
        let data = parse(EXAMPLE.into()).unwrap();
        assert_eq!(second_part(data), 93);
    }

    #[test]
    fn second_part_main() {
        let data = parse(MAIN.into()).unwrap();
        assert_eq!(second_part(data), 22_499);
    }
}
