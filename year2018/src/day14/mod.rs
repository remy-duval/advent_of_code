use std::str::FromStr;

use color_eyre::eyre::Result;
use itertools::Itertools;

use commons::Problem;

pub struct Day;

impl Problem for Day {
    type Input = Rules;
    const TITLE: &'static str = "Day 14: Chocolate Charts";

    fn solve(data: Self::Input) -> Result<()> {
        println!(
            "The last ten numbers are {}",
            first_part(data.full_number())
        );
        println!(
            "The given scores appear after {} recipes",
            second_part(data)
        );
        Ok(())
    }
}

fn first_part(after: usize) -> String {
    Recipes::new(after + 10).dropping(after).take(10).join("")
}

fn second_part(rules: Rules) -> usize {
    Recipes::new(1_000_000).find(&rules.0)
}

/// Iterator on the next elements of the recipe
#[derive(Debug, Clone)]
pub struct Recipes {
    first: usize,
    second: usize,
    inner: Vec<u8>,
    current: usize,
}

impl Recipes {
    /// Build a new iterator of recipes
    pub fn new(capacity: usize) -> Self {
        let mut inner = Vec::with_capacity(capacity);
        inner.push(3);
        inner.push(7);
        Self {
            first: 0,
            second: 1,
            inner,
            current: 0,
        }
    }

    /// Find the first index of a sequence of elements in the recipes
    pub fn find(&mut self, needle: &[u8]) -> usize {
        assert!(!needle.is_empty());
        let first = needle[0];
        let rest = &needle[1..];
        let mut cursor = 0;

        loop {
            self.nth(needle.len() * 1000).expect("Infinite iterator");
            let matched = self.inner[cursor..]
                .iter()
                .copied()
                .positions(|i| i == first)
                .find_map(|pos| {
                    let start = pos + cursor + 1;
                    if self.inner.get(start..(start + rest.len()))? == rest {
                        Some(start - 1)
                    } else {
                        None
                    }
                });

            if let Some(found) = matched {
                return found;
            } else {
                cursor = self.inner.len() - needle.len();
            }
        }
    }

    /// Generate the next recipes and put them back into the vector.
    /// Returns the number of generated elements
    fn generate(&mut self) -> Option<usize> {
        let a = *self.inner.get(self.first)?;
        let b = *self.inner.get(self.second)?;
        let next = a + b;

        let count = if next < 10 {
            self.inner.push(next);
            1
        } else {
            self.inner.push(next / 10);
            self.inner.push(next % 10);
            2
        };

        self.first = (self.first + 1 + a as usize) % self.inner.len();
        self.second = (self.second + 1 + b as usize) % self.inner.len();
        Some(count)
    }
}

impl Iterator for Recipes {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        // Add new items if there isn't enough yet
        if self.current >= self.inner.len() {
            self.generate()?;
        }
        let data = self.inner[self.current];

        self.current += 1;
        Some(data)
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        let mut rest = n.saturating_sub(self.inner.len().saturating_sub(self.current)) + 1;
        while rest > 0 {
            rest = rest.saturating_sub(self.generate()?);
        }
        self.current += n;
        self.next()
    }
}

/// The rules of the recipe search
#[derive(Debug, Clone)]
pub struct Rules(pub Vec<u8>);

impl Rules {
    /// The number of recipe for the first part
    pub fn full_number(&self) -> usize {
        self.0
            .iter()
            .fold(0usize, |acc, &next| acc * 10 + next as usize)
    }
}

impl FromStr for Rules {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.trim()
                .chars()
                .filter_map(|c| char::to_digit(c, 10))
                .map(|i| i as u8)
                .collect(),
        ))
    }
}

#[cfg(test)]
mod tests;
