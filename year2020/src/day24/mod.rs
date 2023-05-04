use std::str::FromStr;

use std::collections::{HashMap, HashSet};

use commons::grid::Point;
use commons::parse::LineSep;
use commons::{Report, Result, WrapErr};

pub const TITLE: &str = "Day 24: Lobby Layout";

pub fn run(raw: String) -> Result<()> {
    let data = parse(&raw)?;
    let initial_state = initial_state(data.data);
    println!("Day   0: {:<4} tiles are up", initial_state.len());

    let final_state = compute_next_state(initial_state, 100);
    println!("Day 100: {:<4} tiles are up", final_state.len());

    Ok(())
}

fn parse(s: &str) -> Result<LineSep<Path>> {
    s.parse()
}

/// Compute the initial state of the tiles from the paths
fn initial_state(paths: Vec<Path>) -> HashSet<Point> {
    let mut black_tiles: HashSet<Point> = HashSet::with_capacity(paths.len());
    paths.iter().map(Path::offset).for_each(|tile| {
        // Insert returns false if the tile was already present, in which case we remove it
        if !black_tiles.insert(tile) {
            black_tiles.remove(&tile);
        }
    });

    black_tiles
}

/// Compute the nth next state of the points (game of life)
fn compute_next_state(mut current: HashSet<Point>, n: usize) -> HashSet<Point> {
    let mut destination: HashSet<Point> = HashSet::with_capacity(current.len());
    let mut down_tiles: HashMap<Point, u8> = HashMap::with_capacity(current.len());

    // This is almost the same as all the previous games of life this year
    (0..n).for_each(|_| {
        current.iter().for_each(|up| {
            let count = Direction::adjacent_tiles(*up).fold(0u8, |acc, adj| {
                if current.contains(&adj) {
                    acc + 1
                } else {
                    *down_tiles.entry(adj).or_default() += 1;
                    acc
                }
            });

            if count == 1 || count == 2 {
                destination.insert(*up);
            }
        });

        down_tiles.drain().for_each(|(up, count)| {
            if count == 2 {
                destination.insert(up);
            };
        });

        std::mem::swap(&mut current, &mut destination);
        destination.clear();
    });

    current
}

/// The directions used in the problem
///
/// east, southeast, southwest, west, northwest, and northeast
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
enum Direction {
    East,
    West,
    SouthEast,
    SouthWest,
    NorthEast,
    NorthWest,
}

impl Direction {
    /// All the directions of a point
    const ALL: [Direction; 6] = [
        Self::East,
        Self::West,
        Self::SouthEast,
        Self::SouthWest,
        Self::NorthEast,
        Self::NorthWest,
    ];

    /// An iterator over the adjacent tiles of a point
    fn adjacent_tiles(point: Point) -> impl Iterator<Item = Point> {
        Self::ALL
            .iter()
            .map(move |direction| direction.offset() + point)
    }

    /// The offset to move in on the grid for a direction
    fn offset(self) -> Point {
        match self {
            Direction::East => Point::new(-2, 0),
            Direction::West => Point::new(2, 0),
            Direction::SouthEast => Point::new(-1, -1),
            Direction::SouthWest => Point::new(1, -1),
            Direction::NorthEast => Point::new(-1, 1),
            Direction::NorthWest => Point::new(1, 1),
        }
    }
}

/// The path to follow to get to a specific tile
pub struct Path(Vec<Direction>);

impl Path {
    /// The offset to move in on the grid for a specific path
    fn offset(&self) -> Point {
        self.0
            .iter()
            .fold(Point::new(0, 0), |acc, next| acc + next.offset())
    }
}

impl FromStr for Path {
    type Err = Report;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let mut chars = line.chars();
        let mut path = Vec::with_capacity(10);
        while let Some(char) = chars.next() {
            let direction = match char {
                'e' => Some(Direction::East),
                'w' => Some(Direction::West),
                'n' => chars.next().and_then(|char| match char {
                    'e' => Some(Direction::NorthEast),
                    'w' => Some(Direction::NorthWest),
                    _ => None,
                }),
                's' => chars.next().and_then(|char| match char {
                    'e' => Some(Direction::SouthEast),
                    'w' => Some(Direction::SouthWest),
                    _ => None,
                }),
                _ => None,
            }
            .wrap_err_with(|| format!("Unknown direction in line '{line}'"))?;

            path.push(direction);
        }

        Ok(Self(path))
    }
}

#[cfg(test)]
mod tests;
