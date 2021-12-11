use commons::eyre::{eyre, Report, Result};
use commons::Problem;

pub struct Day;

impl Problem for Day {
    type Input = Grid;
    const TITLE: &'static str = "Day 11: Dumbo Octopus";

    fn solve(mut data: Self::Input) -> Result<()> {
        println!("1. {} flashes after 100 steps", first_part(&data));
        println!("2. Synchronized after {} steps", second_part(&mut data));

        Ok(())
    }
}

/// Parser of the 10x10 puzzle grid
pub struct Grid([u8; 100]);

/// Count the number of flashes for the 100 first steps
fn first_part(grid: &Grid) -> usize {
    let mut flashed = [false; 100];
    let mut grid = grid.0;
    (0..100).fold(0, |total, _| total + next(&mut grid, &mut flashed))
}

/// Iterate until one step produces 100 flashes
fn second_part(grid: &mut Grid) -> usize {
    let mut flashed = [false; 100];
    let mut i = 1;
    while next(&mut grid.0, &mut flashed) != 100 {
        i += 1;
    }
    i
}

/// Compute the next state of the grid (uses the flashed array as buffer)
fn next(grid: &mut [u8; 100], flashed: &mut [bool; 100]) -> usize {
    // 1. All elements gain +1
    grid.iter_mut().for_each(|i| *i += 1);
    // 2. If an element becomes > 9, it flashes, and increases all adjacent elements by 1
    // 3. The same element can only flash once
    let mut changes = 1;
    while changes != 0 {
        changes = 0;
        (0..grid.len()).for_each(|index| {
            if !flashed[index] && grid[index] > 9 {
                flashed[index] = true;
                increase_adjacent(index, grid);
                changes += 1;
            }
        });
    }

    // 4. At the end of the step, elements that flashed reset to 0
    grid.iter_mut()
        .zip(flashed.iter_mut())
        .fold(0, |count, (i, flashed)| {
            if *flashed {
                *i = 0;
                *flashed = false;
                count + 1
            } else {
                count
            }
        })
}

/// Increase all elements in the array that are adjacent (including diagonals) to the center
fn increase_adjacent(i: usize, g: &mut [u8; 100]) {
    fn increase(i: usize, g: &mut [u8; 100]) {
        if let Some(x) = g.get_mut(i) {
            *x += 1;
        }
    }
    fn increase_opt(i: Option<usize>, g: &mut [u8; 100]) {
        if let Some(i) = i {
            increase(i, g);
        }
    }

    let x = i % 10;
    increase_opt(i.checked_sub(10), g);
    increase(i + 10, g);
    if x != 0 {
        increase_opt(i.checked_sub(11), g);
        increase_opt(i.checked_sub(1), g);
        increase(i + 9, g);
    }
    if x != 9 {
        increase_opt(i.checked_sub(9), g);
        increase(i + 1, g);
        increase(i + 11, g);
    }
}

impl std::str::FromStr for Grid {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self> {
        let mut grid = [0; 100];
        s.lines()
            .flat_map(|s| s.chars())
            .take(grid.len())
            .enumerate()
            .try_for_each(|(index, c)| -> Result<()> {
                let number = c.to_digit(10).ok_or_else(|| eyre!("Bad digit {}", c))?;
                grid[index] = number as u8;
                Ok(())
            })?;

        Ok(Self(grid))
    }
}

#[cfg(test)]
mod tests;
