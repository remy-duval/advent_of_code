use std::collections::BTreeMap;
use std::ops::Range;

use itertools::Itertools;

use commons::error::Result;
use commons::parse::sep_by_empty_lines;
use commons::{err, WrapErr};

pub const TITLE: &str = "Day 19: Aplenty";

pub fn run(raw: String) -> Result<()> {
    let data = parse(raw.into())?;
    let first = first_part(&data);
    println!("1. The sum of accepted part ratings is {first}");
    let second = second_part(&data);
    println!("2. The count of acceptable parts is {second}");

    Ok(())
}

#[derive(Debug)]
struct Input {
    start: u16,
    workflows: Vec<Workflow>,
    parts: Vec<[u16; 4]>,
}

#[repr(u8)]
#[derive(Debug, Copy, Clone)]
enum RatingCategory {
    X = 0,
    M = 1,
    A = 2,
    S = 3,
}

#[derive(Debug, Clone, Default)]
struct Workflow {
    checks: Vec<(RatingCategory, Operator, u16, Decision)>,
    fallback: Decision,
}

#[derive(Debug, Copy, Clone)]
enum Operator {
    Lower,
    Greater,
}

#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
enum Decision {
    Accepted,
    #[default]
    Rejected,
    Workflow(u16),
}

fn first_part(input: &Input) -> u64 {
    input
        .parts
        .iter()
        .filter(|part| {
            let mut current = input.start as usize;
            while let Some(workflow) = input.workflows.get(current) {
                let decision = workflow
                    .checks
                    .iter()
                    .find(|(cat, op, value, _)| match op {
                        Operator::Greater => part[*cat as usize] > *value,
                        Operator::Lower => part[*cat as usize] < *value,
                    })
                    .map_or(workflow.fallback, |c| c.3);

                current = match decision {
                    Decision::Accepted => return true,
                    Decision::Rejected => break,
                    Decision::Workflow(next) => next as usize,
                };
            }
            false
        })
        .flatten()
        .map(|rating| *rating as u64)
        .sum()
}

fn second_part(input: &Input) -> u64 {
    const START: u16 = 1;
    const END: u16 = 4001;

    // Modify the range by restricting by the second one. Returns whether it is non empty
    fn restrict_range(range: &mut Range<u16>, by: Range<u16>) -> bool {
        range.start = range.start.max(by.start);
        range.end = range.end.min(by.end);
        range.start < range.end
    }
    // Count how many possible parts are represented by these ratings ranges
    fn possible_parts(r: [Range<u16>; 4]) -> u64 {
        r.into_iter().map(|r| (r.end - r.start) as u64).product()
    }

    let mut rest = vec![(
        [START..END, START..END, START..END, START..END],
        input.start,
    )];
    let mut combinations = 0;
    while let Some((mut excluded, index)) = rest.pop() {
        let mut excluded_exists = true;
        let workflow = &input.workflows[index as usize];
        for &(cat, op, rhs, decision) in workflow.checks.iter() {
            let rating = cat as usize;
            let mut included = excluded.clone();
            // Split the ranges between the two branches based on whether they pass the check
            let included_exists = match op {
                Operator::Greater => {
                    excluded_exists = restrict_range(&mut excluded[rating], START..(rhs + 1));
                    restrict_range(&mut included[rating], (rhs + 1)..END)
                }
                Operator::Lower => {
                    excluded_exists = restrict_range(&mut excluded[rating], rhs..END);
                    restrict_range(&mut included[rating], START..rhs)
                }
            };

            // If the included branch is not empty, apply its decision
            if included_exists {
                match decision {
                    Decision::Accepted => combinations += possible_parts(included),
                    Decision::Rejected => (),
                    Decision::Workflow(next) => rest.push((included, next)),
                }
            };
            // Short-circuit if the excluded branch no longer represents any part
            if !excluded_exists {
                break;
            }
        }

        // If after all the workflow checks the excluded branch is non empty, apply the fallback
        if excluded_exists {
            match workflow.fallback {
                Decision::Accepted => combinations += possible_parts(excluded),
                Decision::Rejected => (),
                Decision::Workflow(next) => rest.push((excluded, next)),
            }
        }
    }

    combinations
}

