use std::collections::VecDeque;

use hashbrown::HashSet;
use itertools::Itertools;

use commons::eyre::{eyre, Result};
use commons::grid::Point;
use commons::parse::sep_by_empty_lines;

pub const TITLE: &str = "Day 13: Transparent Origami";

pub fn run(raw: String) -> Result<()> {
    let mut origami = parse(&raw)?;
    origami.fold_once();
    println!("1. Dots after first fold: {}", origami.count());
    origami.fold_all();
    println!("2. Final origami:\n{}", origami);
    Ok(())
}

fn parse(s: &str) -> Result<Origami> {
    let (dots, folds) = sep_by_empty_lines(s)
        .collect_tuple::<(_, _)>()
        .ok_or_else(|| eyre!("Missing empty line between dots and folds"))?;

    let dots = dots
        .lines()
        .map(|d| -> Result<Point<i16>> {
            let (x, y) = d
                .split_once(',')
                .ok_or_else(|| eyre!("Missing ',' for a point: {}", d))?;
            Ok(Point::new(x.parse()?, y.parse()?))
        })
        .collect::<Result<HashSet<_>>>()?;

    let folds = folds
        .lines()
        .map(|f| -> Result<Fold> {
            match f
                .strip_prefix("fold along")
                .and_then(|f| f.trim().split_once('='))
            {
                Some(("x", x)) => Ok(Fold::Left(x.parse()?)),
                Some(("y", y)) => Ok(Fold::Up(y.parse()?)),
                _ => Err(eyre!("Can't parse fold instruction: {}", f)),
            }
        })
        .collect::<Result<VecDeque<_>>>()?;

    let buffer = dots.clone();
    Ok(Origami {
        dots,
        folds,
        buffer,
    })
}

/// The origami to process in this puzzle
struct Origami {
    /// The dots of the origami
    dots: HashSet<Point<i16>>,
    /// The remaining folds to apply
    folds: VecDeque<Fold>,
    /// A buffer of the dots used when folding
    buffer: HashSet<Point<i16>>,
}

/// A fold instruction for the origami
enum Fold {
    Left(i16),
    Up(i16),
}

impl Origami {
    /// The number of visible dots in this origami
    fn count(&self) -> usize {
        self.dots.len()
    }

    /// Resolve all remaining folds of the origami
    fn fold_all(&mut self) {
        loop {
            if !self.fold_once() {
                break;
            }
        }
    }

    /// Resolve the next fold of the origami, return true if we can continue
    fn fold_once(&mut self) -> bool {
        if let Some(fold) = self.folds.pop_front() {
            self.buffer.clear();
            match fold {
                Fold::Left(x) => self.dots.iter().for_each(|p| {
                    if p.x > x {
                        self.buffer.insert(Point::new(2 * x - p.x, p.y));
                    } else {
                        self.buffer.insert(*p);
                    }
                }),
                Fold::Up(y) => self.dots.iter().for_each(|p| {
                    if p.y > y {
                        self.buffer.insert(Point::new(p.x, 2 * y - p.y));
                    } else {
                        self.buffer.insert(*p);
                    }
                }),
            };

            std::mem::swap(&mut self.dots, &mut self.buffer);
            true
        } else {
            false
        }
    }
}

impl std::fmt::Display for Origami {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use std::fmt::Write;
        let (min_x, max_x) = match self.dots.iter().map(|p| p.x).minmax().into_option() {
            Some(m) => m,
            None => {
                return f.write_str("NO DATA");
            }
        };
        let (min_y, max_y) = match self.dots.iter().map(|p| p.y).minmax().into_option() {
            Some(m) => m,
            None => {
                return f.write_str("NO DATA");
            }
        };

        (min_y..(max_y + 1)).try_for_each(|y| {
            (min_x..(max_x + 1)).try_for_each(|x| {
                f.write_char(if self.dots.contains(&Point::new(x, y)) {
                    '#'
                } else {
                    '.'
                })
            })?;
            if y != max_y {
                f.write_char('\n')
            } else {
                Ok(())
            }
        })
    }
}

#[cfg(test)]
mod tests;
