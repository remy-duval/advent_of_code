use std::collections::hash_map::Entry;
use std::collections::HashMap;

use commons::error::Result;
use commons::grid::Grid;
use commons::{err, WrapErr};

pub const TITLE: &str = "Day 14: Parabolic Reflector Dish";

const CYCLES: usize = 1_000_000_000;

pub fn run(raw: String) -> Result<()> {
    let data = parse(raw.into())?;
    let first = first_part(data.clone());
    println!("1. The load after the north tilt is {first}");
    let second = second_part(data);
    println!("2. The load after {CYCLES} cycles is {second}");

    Ok(())
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Tile {
    Empty,
    MovingRock,
    FixedRock,
}

fn first_part(mut grid: Grid<Tile>) -> usize {
    tilt_vertical::<true>(&mut grid);
    compute_load(&grid)
}

fn second_part(mut grid: Grid<Tile>) -> usize {
    let mut seen: HashMap<Vec<Tile>, usize> = HashMap::with_capacity(1000);
    let mut current = 0;
    let last_position = loop {
        match seen.entry(grid.as_ref().to_vec()) {
            Entry::Occupied(cycle) => {
                let cycle_start = cycle.get();
                let cycle_length = current - cycle_start;
                break (CYCLES - cycle_start) % cycle_length + cycle_start;
            }
            Entry::Vacant(empty) => {
                empty.insert(current);
                tilt_cycle(&mut grid);
                current += 1;
            }
        }
    };

    let computed = seen
        .into_iter()
        .find(|(_, pos)| *pos == last_position)
        .map(|(tiles, _)| Grid::from_vec(grid.width(), tiles))
        .unwrap_or_else(|| grid);
    compute_load(&computed)
}

fn compute_load(grid: &Grid<Tile>) -> usize {
    (0..grid.height())
        .map(|y| {
            let rocks = grid
                .get_line(y as isize)
                .unwrap_or(&[])
                .iter()
                .filter(|t| matches!(t, Tile::MovingRock))
                .count();

            (grid.height() - y) * rocks
        })
        .sum()
}

fn tilt_cycle(grid: &mut Grid<Tile>) {
    tilt_vertical::<true>(grid);
    tilt_horizontal::<false>(grid);
    tilt_vertical::<false>(grid);
    tilt_horizontal::<true>(grid);
}

fn tilt_vertical<const NORTH: bool>(grid: &mut Grid<Tile>) {
    let offset = if NORTH { 1 } else { -1 };
    let height = grid.height() as isize;
    let modify_row = |y: isize| {
        for x in 0..grid.width() as isize {
            if matches!(grid.get((x, y)), Some(Tile::MovingRock)) {
                let mut dest = y;
                let mut next = y - offset;
                let range = 0..height;
                while range.contains(&next) && matches!(grid.get((x, next)), Some(Tile::Empty)) {
                    dest -= offset;
                    next -= offset;
                }
                if dest != y {
                    grid[(x, dest)] = Tile::MovingRock;
                    grid[(x, y)] = Tile::Empty;
                }
            }
        }
    };

    if NORTH {
        (1..height).for_each(modify_row);
    } else {
        (0..height - 1).rev().for_each(modify_row)
    }
}

fn tilt_horizontal<const EAST: bool>(grid: &mut Grid<Tile>) {
    let offset = if EAST { -1 } else { 1 };
    let width = grid.width() as isize;
    let modify_column = |x: isize| {
        for y in 0..grid.height() as isize {
            if matches!(grid.get((x, y)), Some(Tile::MovingRock)) {
                let mut dest = x;
                let mut next = x - offset;
                let range = 0..width;
                while range.contains(&next) && matches!(grid.get((next, y)), Some(Tile::Empty)) {
                    dest -= offset;
                    next -= offset;
                }
                if dest != x {
                    grid[(dest, y)] = Tile::MovingRock;
                    grid[(x, y)] = Tile::Empty;
                }
            }
        }
    };

    if EAST {
        (0..width - 1).rev().for_each(modify_column)
    } else {
        (1..width).for_each(modify_column);
    }
}

fn parse(s: std::borrow::Cow<'static, str>) -> Result<Grid<Tile>> {
    let width = s.lines().next().wrap_err("empty input")?.trim().len();
    let tiles = s
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.trim().bytes().enumerate().map(move |(x, c)| match c {
                b'O' => Ok(Tile::MovingRock),
                b'#' => Ok(Tile::FixedRock),
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

    const EXAMPLE: &str = include_str!("../examples/14.txt");
    const MAIN: &str = include_str!("../inputs/14.txt");

    #[test]
    fn first_part_example() {
        let data = parse(EXAMPLE.into()).unwrap();
        assert_eq!(first_part(data), 136);
    }

    #[test]
    fn first_part_main() {
        let data = parse(MAIN.into()).unwrap();
        assert_eq!(first_part(data), 109_833);
    }

    #[test]
    fn second_part_example() {
        let data = parse(EXAMPLE.into()).unwrap();
        assert_eq!(second_part(data), 64);
    }

    #[test]
    fn second_part_main() {
        let data = parse(MAIN.into()).unwrap();
        assert_eq!(second_part(data), 99_875);
    }
}
