use std::collections::HashSet;
use std::str::FromStr;

use commons::error::{Result, WrapErr};
use commons::grid::Point;
use commons::parse::LineSep;
use commons::{err, Report};

pub const TITLE: &str = "Day 9: Rope Bridge";

pub fn run(raw: String) -> Result<()> {
    let data = parse(raw.into())?;
    let first = first_part(&data);
    println!("1. With 2 knots the tail has visited {first} positions");
    let second = second_part(&data);
    println!("2. With 10 knots the tail has visited {second} positions");

    Ok(())
}

fn first_part(moves: &[Move]) -> usize {
    simulate_rope::<2>(moves)
}

fn second_part(moves: &[Move]) -> usize {
    simulate_rope::<10>(moves)
}

fn simulate_rope<const KNOTS: usize>(moves: &[Move]) -> usize {
    assert!(KNOTS >= 2, "rope size must be at least 2");
    let mut visited: HashSet<Point<i16>> = HashSet::new();
    let mut rope: [Point<i16>; KNOTS] = [Point::new(0, 0); KNOTS];
    visited.insert(rope[KNOTS - 1]);
    for &m in moves {
        let (dir, amount) = match m {
            Move::Up(amount) => (Point::new(0, 1), amount),
            Move::Down(amount) => (Point::new(0, -1), amount),
            Move::Left(amount) => (Point::new(-1, 0), amount),
            Move::Right(amount) => (Point::new(1, 0), amount),
        };

        for _ in 0..amount {
            let mut knots = rope.iter_mut();
            let mut head = knots.next().expect("rope size is >= 2");
            let mut tail = knots.next().expect("rope size is >= 2");
            // Move the head by the wanted amount
            *head += dir;
            loop {
                let diff = head.subtract(tail);
                if diff.x.abs() > 1 || diff.y.abs() > 1 {
                    *tail += Point::new(diff.x.signum(), diff.y.signum());
                    if let Some(next_tail) = knots.next() {
                        // There is still some part of the rope for which to simulate a move
                        head = std::mem::replace(&mut tail, next_tail);
                        continue;
                    }

                    // The tail end of the rope was moved
                    visited.insert(*tail);
                }
                break;
            }
        }
    }

    visited.len()
}

#[derive(Debug, Copy, Clone)]
enum Move {
    Up(u8),
    Down(u8),
    Left(u8),
    Right(u8),
}

impl FromStr for Move {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self> {
        if let Some((direction, amount)) = s.trim().split_once(' ') {
            let amount: u8 = amount
                .parse()
                .wrap_err_with(|| format!("bad amount: {s}"))?;
            match direction {
                "U" => Ok(Move::Up(amount)),
                "D" => Ok(Move::Down(amount)),
                "L" => Ok(Move::Left(amount)),
                "R" => Ok(Move::Right(amount)),
                _ => Err(err!("bad direction: {s}")),
            }
        } else {
            Err(err!("bad format): {s}"))
        }
    }
}

fn parse(s: std::borrow::Cow<'static, str>) -> Result<Vec<Move>> {
    let split: LineSep<Move> = s.parse()?;
    Ok(split.data)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../examples/09.txt");
    const EXAMPLE_LARGER: &str = include_str!("../examples/09_larger.txt");
    const MAIN: &str = include_str!("../inputs/09.txt");

    #[test]
    fn first_part_example() {
        let data = parse(EXAMPLE.into()).unwrap();
        assert_eq!(first_part(&data), 13);
    }

    #[test]
    fn first_part_main() {
        let data = parse(MAIN.into()).unwrap();
        assert_eq!(first_part(&data), 5_902);
    }

    #[test]
    fn second_part_example() {
        let data = parse(EXAMPLE.into()).unwrap();
        assert_eq!(second_part(&data), 1);
    }

    #[test]
    fn second_part_larger_example() {
        let data = parse(EXAMPLE_LARGER.into()).unwrap();
        assert_eq!(second_part(&data), 36);
    }

    #[test]
    fn second_part_main() {
        let data = parse(MAIN.into()).unwrap();
        assert_eq!(second_part(&data), 2_445);
    }
}
