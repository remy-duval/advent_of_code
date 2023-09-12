use std::collections::hash_map::Entry;
use std::collections::HashMap;

use commons::error::Result;
use commons::{err, WrapErr};

pub const TITLE: &str = "Day 17: Pyroclastic Flow";

const FIRST: usize = 2022;
const SECOND: usize = 1_000_000_000_000;

pub fn run(raw: String) -> Result<()> {
    let data = parse(raw.into())?;
    let first = first_part(&data);
    println!("1. The tower of shapes will reach {first} units after {FIRST} moves");
    let second = second_part(&data);
    println!("2. The tower of shapes will reach {second} units after {SECOND} moves");

    Ok(())
}

fn first_part(moves: &[Direction]) -> u64 {
    let mut well = Vec::with_capacity(4096);
    run_steps(&mut well, FIRST, moves, 0);
    well.len() as u64
}

fn second_part(moves: &[Direction]) -> u64 {
    let mut well = Vec::with_capacity(4096);
    let mut cycles: HashMap<(u64, usize), (u64, usize)> = HashMap::new();
    let mut shapes = 0;
    let mut next_move = 0;
    loop {
        // Try to find a cycle
        for shape in Shape::ALL {
            // Resize the well to fit 3 empty rows + the shape
            well.extend([0; 7]);
            // Make the shape fall down the well until it is at rest
            next_move = shape.fall_down(&mut well, moves, next_move);
            // Trim empty rows following the shape falling down
            if let Some(pos) = well.iter().rposition(|line| *line != 0) {
                well.truncate(pos + 1);
            }
        }

        shapes += Shape::ALL.len();
        // If the top of the well blocks any rock from going further, check if this forms a cycle
        let top = match *well {
            [.., a, b, c, d, e, f, g, h] if (a | b | c | d | e | f | g | h) == 0b1111111 => {
                u64::from_ne_bytes([a, b, c, d, e, f, g, h])
            }
            _ => continue,
        };
        match cycles.entry((top, next_move)) {
            Entry::Vacant(empty) => {
                empty.insert((well.len() as u64, shapes));
            }
            Entry::Occupied(full) => {
                // This is a cycle with the formula:
                // Height(N) = StartHeight + N * CycleHeight
                // Shapes(N) = StartShapes + N * CycleShapes
                let (start_height, start_count) = *full.get();
                let cycle_height = well.len() as u64 - start_height;
                let cycle_shapes = shapes - start_count;

                // Apply the highest N cycles possible
                let cycles = (SECOND - start_count) / cycle_shapes;
                let rest = SECOND - start_count - cycles * cycle_shapes;
                let height = start_height + cycles as u64 * cycle_height;

                // Then run the end of the simulation
                let start_height = well.len() as u64;
                run_steps(&mut well, rest, moves, next_move);
                let last_height = well.len() as u64 - start_height;
                break last_height + height;
            }
        }
    }
}

