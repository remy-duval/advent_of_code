use std::collections::VecDeque;

use itertools::Itertools;
use std::collections::HashSet;

use commons::{Result, WrapErr};

pub const TITLE: &str = "Day 12: Subterranean Sustainability";
const FIFTY_BILLION: usize = 50_000_000_000;

pub fn run(raw: String) -> Result<()> {
    let rules = parse(&raw)?;
    let first = first_part(&rules);
    println!("After 20 generations the sum is {first}");

    let second = second_part(&rules);
    println!("After fifty billion generations the sum is {second}");

    Ok(())
}

/// The rules of the plant generation
struct Rules {
    /// The initial generation, true means a plant is there
    initial_state: Vec<bool>,
    /// The rules for the next generation, compute the pattern of the state and index it
    rules: Vec<bool>,
}

fn parse(s: &str) -> Result<Rules> {
    fn state(state: &str) -> Vec<bool> {
        state.chars().map(|c| c == '#').collect()
    }

    let mut lines = s.lines();
    let initial_state = lines
        .next()
        .wrap_err_with(|| format!("Missing either the initial state or the rules {s}"))?
        .strip_prefix("initial state:")
        .wrap_err_with(|| format!("Bad format for the initial state {s}"))?
        .trim();

    let mut rules = vec![false; 32];
    lines.dropping(1).try_for_each(|line| -> Result<()> {
        let (pattern, after) = line
            .splitn(2, "=>")
            .map(str::trim)
            .collect_tuple::<(_, _)>()
            .wrap_err_with(|| format!("Bad format for a rule {line}"))?;

        let index = Rules::pattern(&state(pattern));
        let active = after.chars().next().map_or(false, |c| c == '#');
        rules[index] = active;
        Ok(())
    })?;

    Ok(Rules {
        initial_state: state(initial_state),
        rules,
    })
}

fn first_part(rules: &Rules) -> isize {
    let mut current = rules.initial_generation();
    let mut swap = HashSet::with_capacity(current.len());

    (0..20).for_each(|_| {
        next_generation(rules, &mut current, &mut swap);
        std::mem::swap(&mut current, &mut swap);
    });

    score(&current)
}

fn second_part(rules: &Rules) -> isize {
    let mut current = rules.initial_generation();
    let mut swap = HashSet::with_capacity(current.len());

    let mut generation: usize = 0;
    let mut current_score: isize = 0;

    // After a certain point the score becomes linear of the generation, to get it:
    // Compute generations while the score increase stabilizes for 10 consecutive generation
    let mut diffs: VecDeque<isize> = VecDeque::with_capacity(10);
    let increase = loop {
        next_generation(rules, &mut current, &mut swap);
        std::mem::swap(&mut current, &mut swap);

        let next_score = score(&current);
        diffs.push_back(next_score - current_score);
        current_score = next_score;
        generation += 1;

        if diffs.len() >= 10 {
            if let Some(diff) = diffs.pop_front() {
                if diffs.iter().all(|&x| x == diff) {
                    break diff;
                }
            }
        }
        if generation == FIFTY_BILLION {
            break 0;
        }
    };

    // Now that we know what the linear increase of the score is, we can compute the last generation
    (FIFTY_BILLION - generation) as isize * increase + current_score
}

/// The core of a generation
fn score(gen: &HashSet<isize>) -> isize {
    gen.iter().sum()
}

/// Compute the next generation of `from`, writing it in `into` (reuse the allocation)
fn next_generation(rules: &Rules, from: &mut HashSet<isize>, into: &mut HashSet<isize>) {
    into.clear();
    from.iter().for_each(|&idx| {
        let row = [
            from.contains(&(idx - 4)),
            from.contains(&(idx - 3)),
            from.contains(&(idx - 2)),
            from.contains(&(idx - 1)),
            true,
            from.contains(&(idx + 1)),
            from.contains(&(idx + 2)),
            from.contains(&(idx + 3)),
            from.contains(&(idx + 4)),
        ];

        (0..5).for_each(|i| {
            if rules.will_be_active(&row[i..(5 + i)]) {
                into.insert(idx + i as isize - 2);
            }
        });
    });
}

impl Rules {
    /// The initial generation per the rules
    fn initial_generation(&self) -> HashSet<isize> {
        let mut set = HashSet::with_capacity(self.initial_state.len());
        self.initial_state
            .iter()
            .enumerate()
            .for_each(|(i, active)| {
                if *active {
                    set.insert(i as isize);
                }
            });

        set
    }

    /// Check if a state will be active on next generation according to the rules
    fn will_be_active(&self, state: &[bool]) -> bool {
        self.rules[Self::pattern(state)]
    }

    /// Compute the pattern index of a state
    fn pattern(state: &[bool]) -> usize {
        state
            .iter()
            .fold(0, |acc, &next| (acc << 1) + usize::from(next))
    }
}

#[cfg(test)]
mod tests;
