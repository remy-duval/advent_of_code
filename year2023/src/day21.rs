use std::collections::VecDeque;

use commons::error::Result;
use commons::{err, WrapErr};

pub const TITLE: &str = "Day 21: Step Counter";

const FIRST_STEPS: usize = 64;
const SECOND_STEPS: usize = 26_501_365;

pub fn run(raw: String) -> Result<()> {
    let data = parse(raw.into())?;
    let (first, second) = both_parts(&data, FIRST_STEPS, SECOND_STEPS);
    println!("1. After {FIRST_STEPS} steps, the elf can be at any of {first} tiles");
    println!("2. After {SECOND_STEPS}, the elf can be at any of {second} tiles");

    Ok(())
}

#[derive(Debug, Copy, Clone)]
enum Tile {
    Empty,
    Rock,
}

#[derive(Debug)]
struct Garden {
    grid: Vec<Tile>,
    width: i16,
    start: (i16, i16),
}

fn both_parts(garden: &Garden, first_steps: usize, second_steps: usize) -> (u64, u64) {
    fn rem_div(value: i16, max: i16) -> (i16, i16) {
        let (mut rem, mut quotient) = (value, 0);
        loop {
            if rem >= max {
                rem -= max;
                quotient += 1;
            } else if rem < 0 {
                rem += max;
                quotient -= 1;
            } else {
                break (rem, quotient);
            }
        }
    }
    let width = garden.width;
    let height = (garden.grid.len() / width as usize) as i16;
    let mut seen = vec![0; garden.grid.len()];
    let mut is_valid_next_point = |(x, y): (i16, i16)| {
        let (x, x_steps) = rem_div(x, width);
        let (y, y_steps) = rem_div(y, height);
        let index = (x + y * width) as usize;
        if !matches!(garden.grid.get((x + y * width) as usize), Some(Tile::Empty)) {
            return false;
        }
        // Use a bitset for each coordinate avoid using a HashSet of points (slow)
        // The furthest we go is 2.5width steps, so -2 <= screen <= 2
        // This means only 5 * 5 bits are needed, so it should fir in a u32
        let bit = 1u32 << ((x_steps + 2) + (y_steps + 2) * 4);
        if let Some(seen) = seen.get_mut(index).filter(|s| **s & bit == 0) {
            *seen |= bit;
            true
        } else {
            false
        }
    };

    let steps = [
        first_steps as i16,
        garden.start.0,
        garden.start.0 + width,
        garden.start.0 + 2 * width,
    ];

    let mut queue = VecDeque::from([(garden.start, 0)]);
    is_valid_next_point(garden.start); // Set the start point as seen
    let mut curve = [0, 0, 0, 0];
    while let Some(((x, y), step)) = queue.pop_front() {
        // If a point is reached at any step, it will continue to be for later step of same parity
        for (steps, reached) in steps.into_iter().zip(curve.iter_mut()) {
            if step <= steps && step % 2 == steps % 2 {
                *reached += 1;
            }
        }
        if step >= steps[steps.len() - 1] {
            continue;
        }
        queue.extend(
            [(-1, 0), (0, -1), (1, 0), (0, 1)]
                .into_iter()
                .map(|(d_x, d_y)| ((x + d_x, y + d_y), step + 1))
                .filter(|(point, _)| is_valid_next_point(*point)),
        );
    }

    let n = second_steps as u64 / width as u64;
    // For steps = Start + N * Width, reachable(N) seems to fit AN² + BN + C
    // Approximate A, B, C with three points on the curve: N = 0, 1, 2
    // curve[3] - 2curve[2] + curve[1] = 4A + 2B + C - 2(A + B + C) + C = 2A
    let a = (curve[3] - 2 * curve[2] + curve[1]) / 2;
    // 2curve[1] = 2C
    let c = curve[1];
    // 2curve[2] - 2A - 2C = 2(A + B + C) - 2A - 2C = 2B
    let b = curve[2] - a - c;
    (curve[0], n * n * a + n * b + c)
}

fn parse(s: std::borrow::Cow<'static, str>) -> Result<Garden> {
    let width = s.lines().next().wrap_err("empty input")?.len() as i16;
    let start = std::cell::Cell::new(None);
    let grid = s
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            let start = &start;
            line.trim().bytes().enumerate().map(move |(x, c)| match c {
                b'S' => {
                    start.set(Some((x as i16, y as i16)));
                    Ok(Tile::Empty)
                }
                b'.' => Ok(Tile::Empty),
                b'#' => Ok(Tile::Rock),
                bad => Err(err!("unknown tile at ({x},{y}) {}", bad as char)),
            })
        })
        .collect::<Result<Vec<Tile>>>()?;

    let start = start.get().wrap_err("missing 'S' tile")?;
    Ok(Garden { grid, width, start })
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../examples/21.txt");
    const MAIN: &str = include_str!("../inputs/21.txt");

    #[test]
    fn test_example() {
        let data = parse(EXAMPLE.into()).unwrap();
        assert_eq!(both_parts(&data, 6, 0).0, 16);
    }

    #[test]
    fn test_main() {
        let data = parse(MAIN.into()).unwrap();
        assert_eq!(
            both_parts(&data, FIRST_STEPS, SECOND_STEPS),
            (3_637, 601_113_643_448_699)
        );
    }
}
