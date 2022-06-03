use std::{
    collections::HashSet,
    fmt::{Display, Formatter},
};

use itertools::Itertools;

use commons::eyre::Result;

pub const TITLE: &str = "Day 24: Planet of Discord";

pub fn run(raw: String) -> Result<()> {
    let data = parse(&raw);
    let result = first_repeat(data);
    println!("{}", result);
    println!("Biodiversity rating is {}", result.biodiversity_rating());

    let result = recursive_expansion(data, 200);
    println!("The number of bugs after 200 minutes is {} (Yikes)", result);

    Ok(())
}

fn parse(s: &str) -> Bugs {
    let mut bugs = [[false; 5]; 5];
    for (y, line) in s.lines().take(5).enumerate() {
        for (x, c) in line.chars().take(5).enumerate() {
            bugs[y][x] = c == '#';
        }
    }

    Bugs { bugs }
}

/// Computes next states of the Bugs with no recursion until we get one we saw before.
fn first_repeat(start: Bugs) -> Bugs {
    let mut seen: HashSet<Bugs> = HashSet::new();
    let mut current = start;
    loop {
        // Insert returns false if value is already present
        if !seen.insert(current) {
            return current;
        } else {
            current = current.next_state();
        }
    }
}

/// Computes next states of the Bugs with recursion for n steps.
fn recursive_expansion(start: Bugs, n: usize) -> usize {
    let mut bugs: Vec<Bugs> = vec![start];

    for _ in 0..n {
        let mut first: Bugs = Bugs::default();
        let mut last: Bugs = Bugs::default();
        bugs = bugs
            .iter()
            .enumerate()
            .map(|(i, current)| {
                let before = if i == 0 {
                    first = bugs[i];
                    None
                } else {
                    bugs.get(i - 1)
                };

                let after = if i == bugs.len() - 1 {
                    last = bugs[i];
                    None
                } else {
                    bugs.get(i + 1)
                };

                current.recursive_next_state(before, after)
            })
            .collect();

        let before = Bugs::default().recursive_next_state(None, Some(&first));
        if before != Bugs::default() {
            bugs.insert(0, before);
        }
        let after = Bugs::default().recursive_next_state(Some(&last), None);
        if after != Bugs::default() {
            bugs.push(after);
        }
    }

    bugs.into_iter().map(|bugs| bugs.insects_number()).sum()
}

/// Represent a Bugs state for one minute
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Default)]
struct Bugs {
    bugs: [[bool; 5]; 5],
}

impl Display for Bugs {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let display = self
            .bugs
            .iter()
            .map(|line| {
                line.iter()
                    .map(|&has_bugs| if has_bugs { '#' } else { '.' })
                    .join("")
            })
            .join("\n");
        display.fmt(f)
    }
}

impl Bugs {
    /// Compute the biodiversity rating of this state
    fn biodiversity_rating(&self) -> u64 {
        let mut current_power = 1;
        let mut total = 0;
        for line in self.bugs.iter() {
            for &has_bugs in line.iter() {
                if has_bugs {
                    total += current_power;
                }
                current_power *= 2;
            }
        }

        total
    }

    /// The number of insects in this.
    fn insects_number(&self) -> usize {
        self.bugs
            .iter()
            .map(|line| line.iter().filter(|x| **x).count())
            .sum()
    }

    /// Computes the next state of this (after 1 minute), without recursion
    fn next_state(&self) -> Self {
        self.lifecycle(self.count_neighbors(), false)
    }

    /// Computes the next state using the recursive configuration of the second part
    fn recursive_next_state(&self, enclosing: Option<&Self>, middle: Option<&Self>) -> Self {
        let mut count = self.count_neighbors();

        // Add the count for the enclosing bugs
        if let Some(enc) = enclosing {
            for n in 0..5 {
                count[0][n] += if enc.bugs[1][2] { 1 } else { 0 };
                count[4][n] += if enc.bugs[3][2] { 1 } else { 0 };
                count[n][0] += if enc.bugs[2][1] { 1 } else { 0 };
                count[n][4] += if enc.bugs[2][3] { 1 } else { 0 };
            }
        }
        // Add the count for the bugs in the middle part.
        if let Some(mid) = middle {
            let up: u8 = mid.bugs[0].iter().map(|&l| if l { 1 } else { 0 }).sum();
            let right: u8 = mid.bugs.iter().map(|l| if l[4] { 1 } else { 0 }).sum();
            let down: u8 = mid.bugs[4].iter().map(|&l| if l { 1 } else { 0 }).sum();
            let left: u8 = mid.bugs.iter().map(|l| if l[0] { 1 } else { 0 }).sum();
            count[1][2] += up;
            count[2][3] += right;
            count[3][2] += down;
            count[2][1] += left;
        }

        self.lifecycle(count, true)
    }

    /// Goes through every tile in this Bugs, and counts the number of live bugs neighboring each.
    /// This does not take into account the recursive parts of the problem however.
    fn count_neighbors(&self) -> [[u8; 5]; 5] {
        let mut count: [[u8; 5]; 5] = [[0; 5]; 5];
        for (y, line) in self.bugs.iter().enumerate() {
            for (x, &has_bugs) in line.iter().enumerate() {
                if has_bugs {
                    if x < 4 {
                        count[y][x + 1] += 1;
                    }
                    if y < 4 {
                        count[y + 1][x] += 1;
                    }
                    if x > 0 {
                        count[y][x - 1] += 1;
                    }
                    if y > 0 {
                        count[y - 1][x] += 1;
                    }
                }
            }
        }

        count
    }

    /// Convert counts of adjacent bugs into a new Bugs state according to the rules.
    fn lifecycle(&self, count: [[u8; 5]; 5], recursive: bool) -> Self {
        let mut next = [[false; 5]; 5];
        for (y, line) in count.iter().enumerate() {
            for (x, &count) in line.iter().enumerate() {
                if recursive && (x == 2 && y == 2) {
                    next[y][x] = false
                } else if self.bugs[y][x] {
                    next[y][x] = count == 1;
                } else {
                    next[y][x] = count == 1 || count == 2;
                }
            }
        }

        Self { bugs: next }
    }
}

#[cfg(test)]
mod tests;
