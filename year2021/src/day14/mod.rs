use hashbrown::HashMap;
use itertools::Itertools;

use commons::eyre::{ensure, eyre, Report, Result};
use commons::parse::sep_by_empty_lines;
use commons::Problem;

pub struct Day;

impl Problem for Day {
    type Input = Polymer;
    const TITLE: &'static str = "Day 14: Extended Polymerization";

    fn solve(polymer: Self::Input) -> Result<()> {
        // The capacity of the cache is huge to avoid a ton of re-allocations
        let mut cache = HashMap::with_capacity(3400);

        let one = polymer.min_max_after(10, &mut cache);
        println!("1. After 10 steps: {}", one.1 - one.0);

        let two = polymer.min_max_after(40, &mut cache);
        println!("2. After 40 steps: {}", two.1 - two.0);

        Ok(())
    }
}

pub struct Polymer {
    initial: Vec<u8>,
    rules: HashMap<(u8, u8), u8>,
}

type Counts = [u64; 26];

impl Polymer {
    /// Find the min and maximum counts of polymers after the given number of steps
    /// Caches intermediates results in the given cache
    fn min_max_after(&self, n: u8, cache: &mut HashMap<(u8, u8, u8), Counts>) -> (u64, u64) {
        let mut rates = Counts::default();
        self.initial.iter().for_each(|&i| rates[i as usize] += 1);
        self.initial.windows(2).for_each(|w| {
            if let [from, to] = w {
                self.rates_between_after((*from, *to), n, cache)
                    .iter()
                    .zip(rates.iter_mut())
                    .for_each(|(a, dest)| {
                        *dest += *a;
                    });
            }
        });

        rates
            .into_iter()
            .filter(|&c| c > 0)
            .minmax()
            .into_option()
            .unwrap_or_default()
    }

    /// Find the counts of polymers generated between two polymers after the given number of steps
    /// Caches intermediates results in the given cache
    fn rates_between_after(
        &self,
        (from, to): (u8, u8),
        steps: u8,
        cache: &mut HashMap<(u8, u8, u8), Counts>,
    ) -> Counts {
        if steps == 0 {
            return Counts::default();
        }
        let key = (from, to, steps);
        if let Some(result) = cache.get(&key).copied() {
            result
        } else {
            let mut result = Counts::default();
            if let Some(&mid) = self.rules.get(&(from, to)) {
                result[mid as usize] += 1;
                self.rates_between_after((from, mid), steps - 1, cache)
                    .into_iter()
                    .zip(self.rates_between_after((mid, to), steps - 1, cache))
                    .zip(result.iter_mut())
                    .for_each(|((a, b), dest)| {
                        *dest += a + b;
                    });
            }

            cache.insert(key, result);
            result
        }
    }
}

impl std::str::FromStr for Polymer {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self> {
        fn alpha_index(b: u8) -> Result<u8> {
            b.to_ascii_uppercase()
                .checked_sub(b'A')
                .ok_or_else(|| eyre!("Bad character {}", b))
        }

        let (initial, rules) = sep_by_empty_lines(s)
            .collect_tuple::<(_, _)>()
            .ok_or_else(|| eyre!("Missing empty line between polymer and rules in {}", s))?;

        let initial = initial.bytes().map(alpha_index).collect::<Result<_>>()?;
        let rules = rules
            .lines()
            .map(|r| -> Result<_> {
                let (from, to) = r
                    .split_once("->")
                    .ok_or_else(|| eyre!("Missing '->' in {}", r))?;
                let from = from.trim().as_bytes();
                let to = to.trim().as_bytes();
                ensure!(from.len() == 2, "Not 2 elements left of {}", r);
                ensure!(to.len() == 1, "Not 1 element right of {}", r);
                Ok((
                    (alpha_index(from[0])?, alpha_index(from[1])?),
                    alpha_index(to[0])?,
                ))
            })
            .collect::<Result<HashMap<_, _>>>()?;

        Ok(Self { initial, rules })
    }
}

#[cfg(test)]
mod tests;
