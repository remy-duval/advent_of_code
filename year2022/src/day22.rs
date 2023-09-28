use itertools::Itertools;

use commons::error::Result;
use commons::grid::{Direction, Point};
use commons::parse::sep_by_empty_lines;
use commons::{err, WrapErr};

pub const TITLE: &str = "Day 22: Monkey Map";

type Coordinate = i32;

pub fn run(raw: String) -> Result<()> {
    let data = parse(raw.into())?;
    let first = first_part(&data);
    println!("1. The password is {first}");
    println!("2.TODO");

    Ok(())
}

fn first_part(map: &Map) -> u64 {
    let mut direction = Direction::East;
    let Some(mut position) = map.rows.first().and_then(|r| {
        let x = r.tiles.iter().position(|t| t.is_open())?;
        Some(Point::new(r.offset + x as Coordinate, 0))
    }) else {
        return 0;
    };

    map.path.iter().for_each(|token| match token {
        PathToken::MoveForward(n) => position = map.planar_move(position, direction, *n),
        PathToken::TurnLeft => direction = direction.left(),
        PathToken::TurnRight => direction = direction.right(),
    });

    let result = 1000 * (position.y as u64 + 1) + 4 * (position.x + 1) as u64;
    match direction {
        Direction::East => result,
        Direction::South => 1 + result,
        Direction::West => 2 + result,
        Direction::North => 3 + result,
    }
}

#[derive(Debug, Copy, Clone)]
enum Tile {
    Open,
    Wall,
}

impl Tile {
    fn to_char(self) -> char {
        match self {
            Tile::Open => '.',
            Tile::Wall => '#',
        }
    }

    fn is_open(self) -> bool {
        !matches!(self, Self::Wall)
    }
}

#[derive(Debug, Copy, Clone)]
enum PathToken {
    MoveForward(u8),
    TurnLeft,
    TurnRight,
}

#[derive(Debug)]
struct Row {
    offset: Coordinate,
    tiles: Vec<Tile>,
}

#[derive(Debug)]
struct Column {
    start: Coordinate,
    end: Coordinate,
}

#[derive(Debug)]
struct Map {
    rows: Vec<Row>,
    columns: Vec<Column>,
    path: Vec<PathToken>,
}

impl std::fmt::Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use std::fmt::Write;
        self.rows
            .iter()
            .flat_map(|line| {
                (0..line.offset)
                    .map(|_| ' ')
                    .chain(line.tiles.iter().map(|t| t.to_char()))
                    .chain(std::iter::once('\n'))
            })
            .chain(std::iter::once('\n'))
            .try_for_each(|char| f.write_char(char))
            .and_then(|_| {
                self.path.iter().try_for_each(|token| match token {
                    PathToken::MoveForward(v) => write!(f, "{v}"),
                    PathToken::TurnLeft => f.write_char('L'),
                    PathToken::TurnRight => f.write_char('R'),
                })
            })
    }
}

