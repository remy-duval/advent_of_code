use std::collections::BTreeMap;
use std::str::FromStr;

use itertools::Itertools;

use commons::parse::LineSep;
use commons::{err, Report, Result};

pub const TITLE: &str = "Day 7: The Sum of Its Parts";

pub fn run(raw: String) -> Result<()> {
    let data = parse(&raw)?;
    let requirements = build_requirements(&data.data);

    let steps = process_steps(requirements.clone());
    println!("The build steps are {steps}");

    let time = count_time(requirements, 5, 60);
    println!("The time to finish the sleigh is {time}s");

    Ok(())
}

fn parse(s: &str) -> Result<LineSep<Step>> {
    s.parse()
}

/// Build the requirements between steps that will be used everywhere else
fn build_requirements(steps: &[Step]) -> BTreeMap<char, Vec<char>> {
    let mut mappings = BTreeMap::new();
    steps.iter().for_each(|step| {
        mappings.entry(step.requires).or_insert_with(Vec::new);
        mappings
            .entry(step.step)
            .or_insert_with(Vec::new)
            .push(step.requires);
    });

    mappings
}

/// Find the order of steps (as a String)
fn process_steps(mut requirements: BTreeMap<char, Vec<char>>) -> String {
    let mut steps = String::with_capacity(26);
    while let Some((&next, _)) = requirements.iter().find(|(_, req)| req.is_empty()) {
        steps.push(next);
        requirements.remove(&next);
        requirements
            .values_mut()
            .for_each(|requires| requires.retain(|&req| req != next));
    }

    steps
}

/// Count the time the steps will take
///
/// ### Arguments
/// * `requirements` - The map of requirements for each step
/// * `workers` - The amount of available workers for the construction
/// * `time_offset` - The base time a step takes (offset + 1..26 depending of ASCII step value)
///
/// ### Returns
/// The instant at which the last step is done (from 0)
fn count_time(
    mut requirements: BTreeMap<char, Vec<char>>,
    workers: usize,
    time_offset: usize,
) -> usize {
    // Convert a char between A and Z to a number between 1 and 26
    fn time_for_step(c: char) -> usize {
        (c as usize).saturating_sub(64)
    }

    let mut in_progress: Vec<char> = Vec::with_capacity(26);
    let mut workers: Vec<(usize, Option<char>)> = vec![(0, None); workers];
    let mut now = 0;

    while !requirements.is_empty() {
        // Check if any worker finished its task
        workers
            .iter_mut()
            .filter_map(|(time, task)| if *time <= now { task.take() } else { None })
            .for_each(|done| {
                // Finished tasks are removed from the requirements pre-conditions
                requirements
                    .values_mut()
                    .for_each(|requires| requires.retain(|&req| req != done))
            });

        // Start all task that can be started (pair each possible task with a free worker)
        requirements
            .iter()
            .filter(|(_, req)| req.is_empty())
            .zip(workers.iter_mut().filter(|(time, _)| *time <= now))
            .for_each(|((&next, _), free)| {
                // Assign to the worker the offset + time for the task
                *free = (now + time_for_step(next) + time_offset, Some(next));
                in_progress.push(next);
            });

        // Remove started task from the requirements
        in_progress.drain(..).for_each(|started| {
            requirements.remove(&started);
        });

        now += 1;
    }

    workers
        .into_iter()
        .map(|(time, _)| time)
        .max()
        .unwrap_or_default()
}

/// A step in building the sleigh
struct Step {
    step: char,
    requires: char,
}

impl FromStr for Step {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (requires, step) = s
            .trim()
            .strip_prefix("Step")
            .and_then(|s| s.strip_suffix("can begin."))
            .and_then(|s| {
                s.splitn(2, "must be finished before step")
                    .filter_map(|s| s.trim().chars().next())
                    .collect_tuple::<(_, _)>()
            })
            .ok_or_else(|| err!("Bad format for a step, got {}", s))?;

        Ok(Self { step, requires })
    }
}

#[cfg(test)]
mod tests;
