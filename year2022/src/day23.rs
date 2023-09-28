use commons::err;
use commons::error::Result;
use commons::grid::{Direction, Point};

pub const TITLE: &str = "Day 23: Unstable Diffusion";

type Coordinate = i16;

pub fn run(raw: String) -> Result<()> {
    let mut data = parse(raw.into())?;
    let first = first_part(&mut data);
    println!("1. After 10 rounds there are {first} empty tiles in the bounding rectangle");
    let second = second_part(&mut data);
    println!("2. The first turn elves did not move is {second}");

    Ok(())
}

fn first_part(simulation: &mut Simulation) -> usize {
    for _ in 0..10 {
        let _ = simulation.next_turn();
    }

    simulation.grid.empty_tiles()
}

fn second_part(elves: &mut Simulation) -> usize {
    while elves.next_turn() {}
    elves.turns
}

struct Simulation {
    grid: Grid,
    swap: Grid,
    directions: [Direction; 4],
    turns: usize,
}

impl Simulation {
    fn new(grid: Grid) -> Self {
        let swap = Grid {
            min_y: grid.min_y,
            rows: vec![vec![]; grid.rows.len()],
        };
        Self {
            grid,
            swap,
            directions: [
                Direction::North,
                Direction::South,
                Direction::West,
                Direction::East,
            ],
            turns: 0,
        }
    }

    fn next_turn(&mut self) -> bool {
        let changed = self.grid.run_turn(self.directions, &mut self.swap);
        self.directions.rotate_left(1); // Move the first direction to the back of the list
        std::mem::swap(&mut self.grid, &mut self.swap);
        self.turns += 1;
        changed
    }
}

struct Grid {
    min_y: Coordinate,
    rows: Vec<Vec<Coordinate>>,
}

impl Grid {
    /// Clear all points from the grid, keeping the allocation intact
    fn clear(&mut self) {
        self.rows.iter_mut().for_each(Vec::clear);
    }

    /// Find the rectangle bounding the grid and the number of points in it
    fn empty_tiles(&self) -> usize {
        let (mut min_x, mut max_x) = (Coordinate::MAX, Coordinate::MIN);
        let mut count = 0;
        let height = self
            .rows
            .iter()
            .skip_while(|r| r.is_empty())
            .enumerate()
            .filter_map(|(y, row)| {
                (min_x, max_x) = match row.as_slice() {
                    [a, .., b] => (min_x.min(*a), max_x.max(*b)),
                    [a] => (min_x.min(*a), max_x.max(*a)),
                    [] => return None,
                };
                count += row.len();
                Some(y + 1)
            })
            .last();

        let width = max_x.abs_diff(min_x) as usize + 1;
        width * height.unwrap_or_default() - count
    }

    fn get_row_mut(&mut self, y: Coordinate) -> &mut Vec<Coordinate> {
        while y < self.min_y {
            self.rows.insert(0, vec![]);
            self.min_y -= 1;
        }

        let index = y.abs_diff(self.min_y) as usize;
        while index >= self.rows.len() {
            self.rows.push(vec![]);
        }
        &mut self.rows[index]
    }

    fn get_row(&self, y: Coordinate) -> &[Coordinate] {
        if y < self.min_y {
            &[]
        } else if let Some(row) = self.rows.get(y.abs_diff(self.min_y) as usize) {
            row.as_slice()
        } else {
            &[]
        }
    }

