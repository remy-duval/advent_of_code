use std::str::FromStr;

use crate::commons::grid::{Direction, Point};
use crate::parse::LineSep;
use crate::Problem;

pub struct Day;

impl Problem for Day {
    type Input = LineSep<Instruction>;
    type Err = std::convert::Infallible;
    const TITLE: &'static str = "Day 12: Rain Risk";

    fn solve(data: Self::Input) -> Result<(), Self::Err> {
        let first = first_part(&data.data);

        println!(
            "First instructions: arrived at {:?} (manhattan distance is {} units)",
            first,
            first.manhattan_distance()
        );

        let second = second_part(&data.data);

        println!(
            "Second instructions: arrived at {:?} (manhattan distance is {} units)",
            second,
            second.manhattan_distance()
        );

        Ok(())
    }
}

/// Move will move the ship directly
fn first_part(instructions: &[Instruction]) -> Point {
    move_ship(
        Ship::new(Direction::East.offset()),
        instructions,
        |ship, offset| ship.position = ship.position + offset,
    )
        .position
}

/// Move will move the waypoint
fn second_part(instructions: &[Instruction]) -> Point {
    move_ship(
        Ship::new(Point::new(10, -1)),
        instructions,
        |ship, offset| ship.waypoint = ship.waypoint + offset,
    )
        .position
}

/// Rotate a point by 90 degree right around the center
pub fn rotate_right(point: Point) -> Point {
    Point::new(-point.y, point.x)
}

/// Rotate a point by 90 degree left around the center
pub fn rotate_left(point: Point) -> Point {
    Point::new(point.y, -point.x)
}

/// Move the ship from its initial state to the end
///
/// ### Arguments
/// * `initial` - The ship initial state
/// * `instructions` - The instructions to follow
/// * `on_move` - The action to execute on a Move instruction with (mutable Ship, movement)
pub fn move_ship<F>(initial: Ship, instructions: &[Instruction], mut on_move: F) -> Ship
    where
        F: FnMut(&mut Ship, Point),
{
    let mut ship = initial;
    instructions.iter().for_each(|inst| match *inst {
        Instruction::Forward(n) => ship.position = ship.position + ship.waypoint.multiply(n),
        Instruction::Move(direction, n) => on_move(&mut ship, direction.offset().multiply(n)),
        Instruction::RotateLeft(n) => {
            ship.waypoint = (0..n).fold(ship.waypoint, |p, _| rotate_left(p));
        }
        Instruction::RotateRight(n) => {
            ship.waypoint = (0..n).fold(ship.waypoint, |p, _| rotate_right(p));
        }
    });

    ship
}

/// The current state of the ship
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Ship {
    /// The ship current position
    position: Point,
    /// The ship current waypoint, followed by the Forward instruction
    waypoint: Point,
}

impl Ship {
    /// The ship at its initial position with the given waypoint
    pub fn new(waypoint: Point) -> Self {
        Self {
            position: Point::default(),
            waypoint,
        }
    }
}

/// An instruction to follow when moving the ship in this problem
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Instruction {
    /// Depends on the part: either move the ship directly, or move its waypoint
    Move(Direction, i64),
    /// Move the ship directly by n * its waypoint
    Forward(i64),
    /// Rotate the waypoint n * 90 degree around the ship
    RotateLeft(i64),
    /// Rotate the waypoint n * -90 degree around the ship
    RotateRight(i64),
}

/// An error that can happen when parsing an instruction
#[derive(Debug, thiserror::Error)]
pub enum ParseInstructionError {
    #[error("Not enough characters to parse an instruction")]
    BadFormat,
    #[error("Instruction type is not known: {0}")]
    UnknownInstruction(char),
    #[error("Could not parse the instruction argument because {0}")]
    ParseDigitError(#[from] std::num::ParseIntError),
    #[error("A rotation argument should be a multiple of 90 degrees, not {0}")]
    NonWholeRotation(i64),
}

impl FromStr for Instruction {
    type Err = ParseInstructionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.is_char_boundary(1) {
            Err(ParseInstructionError::BadFormat)
        } else {
            let (instruction, argument) = s.split_at(1);
            let argument: i64 = argument.parse()?;
            match instruction.chars().next() {
                None => Err(ParseInstructionError::BadFormat),
                Some('N') => Ok(Instruction::Move(Direction::North, argument)),
                Some('S') => Ok(Instruction::Move(Direction::South, argument)),
                Some('E') => Ok(Instruction::Move(Direction::East, argument)),
                Some('W') => Ok(Instruction::Move(Direction::West, argument)),
                Some('F') => Ok(Instruction::Forward(argument)),
                Some('R') => {
                    if argument % 90 != 0 {
                        Err(ParseInstructionError::NonWholeRotation(argument))
                    } else {
                        Ok(Instruction::RotateRight(argument / 90))
                    }
                }
                Some('L') => {
                    if argument % 90 != 0 {
                        Err(ParseInstructionError::NonWholeRotation(argument))
                    } else {
                        Ok(Instruction::RotateLeft(argument / 90))
                    }
                }
                Some(other) => Err(ParseInstructionError::UnknownInstruction(other)),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const A: &str = include_str!("test_resources/12-A.txt");
    const B: &str = include_str!("test_resources/12-B.txt");

    #[test]
    fn rotations() {
        let mut point = Direction::North.offset();
        point = rotate_right(point);
        assert_eq!(point, Direction::East.offset());
        point = rotate_right(point);
        assert_eq!(point, Direction::South.offset());
        point = rotate_right(point);
        assert_eq!(point, Direction::West.offset());
        point = rotate_right(point);
        assert_eq!(point, Direction::North.offset());
        point = rotate_left(point);
        assert_eq!(point, Direction::West.offset());
        point = rotate_left(point);
        assert_eq!(point, Direction::South.offset());
        point = rotate_left(point);
        assert_eq!(point, Direction::East.offset());
        point = rotate_left(point);
        assert_eq!(point, Direction::North.offset());
    }

    #[test]
    fn first_part_test_a() {
        let instructions = Day::parse(A).unwrap().data;
        let result = first_part(&instructions);
        assert_eq!(Point { x: 17, y: 8 }, result);
        assert_eq!(25, result.manhattan_distance());
    }

    #[test]
    fn first_part_test_b() {
        let instructions = Day::parse(B).unwrap().data;
        let result = first_part(&instructions);
        assert_eq!(Point { x: -112, y: 470 }, result);
        assert_eq!(582, result.manhattan_distance());
    }

    #[test]
    fn second_part_test_a() {
        let instructions = Day::parse(A).unwrap().data;
        let result = second_part(&instructions);
        assert_eq!(Point { x: 214, y: 72 }, result);
        assert_eq!(286, result.manhattan_distance());
    }

    #[test]
    fn second_part_test_b() {
        let instructions = Day::parse(B).unwrap().data;
        let result = second_part(&instructions);
        assert_eq!(Point { x: 15039, y: 37030 }, result);
        assert_eq!(52069, result.manhattan_distance());
    }
}
