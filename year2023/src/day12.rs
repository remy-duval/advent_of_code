use std::collections::HashMap;
use std::num::NonZeroU8;

use commons::error::Result;
use commons::{err, WrapErr};

pub const TITLE: &str = "Day 12: Hot Springs";

pub fn run(raw: String) -> Result<()> {
    let data = parse(raw.into())?;
    let first = first_part(&data);
    println!("1. There are {first} combinations that satisfy the group constraints");
    let second = second_part(data);
    println!("2. There are {second} combinations after unfold");

    Ok(())
}

#[derive(Debug, Copy, Clone)]
enum Gear {
    Good,
    Broken,
    Unknown,
}

#[derive(Debug)]
struct Row {
    row: Vec<Gear>,
    groups: Vec<NonZeroU8>,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Default)]
struct Prefix {
    next: u8,
    next_group: u8,
}

fn first_part(rows: &[Row]) -> u64 {
    let mut cache = HashMap::new();
    rows.iter()
        .map(|row| {
            cache.clear();
            count_combinations(row, Prefix::default(), &mut cache)
        })
        .sum()
}

fn second_part(rows: Vec<Row>) -> u64 {
    let mut cache = HashMap::new();
    rows.into_iter()
        .map(|mut row| {
            let row_len = row.row.len();
            let group_len = row.groups.len();
            for _ in 0..4 {
                row.row.push(Gear::Unknown);
                row.row.extend_from_within(0..row_len);
                row.groups.extend_from_within(0..group_len);
            }
            cache.clear();
            count_combinations(&row, Prefix::default(), &mut cache)
        })
        .sum()
}

fn count_combinations(row: &Row, prefix: Prefix, cache: &mut HashMap<Prefix, u64>) -> u64 {
    if let Some(cached) = cache.get(&prefix) {
        return *cached;
    }

    let suffix = row.row.get((prefix.next as usize)..).unwrap_or(&[]);
    let mut next_group = prefix.next_group;
    let mut group = None;
    let mut valid = 0;
    let done = suffix.iter().zip(prefix.next..).all(|(gear, i)| {
        match (gear, group.as_mut()) {
            (Gear::Good, None) | (Gear::Good | Gear::Unknown, Some(0)) => group = None,
            (Gear::Good, Some(_)) | (Gear::Broken, Some(0)) => return false,
            (Gear::Broken | Gear::Unknown, Some(group)) => *group -= 1,
            (Gear::Broken | Gear::Unknown, None) => {
                if matches!(gear, Gear::Unknown) {
                    let next = i + 1;
                    valid += count_combinations(row, Prefix { next, next_group }, cache);
                }

                if let Some(new_group) = row.groups.get(next_group as usize).copied() {
                    group = Some(new_group.get() - 1);
                    next_group += 1;
                } else {
                    return false;
                }
            }
        };

        true
    });

    // This run was valid if it did not short-circuit, and the last group was fully emptied
    valid += match group {
        Some(0) | None if done && next_group as usize >= row.groups.len() => 1,
        _ => 0,
    };
    cache.insert(prefix, valid);
    valid
}

fn parse(s: std::borrow::Cow<'static, str>) -> Result<Vec<Row>> {
    s.lines()
        .filter_map(|line| {
            let (row, groups) = line.trim().split_once(' ')?;
            let res = row
                .chars()
                .map(|c| match c {
                    '.' => Ok(Gear::Good),
                    '#' => Ok(Gear::Broken),
                    '?' => Ok(Gear::Unknown),
                    bad => Err(err!("unknown spring character '{bad}'")),
                })
                .collect::<Result<Vec<Gear>>>()
                .and_then(|row| {
                    Ok(Row {
                        row,
                        groups: groups
                            .split(',')
                            .map(|s| s.trim().parse())
                            .collect::<Result<Vec<_>, _>>()
                            .wrap_err("parsing contiguous broken")?,
                    })
                })
                .wrap_err_with(|| format!("for {line:?}"));

            Some(res)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../examples/12.txt");
    const MAIN: &str = include_str!("../inputs/12.txt");

    #[test]
    fn first_part_example() {
        let data = parse(EXAMPLE.into()).unwrap();
        assert_eq!(first_part(&data), 21);
    }

    #[test]
    fn first_part_main() {
        let data = parse(MAIN.into()).unwrap();
        assert_eq!(first_part(&data), 7_025);
    }

    #[test]
    fn second_part_example() {
        let data = parse(EXAMPLE.into()).unwrap();
        assert_eq!(second_part(data), 525_152);
    }

    #[test]
    fn second_part_main() {
        let data = parse(MAIN.into()).unwrap();
        assert_eq!(second_part(data), 11_461_095_383_315);
    }
}
