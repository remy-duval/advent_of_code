use std::collections::BinaryHeap;

use commons::{err, Result};

pub const TITLE: &str = "Day 15: Chiton";

pub fn run(raw: String) -> Result<()> {
    let grid = parse(&raw)?;
    println!("1. Lowest cost: {}", first_part(&grid));
    println!("2. Lowest cost on 5x grid: {}", second_part(&grid));

    Ok(())
}

fn parse(s: &str) -> Result<CostSquare> {
    let mut lines = s.lines();
    if let Some(first) = lines.next() {
        let width = first.chars().count();
        let mut storage = Vec::with_capacity(width * width);
        first
            .chars()
            .chain(lines.flat_map(|l| l.chars()))
            .try_for_each(|c| match c.to_digit(10) {
                None => Err(err!("Bad digit {}", c)),
                Some(i) => {
                    storage.push(i as u8);
                    Ok(())
                }
            })?;

        Ok(CostSquare { width, storage })
    } else {
        Err(err!("Missing first line in {}", s))
    }
}

fn first_part(s: &CostSquare) -> u16 {
    s.lowest_cost().unwrap_or_default()
}

fn second_part(s: &CostSquare) -> u16 {
    let width = s.width * 5;
    let mut storage = Vec::with_capacity(width);
    (0..5).for_each(|y| {
        (0..s.width).for_each(|i| {
            if let Some(line) = s.storage.get((i * s.width)..((i + 1) * s.width)) {
                (0..5).for_each(|x| {
                    for &cost in line {
                        let mut updated = cost as usize + x + y;
                        while updated > 9 {
                            updated -= 9;
                        }
                        storage.push(updated as u8);
                    }
                });
            }
        });
    });

    first_part(&CostSquare { width, storage })
}

/// The grid for the puzzle
struct CostSquare {
    width: usize,
    storage: Vec<u8>,
}

impl CostSquare {
    /// Find the lowest cost path from the first index to the last
    /// Uses Djikstra algorithm (A* with manhattan distance doesn't seem to be faster in this case)
    fn lowest_cost(&self) -> Option<u16> {
        #[derive(Eq, PartialEq)]
        struct Path(usize, u16);

        impl PartialOrd<Self> for Path {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }

        impl Ord for Path {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                // Reversed, since BinaryHeap returns the highest element and we want the lowest cost
                other.1.cmp(&self.1)
            }
        }

        let mut queue = BinaryHeap::with_capacity(self.width);
        let mut costs = vec![u16::MAX; self.storage.len()];
        queue.push(Path(0, 0));

        while let Some(Path(from, cost)) = queue.pop() {
            if from == self.storage.len() - 1 {
                return Some(cost);
            }

            self.for_each_adjacent(from, |p, c| {
                let cost = cost + c as u16;
                if cost < costs[p] {
                    costs[p] = cost;
                    queue.push(Path(p, cost));
                }
            });
        }

        None
    }

    /// Apply a callback on all elements around an index
    #[inline(always)]
    fn for_each_adjacent<ForEach: FnMut(usize, u8)>(&self, index: usize, mut f: ForEach) {
        let x = index % self.width;
        let mut execute = move |opt: Option<usize>| {
            if let Some(i) = opt {
                if let Some(content) = self.storage.get(i) {
                    f(i, *content)
                }
            }
        };

        execute(index.checked_add(self.width));
        if x != self.width {
            execute(index.checked_add(1));
        }
        execute(index.checked_sub(self.width));
        if x != 0 {
            execute(index.checked_sub(1));
        }
    }
}

#[cfg(test)]
mod tests;
