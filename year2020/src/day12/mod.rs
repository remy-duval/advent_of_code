use std::str::FromStr;

use color_eyre::eyre::{ensure, eyre, Report, Result, WrapErr};

use commons::grid::{Direction, Point};
use commons::parse::LineSep;
use commons::Problem;

pub struct Day;

impl Problem for Day {
    type Input = LineSep<Instruction>;
    const TITLE: &'static str = "Day 12: Rain Risk";

    fn solve(data: Self::Input) -> Result<()> {
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

impl FromStr for Instruction {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self> {
        ensure!(
            s.is_char_boundary(1),
            "Not enough characters to parse an instruction"
        );
        let (instruction, argument) = s.split_at(1);
        let argument: i64 = argument
            .parse()
            .wrap_err("Could not parse the instruction argument")?;
        match instruction.chars().next() {
            None => Err(eyre!("Not enough characters to parse an instruction")),
            Some('N') => Ok(Instruction::Move(Direction::North, argument)),
            Some('S') => Ok(Instruction::Move(Direction::South, argument)),
            Some('E') => Ok(Instruction::Move(Direction::East, argument)),
            Some('W') => Ok(Instruction::Move(Direction::West, argument)),
            Some('F') => Ok(Instruction::Forward(argument)),
            Some('R') => {
                ensure!(
                    argument % 90 == 0,
                    "A rotation argument should be a multiple of 90 degrees, not {}",
                    argument
                );
                Ok(Instruction::RotateRight(argument / 90))
            }
            Some('L') => {
                ensure!(
                    argument % 90 == 0,
                    "A rotation argument should be a multiple of 90 degrees, not {}",
                    argument
                );
                Ok(Instruction::RotateLeft(argument / 90))
            }
            Some(other) => Err(eyre!("Instruction type is not known: {}", other)),
        }
    }
}

#[cfg(test)]
mod tests;
