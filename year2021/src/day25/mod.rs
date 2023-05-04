use itertools::Itertools;

use commons::{bail, Result};

pub const TITLE: &str = "Day 25: Sea Cucumber";

pub fn run(raw: String) -> Result<()> {
    let sea = parse(&raw)?;
    println!("1. Steps before stop: {}", move_until_deadlock(sea));

    Ok(())
}

type Tile = u8;
const EMPTY: Tile = 0;
const SOUTH: Tile = 1;
const EAST: Tile = 2;

struct Sea {
    width: usize,
    grid: Vec<Tile>,
}

fn parse(s: &str) -> Result<Sea> {
    let mut grid = Vec::new();
    let mut height = 0;
    for line in s.lines() {
        for c in line.chars() {
            let tile = match c {
                '.' => EMPTY,
                '>' => EAST,
                'v' => SOUTH,
                _ => bail!("Bad tile '{c}' in {line} (line {height})"),
            };
            grid.push(tile);
        }
        height += 1;
    }

    Ok(Sea {
        width: grid.len() / height,
        grid,
    })
}

fn move_until_deadlock(sea: Sea) -> usize {
    fn delay_first<T>(mut iterator: impl Iterator<Item = T>) -> impl Iterator<Item = T> {
        let first = iterator.next();
        iterator.chain(first)
    }

    let width = sea.width;
    let mut first = sea.grid;
    let mut swap = first.clone();
    let mut changed = 1;
    let mut steps = 0;
    while changed != 0 {
        changed = 0;
        steps += 1;

        // Move each EAST tiles right if empty before any move
        first
            .chunks(width)
            .zip(swap.chunks_mut(width))
            .for_each(|(from, to)| {
                from.iter()
                    .circular_tuple_windows::<(_, _, _)>()
                    .zip(delay_first(to.iter_mut()))
                    .for_each(|((before, current, after), result)| {
                        *result = match (*before, *current, *after) {
                            (EAST, EMPTY, _) => {
                                changed += 1;
                                EAST
                            }
                            (_, EAST, EMPTY) => EMPTY,
                            (_, other, _) => other,
                        }
                    });
            });

        std::mem::swap(&mut first, &mut swap);

        // Move each SOUTH tiles down if empty before any move
        first
            .chunks(width)
            .circular_tuple_windows::<(_, _, _)>()
            .zip(delay_first(swap.chunks_mut(width)))
            .for_each(|((a, b, c), d)| {
                for (((before, current), after), result) in a.iter().zip(b).zip(c).zip(d) {
                    *result = match (*before, *current, *after) {
                        (SOUTH, EMPTY, _) => {
                            changed += 1;
                            SOUTH
                        }
                        (_, SOUTH, EMPTY) => EMPTY,
                        (_, other, _) => other,
                    }
                }
            });

        std::mem::swap(&mut first, &mut swap);
    }

    steps
}

#[cfg(test)]
mod tests;