    /// Move all points from to their next position in `into`, returns whether there was a change
    fn run_turn(&self, directions: [Direction; 4], into: &mut Self) -> bool {
        into.clear();
        let mut changes = 0;
        for (row, y) in self.rows.iter().zip(self.min_y..) {
            let mut row = row.iter().copied().peekable();
            let mut above = self.get_row(y - 1);
            let mut below = self.get_row(y + 1);
            let mut previous = None;
            while let Some(x) = row.next() {
                match get_direction(
                    surroundings(&mut above, x),
                    previous.replace(x).map_or(true, |v| v != x - 1),
                    row.peek().map_or(true, |&v| v != x + 1),
                    surroundings(&mut below, x),
                    directions,
                ) {
                    Some(dir) => {
                        let Point { x, y } = Point::new(x, y).moved(dir);
                        let row = into.get_row_mut(y);
                        if insert_or_remove(row, x) {
                            // No collision, increase the number of moves by 1
                            changes += 1;
                        } else if matches!(dir, Direction::East | Direction::West) {
                            // Collision: push back the collided tile and ourselves back on X
                            insert_or_remove(row, x - 1);
                            insert_or_remove(row, x + 1);
                            changes -= 1;
                        } else {
                            // Collision: push back the collided tile and ourselves back on Y
                            insert_or_remove(into.get_row_mut(y - 1), x);
                            insert_or_remove(into.get_row_mut(y + 1), x);
                            changes -= 1;
                        }
                    }
                    None => {
                        insert_or_remove(into.get_row_mut(y), x);
                    }
                }
            }
        }
        changes != 0
    }
}

/// Insert the coordinate if it does not exist, otherwise remove it
fn insert_or_remove(row: &mut Vec<Coordinate>, x: Coordinate) -> bool {
    match row.binary_search(&x) {
        Ok(index) => {
            row.remove(index);
            false
        }
        Err(index) => {
            row.insert(index, x);
            true
        }
    }
}

/// Find which tiles around `x` in `row` are empty, advance `row` to the first position >= `x`
fn surroundings(row: &mut &[Coordinate], x: Coordinate) -> [bool; 3] {
    let (start, end) = row.split_at(row.partition_point(|&coord| coord < x));
    *row = end;
    let west = start.last().map_or(true, |c| *c != x - 1);
    match end.first() {
        Some(&a) if a == x => [west, false, end.get(1).map_or(true, |c| *c != x + 1)],
        Some(&a) if a == x + 1 => [west, true, false],
        _ => [west, true, true],
    }
}

/// Receives the emptiness status of the 8 surrounding tiles around a position
/// Computes which direction should be taken:
/// - if everything is empty, nothing
/// - otherwise the first direction in the given array for which all 3 related tiles are empty
fn get_direction(
    [nw, n, ne]: [bool; 3],
    w: bool,
    e: bool,
    [sw, s, se]: [bool; 3],
    directions: [Direction; 4],
) -> Option<Direction> {
    if nw && n && ne && w && e && sw && s && se {
        None
    } else {
        directions.into_iter().find(|dir| match *dir {
            Direction::North => nw && n && ne,
            Direction::West => nw && w && sw,
            Direction::East => ne && e && se,
            Direction::South => sw & s & se,
        })
    }
}

fn parse(s: std::borrow::Cow<'static, str>) -> Result<Simulation> {
    s.lines()
        .map(|line| {
            line.chars()
                .enumerate()
                .filter_map(move |(x, char)| match char {
                    '#' => Some(Ok(x as Coordinate)),
                    '.' => None,
                    _ => Some(Err(err!("unknown tile: {char}"))),
                })
                .collect::<Result<Vec<Coordinate>>>()
        })
        .collect::<Result<Vec<Vec<Coordinate>>>>()
        .map(|rows| Simulation::new(Grid { min_y: 0, rows }))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../examples/23.txt");
    const MAIN: &str = include_str!("../inputs/23.txt");

    #[test]
    fn first_part_small() {
        let mut data = parse(".....\n..##.\n..#..\n.....\n..##.\n.....".into()).unwrap();
        assert_eq!(first_part(&mut data), 25);
    }

    #[test]
    fn first_part_example() {
        let mut data = parse(EXAMPLE.into()).unwrap();
        assert_eq!(first_part(&mut data), 110);
    }

    #[test]
    fn first_part_main() {
        let mut data = parse(MAIN.into()).unwrap();
        assert_eq!(first_part(&mut data), 3766);
    }

    #[test]
    fn second_part_example() {
        let mut data = parse(EXAMPLE.into()).unwrap();
        assert_eq!(second_part(&mut data), 20);
    }

    #[test]
    fn second_part_main() {
        let mut data = parse(MAIN.into()).unwrap();
        assert_eq!(second_part(&mut data), 954);
    }
}
