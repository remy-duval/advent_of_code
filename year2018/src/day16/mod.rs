use color_eyre::eyre::Result;
use hashbrown::{hash_map::Entry, HashMap, HashSet};

use commons::Problem;
use instructions::{Int, OpCode};
use parse::Sample;

pub use super::instructions;

mod parse;

pub struct Day;

impl Problem for Day {
    type Input = parse::Program;
    const TITLE: &'static str = "Day 16: Chronal Classification";

    fn solve(data: Self::Input) -> Result<()> {
        let (first, possible) = find_possible(&data.samples);
        println!("{} samples behave like 3+ op codes", first);
        println!(
            "The register 0 contains {} after executing the program",
            data.execute(&sieve(possible)).0[0]
        );

        Ok(())
    }
}

/// Find the possible corresponding OpCode for each instruction code
/// Also for first part, return the number of sample that behave like 3+ OpCodes
fn find_possible(samples: &[Sample]) -> (usize, HashMap<Int, HashSet<OpCode>>) {
    let mut more_than_three = 0;
    let mut codes = HashMap::with_capacity(OpCode::ALL.len());
    samples.iter().for_each(|sample| {
        let possible: HashSet<OpCode> = corresponding_codes(sample).collect();
        if possible.len() >= 3 {
            more_than_three += 1;
        }

        match codes.entry(sample.instruction.code) {
            Entry::Vacant(empty) => {
                empty.insert(possible);
            }
            Entry::Occupied(mut full) => {
                *full.get_mut() = full.get().intersection(&possible).copied().collect();
            }
        };
    });

    (more_than_three, codes)
}

/// An iterator over the OpCode that are possible for the given sample
fn corresponding_codes(sample: &Sample) -> impl Iterator<Item = OpCode> + '_ {
    OpCode::ALL.iter().copied().filter(move |code| {
        let mut registers = sample.before.clone();
        match sample.instruction.execute(&mut registers, *code) {
            Ok(()) => registers == sample.after,
            Err(_) => false,
        }
    })
}

/// Find a guaranteed mapping Int -> OpCode, fix it, then do it again until all are attributed
fn sieve(mut possible: HashMap<Int, HashSet<OpCode>>) -> HashMap<Int, OpCode> {
    let mut found = HashMap::with_capacity(OpCode::ALL.len());
    while let Some((int, code)) = possible.iter().find_map(|(code, s)| {
        if s.len() == 1 {
            s.iter().next().map(|c| (*code, *c))
        } else {
            None
        }
    }) {
        possible.retain(|_, codes| {
            codes.remove(&code);
            !codes.is_empty()
        });
        found.insert(int, code);
    }

    found
}

#[cfg(test)]
mod tests;
