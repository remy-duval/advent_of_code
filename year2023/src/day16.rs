use commons::error::Result;
use commons::grid::{Direction, Grid, Point};
use commons::{err, WrapErr};

pub const TITLE: &str = "Day 16: The Floor Will Be Lava";

pub fn run(raw: String) -> Result<()> {
    let data = parse(raw.into())?;
    let first = first_part(&data);
    println!("1. After the beam propagates, there are {first} energized tiles");
    let second = second_part(&data);
    println!("2. With the best beam starting point, there are {second} energized tiles");

    Ok(())
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Tile {
    Empty,
    SlashMirror,
    BackSlashMirror,
    VerticalSplitter,
    HorizontalSplitter,
}

impl Tile {
    fn outgoing(self, direction: Direction) -> (Direction, Option<Direction>) {
        match self {
            Tile::SlashMirror => match direction {
                Direction::North => (Direction::East, None),
                Direction::South => (Direction::West, None),
                Direction::East => (Direction::North, None),
                Direction::West => (Direction::South, None),
            },
            Tile::BackSlashMirror => match direction {
                Direction::North => (Direction::West, None),
                Direction::South => (Direction::East, None),
                Direction::East => (Direction::South, None),
                Direction::West => (Direction::North, None),
            },
            Tile::VerticalSplitter if horizontal(direction) => {
                (Direction::South, Some(Direction::North))
            }
            Tile::HorizontalSplitter if !horizontal(direction) => {
                (Direction::East, Some(Direction::West))
            }
            _ => (direction, None),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum State {
    Empty,
    HorizontalBeam,
    VerticalBeam,
    CrossedBeams,
}

impl State {
    fn with_beam(self, direction: Direction) -> Self {
        match self {
            Self::Empty if horizontal(direction) => Self::HorizontalBeam,
            Self::Empty => Self::VerticalBeam,
            Self::HorizontalBeam if horizontal(direction) => Self::HorizontalBeam,
            Self::VerticalBeam if !horizontal(direction) => Self::VerticalBeam,
            _ => Self::CrossedBeams,
        }
    }
}
fn horizontal(direction: Direction) -> bool {
    match direction {
        Direction::East | Direction::West => true,
        Direction::North | Direction::South => false,
    }
}

fn first_part(grid: &Grid<Tile>) -> usize {
    Energizer::new(grid).count_energized((Point::new(0, 0), Direction::East))
}

fn second_part(grid: &Grid<Tile>) -> usize {
    let mut energizer = Energizer::new(grid);
    let width = grid.width() as isize;
    let height = grid.height() as isize;
    (0..width)
        .map(|x| (Point::new(x, 0), Direction::South))
        .chain((0..width).map(|x| (Point::new(x, height - 1), Direction::North)))
        .chain((0..height).map(|y| (Point::new(0, y), Direction::East)))
        .chain((0..height).map(|y| (Point::new(width - 1, y), Direction::West)))
        .map(|start| energizer.count_energized(start))
        .max()
        .unwrap_or_default()
}

/// Cache the inner state allocations required for traversing the grid many times (for part 2)
struct Energizer<'a> {
    grid: &'a Grid<Tile>,
    state: Grid<State>,
    stack: Vec<(Point<isize>, Direction)>,
}

impl<'a> Energizer<'a> {
    fn new(grid: &'a Grid<Tile>) -> Self {
        let state = Grid::fill(grid.width(), grid.height(), State::Empty);
        Self {
            grid,
            state,
            stack: vec![],
        }
    }

    fn count_energized(&mut self, start: (Point<isize>, Direction)) -> usize {
        self.stack.push(start);
        self.state.iter_mut().for_each(|s| *s = State::Empty);
        while let Some((point, direction)) = self.stack.pop() {
            let p = point.tupled();
            if let Some((&tile, state)) = self.grid.get(p).zip(self.state.get_mut(p)) {
                let updated = state.with_beam(direction);
                if updated == *state && tile != Tile::SlashMirror && tile != Tile::BackSlashMirror {
                    continue;
                }

                *state = updated;
                let (direction, additional) = tile.outgoing(direction);
                self.stack.push((point.moved(direction), direction));
                if let Some(direction) = additional {
                    self.stack.push((point.moved(direction), direction));
                }
            }
        }

        self.state.iter_mut().fold(0, |acc, s| {
            let next = acc + usize::from(!matches!(s, State::Empty));
            *s = State::Empty;
            next
        })
    }
}

fn parse(s: std::borrow::Cow<'static, str>) -> Result<Grid<Tile>> {
    let width = s.lines().next().wrap_err("empty input")?.len();
    let tiles = s
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.trim().bytes().enumerate().map(move |(x, c)| match c {
                b'-' => Ok(Tile::HorizontalSplitter),
                b'|' => Ok(Tile::VerticalSplitter),
                b'/' => Ok(Tile::SlashMirror),
                b'\\' => Ok(Tile::BackSlashMirror),
                b'.' => Ok(Tile::Empty),
                bad => Err(err!("unknown tile at ({x},{y}) {}", bad as char)),
            })
        })
        .collect::<Result<Vec<Tile>>>()?;

    Ok(Grid::from_vec(width, tiles))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../examples/16.txt");
    const MAIN: &str = include_str!("../inputs/16.txt");

    #[test]
    fn first_part_example() {
        let data = parse(EXAMPLE.into()).unwrap();
        assert_eq!(first_part(&data), 46);
    }

    #[test]
    fn first_part_main() {
        let data = parse(MAIN.into()).unwrap();
        assert_eq!(first_part(&data), 8_116);
    }

    #[test]
    fn second_part_example() {
        let data = parse(EXAMPLE.into()).unwrap();
        assert_eq!(second_part(&data), 51);
    }

    #[test]
    fn second_part_main() {
        let data = parse(MAIN.into()).unwrap();
        assert_eq!(second_part(&data), 8_383);
    }
}
