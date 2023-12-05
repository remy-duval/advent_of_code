use std::collections::HashMap;

use itertools::Itertools;

use commons::error::Result;
use commons::parse::sep_by_empty_lines;
use commons::{ensure, WrapErr};

pub const TITLE: &str = "Day 5: If You Give A Seed A Fertilizer";

pub fn run(raw: String) -> Result<()> {
    let data = parse(&raw)?;
    let first = first_part(&data)?;
    println!("1. The minimum seed location is {first}");
    let second = second_part(&data)?;
    println!("2. The corrected minimum seed location is {second}");

    Ok(())
}

#[derive(Debug)]
struct Almanac<'a> {
    seeds: Vec<u64>,
    mappings: HashMap<&'a str, (&'a str, Vec<Mapping>)>,
}

#[derive(Debug)]
struct Mapping {
    from: Range,
    dest: u64,
}

#[derive(Debug)]
struct Range {
    start: u64,
    end: u64,
}

impl Range {
    fn new(start: u64, end: u64) -> Self {
        Self { start, end }
    }
}

fn first_part(almanac: &Almanac) -> Result<u64> {
    almanac
        .seeds
        .iter()
        .filter_map(|seed| {
            // Move each value through the mappings
            let mut current = "seed";
            let mut value = *seed;
            while current != "location" {
                let (next, map) = almanac.mappings.get(current)?;
                current = next;
                value = map
                    .iter()
                    .find_map(|m| {
                        if value >= m.from.start && value <= m.from.end {
                            Some(value - m.from.start + m.dest)
                        } else {
                            None
                        }
                    })
                    .unwrap_or(value);
            }
            Some(value)
        })
        .min()
        .wrap_err_with(|| format!("failed to map to location in {almanac:?}"))
}

fn second_part(almanac: &Almanac) -> Result<u64> {
    let mut seeds: Vec<Range> = almanac
        .seeds
        .chunks(2)
        .filter_map(|s| match *s {
            [start, length] => Some(Range::new(start, start + length)),
            _ => None,
        })
        .collect();

    let mut swap = Vec::with_capacity(seeds.len());

    let mut current = "seed";
    while current != "location" {
        let (next, mappings) = almanac
            .mappings
            .get(current)
            .wrap_err_with(|| format!("missing mapping from {current:?} in {almanac:?}"))?;
        current = next;

        // Intersect the current seeds ranges with the mappings
        while let Some(mut r) = seeds.pop() {
            for m in mappings.iter() {
                // Try to translate the range, leaving the untranslated part behind for the next try
                if r.start >= r.end || r.start > m.from.end || r.end < m.from.start {
                    // Empty range, or non overlapping ranges, skip
                    continue;
                } else if r.end < m.from.end {
                    let end = m.dest + r.end - m.from.start;
                    let start = if r.start < m.from.start {
                        // X  X
                        //  Y    Y
                        m.dest
                    } else {
                        //  X X
                        // Y    Y
                        m.dest + r.start - m.from.start
                    };
                    r.end = m.from.start; // The prefix may remain untranslated
                    swap.push(Range::new(start, end));
                } else {
                    let end = m.dest + m.from.end - m.from.start;
                    let start = if r.start < m.from.start {
                        // X     X
                        //  Y  Y
                        // The prefix is still untranslated
                        seeds.push(Range::new(r.start, m.from.start));
                        m.dest
                    } else {
                        //   X   X
                        // Y   Y
                        m.dest + r.start - m.from.start
                    };
                    r.start = m.from.end; // The suffix may remain untranslated
                    swap.push(Range::new(start, end));
                }
            }
            // If an untranslated part remains after all mappings, keep it
            if r.start < r.end {
                swap.push(r);
            }
        }

        ensure!(
            !swap.is_empty(),
            "no ranges remaining after conversion to {current:?}"
        );

        // Fuse overlapping to reduce the number of ranges
        std::mem::swap(&mut swap, &mut seeds);
        seeds.sort_unstable_by_key(|r| r.start);
        let mut i = 0;
        while i < seeds.len() - 1 {
            if seeds[i].end >= seeds[i + 1].start {
                seeds[i].end = seeds[i].end.max(seeds[i + 1].end);
                seeds.remove(i + 1);
            } else {
                i += 1;
            }
        }
    }

    seeds
        .into_iter()
        .map(|r| r.start)
        .min()
        .wrap_err_with(|| format!("failed to map to location in {almanac:?}"))
}

fn parse(s: &str) -> Result<Almanac> {
    let mut sections = sep_by_empty_lines(s);
    let seeds = sections
        .next()
        .and_then(|seeds| seeds.strip_prefix("seeds:"))
        .wrap_err("missing initial seeds section")
        .and_then(|seeds| {
            seeds
                .trim()
                .split_ascii_whitespace()
                .map(str::parse)
                .collect::<Result<Vec<u64>, _>>()
                .wrap_err_with(|| format!("seeds: {seeds:?}"))
        })?;

    let mappings = sections
        .filter_map(|section| {
            let mut lines = section.lines();
            let (from, to) = lines
                .next()?
                .trim()
                .strip_suffix("map:")?
                .split_once("-to-")?;
            let mapping = lines
                .map(|line| {
                    let num = line
                        .trim()
                        .split_ascii_whitespace()
                        .map(|n| n.parse().wrap_err("bad number"));
                    itertools::process_results(num, |num| {
                        num.collect_tuple::<(_, _, _)>()
                            .map(|(dest, start, length)| Mapping {
                                from: Range::new(start, start + length),
                                dest,
                            })
                            .wrap_err("not enough numbers")
                    })
                    .and_then(|i| i)
                    .wrap_err_with(|| format!("section {section:?}"))
                })
                .collect::<Result<Vec<Mapping>>>()
                .map(|map| (from.trim(), (to.trim(), map)));

            Some(mapping)
        })
        .collect::<Result<_>>()?;

    Ok(Almanac { seeds, mappings })
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../examples/05.txt");
    const MAIN: &str = include_str!("../inputs/05.txt");

    #[test]
    fn first_part_example() {
        let data = parse(EXAMPLE).unwrap();
        assert_eq!(first_part(&data).unwrap(), 35);
    }

    #[test]
    fn first_part_main() {
        let data = parse(MAIN).unwrap();
        assert_eq!(first_part(&data).unwrap(), 662_197_086);
    }

    #[test]
    fn second_part_example() {
        let data = parse(EXAMPLE).unwrap();
        assert_eq!(second_part(&data).unwrap(), 46);
    }

    #[test]
    fn second_part_main() {
        let data = parse(MAIN).unwrap();
        assert_eq!(second_part(&data).unwrap(), 52_510_809);
    }
}