fn run_steps(well: &mut Vec<u8>, steps: usize, moves: &[Direction], mut next_move: usize) {
    for shape in Shape::ALL.into_iter().cycle().take(steps) {
        // Resize the well to fit 3 empty rows + the shape
        well.extend([0; 7]);
        // Make the shape fall down the well until it is at rest
        next_move = shape.fall_down(well, moves, next_move);
        // Trim empty rows following the shape falling down
        if let Some(pos) = well.iter().rposition(|line| *line != 0) {
            well.truncate(pos + 1);
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Shape(u32);

impl Shape {
    /// The left most tiles of the well as a shape
    const LEFT_MOST: Self = Self::new(0b1000000, 0b1000000, 0b1000000, 0b1000000);

    /// The right most tiles of the well as a shape
    const RIGHT_MOST: Self = Self::new(0b0000001, 0b0000001, 0b0000001, 0b0000001);

    /// The shapes of the 5 rocks that fall down the well
    const ALL: [Self; 5] = [
        Self::new(0b0000000, 0b0000000, 0b0000000, 0b0011110), // Horizontal line shape
        Self::new(0b0000000, 0b0001000, 0b0011100, 0b0001000), // Cross shape
        Self::new(0b0000000, 0b0000100, 0b0000100, 0b0011100), // Mirrored L shape
        Self::new(0b0010000, 0b0010000, 0b0010000, 0b0010000), // Vertical line shape
        Self::new(0b0000000, 0b0000000, 0b0011000, 0b0011000), // Square shape
    ];

    /// Fuse 4 rows of tiles into a shape
    const fn new(a: u8, b: u8, c: u8, d: u8) -> Self {
        Self(u32::from_ne_bytes([a, b, c, d]))
    }

    /// The shape formed by the last 4 rows at the top of the well if there are at least 4
    const fn top(well: &[u8]) -> Self {
        if let [.., d, c, b, a] = *well {
            Self::new(a, b, c, d)
        } else {
            Self::new(0b1111111, 0b1111111, 0b1111111, 0b1111111)
        }
    }

    /// Stop at the current top of the well, writing the shape to it
    fn stop(self, well: &mut [u8]) {
        if let [.., d, c, b, a] = well {
            let [first, second, third, fourth] = self.rows();
            *a |= first;
            *b |= second;
            *c |= third;
            *d |= fourth;
        }
    }

    /// Explode the shape to the 4 rows that build it
    const fn rows(self) -> [u8; 4] {
        self.0.to_ne_bytes()
    }

    /// Check if the two shape share any part
    const fn collides(self, other: Self) -> bool {
        (self.0 & other.0) != 0
    }

    /// Move the shape to the left or the right unless it is already touching the walls
    const fn moved(self, dir: Direction) -> Self {
        match dir {
            Direction::Left if !self.collides(Self::LEFT_MOST) => Self(self.0 << 1),
            Direction::Right if !self.collides(Self::RIGHT_MOST) => Self(self.0 >> 1),
            _ => self,
        }
    }

    /// Make the shape fall down from the top of the well until it collides with something
    /// Returns the number of right / left moves performed
    fn fall_down(
        mut self,
        mut well: &mut [u8],
        moves: &[Direction],
        mut next_move: usize,
    ) -> usize {
        while let Some(&dir) = moves.get(next_move) {
            // Move the shape in the next direction if it does not collide with a wall or the well
            let next = self.moved(dir);
            next_move += 1;
            if next_move >= moves.len() {
                next_move = 0;
            }

            if !next.collides(Self::top(well)) {
                self = next;
            }

            // Go down one stage if there is no collision, otherwise stop
            if well.len() > 4 && !self.collides(Self::top(&well[..(well.len() - 1)])) {
                let end = well.len() - 1;
                well = &mut well[..end];
            } else {
                break;
            }
        }

        // Stop right here, writing the current values
        self.stop(well);
        next_move
    }
}

#[derive(Copy, Clone)]
enum Direction {
    Left,
    Right,
}

fn parse(s: std::borrow::Cow<'static, str>) -> Result<Vec<Direction>> {
    s.lines()
        .next()
        .wrap_err("empty input")?
        .chars()
        .map(|c| match c {
            '<' => Ok(Direction::Left),
            '>' => Ok(Direction::Right),
            c => Err(err!("Unknown character: {c}")),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../examples/17.txt");
    const MAIN: &str = include_str!("../inputs/17.txt");

    #[test]
    fn first_part_example() {
        let data = parse(EXAMPLE.into()).unwrap();
        assert_eq!(first_part(&data), 3068);
    }

    #[test]
    fn first_part_main() {
        let data = parse(MAIN.into()).unwrap();
        assert_eq!(first_part(&data), 3191);
    }

    #[test]
    fn second_part_example() {
        let data = parse(EXAMPLE.into()).unwrap();
        assert_eq!(second_part(&data), 1_514_285_714_288);
    }

    #[test]
    fn second_part_main() {
        let data = parse(MAIN.into()).unwrap();
        assert_eq!(second_part(&data), 1_572_093_023_267);
    }
}