impl Map {
    fn planar_move(&self, from: Point<Coordinate>, dir: Direction, n: u8) -> Point<Coordinate> {
        match dir {
            Direction::North | Direction::South => {
                let x = from.x as usize;
                let Some(column) = self.columns.get(x) else {
                    return from;
                };
                let is_open = |y: Coordinate| {
                    usize::try_from(y)
                        .ok()
                        .and_then(|y| {
                            let row = self.rows.get(y)?;
                            row.tiles.get(x.checked_sub(row.offset as usize)?)
                        })
                        .is_some_and(|x| x.is_open())
                };

                let mut y = from.y;
                let (start, end) = (column.start, column.end);
                if dir.offset::<Coordinate>().y.is_positive() {
                    for _ in 0..n {
                        let next = if y >= end { start } else { y + 1 };
                        if is_open(next) {
                            y = next;
                        } else {
                            break;
                        }
                    }
                } else {
                    for _ in 0..n {
                        let next = if y <= start { end } else { y - 1 };
                        if is_open(next) {
                            y = next;
                        } else {
                            break;
                        }
                    }
                }

                Point::new(from.x, y)
            }
            Direction::East | Direction::West => {
                let Some(row) = self.rows.get(from.y as usize) else {
                    return from;
                };

                let (start, end) = (0, row.tiles.len() - 1);
                let is_open = |i: usize| row.tiles.get(i).is_some_and(|x| x.is_open());
                let Ok(mut i) = usize::try_from(from.x - row.offset) else {
                    return from;
                };
                if dir.offset::<Coordinate>().x.is_positive() {
                    for _ in 0..n {
                        let next = if i >= end { start } else { i + 1 };
                        if is_open(next) {
                            i = next;
                        } else {
                            break;
                        }
                    }
                } else {
                    for _ in 0..n {
                        let next = if i <= start { end } else { i - 1 };
                        if is_open(next) {
                            i = next;
                        } else {
                            break;
                        }
                    }
                }

                Point::new(i as Coordinate + row.offset, from.y)
            }
        }
    }
}

fn parse(s: std::borrow::Cow<'static, str>) -> Result<Map> {
    fn empty_column() -> Column {
        Column {
            start: Coordinate::MAX,
            end: Coordinate::MIN,
        }
    }

    let (grid, path) = sep_by_empty_lines(s.as_ref())
        .collect_tuple::<(_, _)>()
        .wrap_err("missing the two parts (grid and path)")?;

    let mut columns = vec![];
    let rows = grid
        .lines()
        .enumerate()
        .map(|(y, line)| {
            let y = y as Coordinate;
            line.find(['.', '#'])
                .map(|offset| {
                    let tiles: Vec<Tile> = line[offset..]
                        .bytes()
                        .filter_map(|b| match b {
                            b'.' => Some(Tile::Open),
                            b'#' => Some(Tile::Wall),
                            _ => None,
                        })
                        .collect();

                    // Update the minimum and maximum valid Y coordinate for each column
                    let width = tiles.len() + offset;
                    columns.resize_with(columns.len().max(width), empty_column);
                    columns[offset..width].iter_mut().for_each(|column| {
                        column.start = y.min(column.start);
                        column.end = y.max(column.end);
                    });

                    let offset = offset as Coordinate;
                    Row { offset, tiles }
                })
                .wrap_err_with(|| format!("Bad line #{y}:\n{line}"))
        })
        .collect::<Result<Vec<_>>>()?;

    let path = path
        .trim()
        .chars()
        .peekable()
        .batching(|chars| match chars.next()? {
            'L' => Some(Ok(PathToken::TurnLeft)),
            'R' => Some(Ok(PathToken::TurnRight)),
            num if num.is_ascii_digit() => {
                let mut forward = num.to_digit(10).unwrap_or_default() as u8;
                while let Some(num) = chars.next_if(char::is_ascii_digit) {
                    forward = forward * 10 + num.to_digit(10).unwrap_or_default() as u8;
                }
                Some(Ok(PathToken::MoveForward(forward)))
            }
            other => Some(Err(err!("Unknown path token: {other}"))),
        })
        .collect::<Result<Vec<_>>>()?;

    Ok(Map {
        rows,
        columns,
        path,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../examples/22.txt");
    const MAIN: &str = include_str!("../inputs/22.txt");

    #[test]
    fn first_part_example() {
        let data = parse(EXAMPLE.into()).unwrap();
        assert_eq!(first_part(&data), 6_032);
    }

    #[test]
    fn first_part_main() {
        let data = parse(MAIN.into()).unwrap();
        assert_eq!(first_part(&data), 190_066);
    }

    #[test]
    fn second_part_example() {
        let data = parse(EXAMPLE.into()).unwrap();
        println!("{data}");
    }

    #[test]
    fn second_part_main() {
        let data = parse(MAIN.into()).unwrap();
        println!("{data}");
    }
}
