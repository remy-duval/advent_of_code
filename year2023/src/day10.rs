use commons::error::Result;
use commons::grid::{Direction, Grid, Point};
use commons::{err, WrapErr};

pub const TITLE: &str = "Day 10: Pipe Maze";

pub fn run(raw: String) -> Result<()> {
    let data = parse(raw.into())?;
    let first = first_part(&data);
    println!("1. The farthest point from the start of the loop is at {first}");
    let second = second_part(&data);
    println!("2. The area enclosed by the loop is {second}");

    Ok(())
}

#[derive(Debug)]
struct Map {
    tiles: Grid<Tile>,
    start: Point<isize>,
}

#[derive(Debug, Clone, Copy)]
enum Tile {
    Empty,
    Start,
    Pipe(Direction, Direction),
}

impl Map {
    fn get_next(&self, point: Point<isize>, dir: Direction) -> Option<(Direction, Point<isize>)> {
        let neighbour = point.moved(dir);
        match self.tiles.get(neighbour.tupled()) {
            Some(Tile::Pipe(a, b)) if b.back() == dir => Some((*a, neighbour)),
            Some(Tile::Pipe(a, b)) if a.back() == dir => Some((*b, neighbour)),
            _ => None,
        }
    }

    fn connections(&self, point: Point<isize>) -> Option<[Direction; 2]> {
        match self.tiles.get(point.tupled()) {
            Some(Tile::Pipe(a, b)) => Some([*a, *b]),
            Some(Tile::Start) => {
                let mut dirs = Direction::ALL
                    .into_iter()
                    .filter_map(|dir| self.get_next(self.start, dir).map(|_| dir));
                Some([dirs.next()?, dirs.next()?])
            }
            _ => None,
        }
    }

    fn loop_iter(&self) -> impl Iterator<Item = (Direction, Point<isize>)> + '_ {
        let from = self
            .connections(self.start)
            .map(|dirs| (dirs[0], self.start));
        std::iter::successors(from, |prev| self.get_next(prev.1, prev.0))
    }
}

fn first_part(map: &Map) -> usize {
    map.loop_iter().count() / 2
}

fn second_part(map: &Map) -> usize {
    let width = map.tiles.width();
    let height = map.tiles.height();
    let mut loop_part = vec![None; width * height];
    map.loop_iter().for_each(|(_, p)| {
        loop_part[p.x as usize + p.y as usize * width] = map.connections(p);
    });

    (0..height)
        .map(|y| {
            let mut line = (y * width)..((y + 1) * width);
            // Skip the parts of the lines that are not inside the perimeter of the loop
            // And points there are inside if there is an odd number of perpendicular pipes West
            line.find(|&i| loop_part[i].is_some())
                .and_then(|_| line.rfind(|&i| loop_part[i].is_some()))
                .map_or(0, |_| {
                    // Bent pipes are only partially perpendicular, so count each direction
                    let (mut north, mut south) = (0, 0);
                    line.filter(|&i| {
                        if let Some([a, b]) = loop_part[i - 1] {
                            if a == Direction::North || b == Direction::North {
                                north += 1;
                            }
                            if a == Direction::South || b == Direction::South {
                                south += 1;
                            }
                        }
                        loop_part[i].is_none() && north.max(south) % 2 == 1
                    })
                    .count()
                })
        })
        .sum()
}

fn parse(s: std::borrow::Cow<'static, str>) -> Result<Map> {
    let width = s.lines().next().wrap_err("empty input")?.trim().len();
    let tiles = s
        .lines()
        .flat_map(|line| {
            line.trim().bytes().map(|c| match c {
                b'|' => Ok(Tile::Pipe(Direction::North, Direction::South)),
                b'-' => Ok(Tile::Pipe(Direction::East, Direction::West)),
                b'L' => Ok(Tile::Pipe(Direction::North, Direction::East)),
                b'J' => Ok(Tile::Pipe(Direction::North, Direction::West)),
                b'7' => Ok(Tile::Pipe(Direction::South, Direction::West)),
                b'F' => Ok(Tile::Pipe(Direction::South, Direction::East)),
                b'.' => Ok(Tile::Empty),
                b'S' => Ok(Tile::Start),
                bad => Err(bad),
            })
        })
        .collect::<Result<Vec<Tile>, _>>()
        .map_err(|bad| err!("bad tile character {}", bad as char))?;

    let tiles = Grid::from_vec(width, tiles);
    let start = tiles
        .indexed_values()
        .find(|(_, tile)| matches!(tile, Tile::Start))
        .map(|((x, y), _)| Point::new(x, y))
        .wrap_err("missing S tile")?;
    Ok(Map { tiles, start })
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../examples/10.txt");
    const EXAMPLE_2: &str = include_str!("../examples/10_2.txt");
    const EXAMPLE_3: &str = include_str!("../examples/10_3.txt");
    const EXAMPLE_4: &str = include_str!("../examples/10_4.txt");
    const MAIN: &str = include_str!("../inputs/10.txt");

    #[test]
    fn first_part_example() {
        let data = parse(EXAMPLE.into()).unwrap();
        assert_eq!(first_part(&data), 8);
    }

    #[test]
    fn first_part_main() {
        let data = parse(MAIN.into()).unwrap();
        assert_eq!(first_part(&data), 6_927);
    }

    #[test]
    fn second_part_example_a() {
        let data = parse(EXAMPLE_2.into()).unwrap();
        assert_eq!(second_part(&data), 4);
    }

    #[test]
    fn second_part_example_b() {
        let data = parse(EXAMPLE_3.into()).unwrap();
        assert_eq!(second_part(&data), 8);
    }

    #[test]
    fn second_part_example_c() {
        let data = parse(EXAMPLE_4.into()).unwrap();
        assert_eq!(second_part(&data), 10);
    }

    #[test]
    fn second_part_main() {
        let data = parse(MAIN.into()).unwrap();
        assert_eq!(second_part(&data), 467);
    }
}