fn parse(s: std::borrow::Cow<'static, str>) -> Result<Input> {
    let mut names: BTreeMap<&str, u16> = BTreeMap::new();
    let mut workflows: Vec<Workflow> = vec![];
    let (workflow_section, part_section) = sep_by_empty_lines(s.as_ref())
        .collect_tuple::<(_, _)>()
        .wrap_err("missing blank line between sections")?;

    for line in workflow_section.lines() {
        let (name, rules) = line
            .split_once('{')
            .and_then(|(n, r)| Some((n, r.strip_suffix('}')?)))
            .wrap_err_with(|| format!("missing {{}} in {line:?}"))?;
        let index = *names.entry(name).or_insert_with(|| {
            workflows.push(Workflow::default());
            workflows.len() as u16 - 1
        });

        rules
            .split(',')
            .try_for_each(|r| -> Result<()> {
                let (check, decision) = match r.split_once(':') {
                    Some((test, decision)) => {
                        let ((rating, value), operator) = {
                            test.split_once('<')
                                .map(|s| (s, Operator::Lower))
                                .or_else(|| test.split_once('>').map(|s| (s, Operator::Greater)))
                                .wrap_err("operator is not < or >")?
                        };
                        (Some((rating, operator, value)), decision)
                    }
                    None => (None, r),
                };

                let decision = match decision {
                    "A" => Decision::Accepted,
                    "R" => Decision::Rejected,
                    name => {
                        let index = *names.entry(name).or_insert_with(|| {
                            workflows.push(Workflow::default());
                            workflows.len() as u16 - 1
                        });
                        Decision::Workflow(index)
                    }
                };

                match check {
                    Some((rating, operator, value)) => {
                        workflows[index as usize].checks.push((
                            match rating {
                                "x" => RatingCategory::X,
                                "m" => RatingCategory::M,
                                "a" => RatingCategory::A,
                                "s" => RatingCategory::S,
                                _ => return Err(err!("unknown rating {rating:?}")),
                            },
                            operator,
                            value.parse().wrap_err("bad test operand")?,
                            decision,
                        ));
                    }
                    None => workflows[index as usize].fallback = decision,
                };
                Ok(())
            })
            .wrap_err_with(|| format!("in {line:?}"))?;

        // Optimization: remove the last conditions if they lead to the fallback decision anyway
        let workflow = &mut workflows[index as usize];
        while workflow
            .checks
            .last()
            .is_some_and(|(_, _, _, decision)| *decision == workflow.fallback)
        {
            workflow.checks.pop();
        }
    }

    let parts = part_section
        .lines()
        .map(|part| {
            part.strip_prefix('{')
                .and_then(|part| part.strip_suffix('}'))
                .wrap_err("missing surrounding '{}'")
                .and_then(|part| {
                    let mut ratings = [0; 4];
                    for rating in part.split(',') {
                        match rating.split_once('=') {
                            Some(("x", value)) => ratings[0] = value.parse()?,
                            Some(("m", value)) => ratings[1] = value.parse()?,
                            Some(("a", value)) => ratings[2] = value.parse()?,
                            Some(("s", value)) => ratings[3] = value.parse()?,
                            _ => return Err(err!("unknown rating {rating:?}")),
                        }
                    }
                    Ok(ratings)
                })
                .wrap_err_with(|| format!("in {part:?}"))
        })
        .collect::<Result<Vec<_>>>()?;

    let start = names.get("in").copied().wrap_err("missing 'in' workflow")?;
    Ok(Input {
        start,
        workflows,
        parts,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../examples/19.txt");
    const MAIN: &str = include_str!("../inputs/19.txt");

    #[test]
    fn first_part_example() {
        let data = parse(EXAMPLE.into()).unwrap();
        assert_eq!(first_part(&data), 19_114);
    }

    #[test]
    fn first_part_main() {
        let data = parse(MAIN.into()).unwrap();
        assert_eq!(first_part(&data), 495_298);
    }

    #[test]
    fn second_part_example() {
        let data = parse(EXAMPLE.into()).unwrap();
        assert_eq!(second_part(&data), 167_409_079_868_000);
    }

    #[test]
    fn second_part_main() {
        let data = parse(MAIN.into()).unwrap();
        assert_eq!(second_part(&data), 132_186_256_794_011);
    }
}
