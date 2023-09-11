use std::collections::{HashMap, HashSet, VecDeque};
use std::ops::BitAnd;
use std::str::FromStr;

use itertools::Itertools;

use commons::error::Result;
use commons::parse::LineSep;
use commons::{ensure, Report, WrapErr};

pub const TITLE: &str = "Day 16: Proboscidea Volcanium";

pub fn run(raw: String) -> Result<()> {
    let data = parse(raw.into())?;
    let first = first_part(&data);
    println!("1. The most pressure in 30 minutes is {first}");
    let second = second_part(&data);
    println!("2. The most pressure with two players in 26 minutes is {second}");

    Ok(())
}

fn first_part(dist: &Distances) -> u32 {
    // This will compute the maximum value reached for each set of valve
    // The overall maximum of this mapping will be the answer
    dist.compute_max_values(30)
        .into_values()
        .max()
        .unwrap_or_default()
}

fn second_part(dist: &Distances) -> u32 {
    // This time two explorations are going on simultaneously
    // Pairs of distinct explorations (only sharing the start) can be summed to compute a duo
    // Find the maximum value of all those pairs by just doing a cartesian product of the maxes
    let max_values = dist.compute_max_values(26);
    let only_start = ValveSet::EMPTY.add(dist.start);
    max_values
        .iter()
        .flat_map(|(&set1, &total1)| {
            max_values
                .iter()
                .filter(move |(set2, _)| (set1 & **set2) == only_start)
                .map(move |(_, &total2)| total1 + total2)
        })
        .max()
        .unwrap_or_default()
}

/// A set of distances between all non broken valves (flow rates != 0)
struct Distances {
    start: usize,
    to_open: Vec<Valve>,
    distances: Vec<Vec<(usize, u32)>>,
}

impl Distances {
    fn build(valves: Vec<Valve>) -> Result<Self> {
        let to_open: Vec<Valve> = valves
            .iter()
            .filter(|v| v.flow_rate != 0 || v.name == Id::AA)
            .cloned()
            .collect();

        let start = to_open
            .iter()
            .position(|v| v.name == Id::AA)
            .wrap_err("missing valve AA")?;

        ensure!(to_open.len() <= 64, "too many valves for 64 values bitset");
        // Compute the time to open using one BFS per start node
        let distances: Vec<Vec<(usize, u32)>> = to_open
            .iter()
            .map(|from| {
                let mut seen = HashSet::new();
                let mut stack = VecDeque::new();
                let mut times_to_open = Vec::with_capacity(to_open.len() - 1);
                stack.push_back((from.name, 0u32));
                seen.insert(from.name);
                while let Some((id, distance)) = stack.pop_front() {
                    if let Some(to) = valves.iter().find(|v| v.name == id) {
                        for &next in to.tunnels.iter() {
                            if seen.insert(next) {
                                stack.push_back((next, distance + 1));
                            }
                        }
                    }
                    if let Some(j) = to_open.iter().position(|v| v.name == id) {
                        times_to_open.push((j, distance));
                    }
                }
                times_to_open
            })
            .collect();

        Ok(Self {
            start,
            to_open,
            distances,
        })
    }

    /// Find the maximum value reached for each combination of valves explored in the time given
    fn compute_max_values(&self, time_available: u32) -> HashMap<ValveSet, u32> {
        let mut max_values = HashMap::new();
        let mut stack = vec![(self.start, time_available, 0, ValveSet::EMPTY)];
        while let Some((pos, time_left, current, opened)) = stack.pop() {
            let current = current + self.to_open[pos].flow_rate * time_left;
            let opened = opened.add(pos);
            max_values
                .entry(opened)
                .and_modify(|max| *max = current.max(*max))
                .or_insert(current);

            for &(next, time_consumed) in self.distances[pos].iter() {
                if opened.contains(next) {
                    continue;
                }
                if let Some(time_left) = time_left.checked_sub(time_consumed + 1) {
                    stack.push((next, time_left, current, opened));
                }
            }
        }

        max_values
    }
}

#[derive(Clone)]
struct Valve {
    name: Id,
    flow_rate: u32,
    tunnels: Vec<Id>,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct ValveSet(u64);

impl ValveSet {
    const EMPTY: ValveSet = ValveSet(0);

    fn add(self, index: usize) -> ValveSet {
        Self(self.0 | (1 << index as u64))
    }

    fn contains(self, index: usize) -> bool {
        self.0 & (1 << index as u64) != 0
    }
}

impl BitAnd for ValveSet {
    type Output = ValveSet;

    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct Id(u16);

impl Id {
    const AA: Id = Id(u16::from_ne_bytes([0, 0]));
}

impl std::fmt::Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let [first, second] = self.0.to_ne_bytes();
        write!(f, "{}{}", (b'A' + first) as char, (b'A' + second) as char)
    }
}

impl std::fmt::Debug for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <Self as std::fmt::Display>::fmt(self, f)
    }
}

impl FromStr for Id {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self> {
        let (a, b) = s
            .chars()
            .collect_tuple::<(_, _)>()
            .filter(|(a, b)| s.len() == 2 && a.is_ascii_uppercase() && b.is_ascii_uppercase())
            .wrap_err_with(|| format!("expected two uppercase characters, got {s:?}"))?;
        Ok(Self(u16::from_ne_bytes([a as u8 - b'A', b as u8 - b'A'])))
    }
}

impl FromStr for Valve {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self> {
        s.strip_prefix("Valve")
            .and_then(|s| {
                let (start, end) = s.split_once(';')?;
                let end = end.trim_start();
                let (name, flow_rate) = start.split_once("has flow rate=")?;
                let tunnels = end
                    .strip_prefix("tunnels lead to valves")
                    .or_else(|| end.strip_prefix("tunnel leads to valve"))?;
                Some((
                    name.trim().parse::<Id>().wrap_err("for valve name"),
                    flow_rate.trim().parse::<u32>().wrap_err("for flow rate"),
                    tunnels
                        .split(',')
                        .map(|t| t.trim().parse())
                        .collect::<Result<Vec<Id>>>()
                        .wrap_err("for tunnel names"),
                ))
            })
            .wrap_err("wanted 'Valve {v} has flow rate={flow}; tunnel(s) lead to valve(s) {id}'")
            .and_then(|(name, flow_rate, tunnels)| {
                Ok(Self {
                    name: name?,
                    flow_rate: flow_rate?,
                    tunnels: tunnels?,
                })
            })
            .wrap_err_with(|| format!("for {s:?}"))
    }
}

fn parse(s: std::borrow::Cow<'static, str>) -> Result<Distances> {
    let split: LineSep<Valve> = s.parse()?;
    Distances::build(split.data)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../examples/16.txt");
    const MAIN: &str = include_str!("../inputs/16.txt");

    #[test]
    fn first_part_example() {
        let data = parse(EXAMPLE.into()).unwrap();
        assert_eq!(first_part(&data), 1651);
    }

    #[test]
    fn first_part_main() {
        let data = parse(MAIN.into()).unwrap();
        assert_eq!(first_part(&data), 1653);
    }

    #[test]
    fn second_part_example() {
        let data = parse(EXAMPLE.into()).unwrap();
        assert_eq!(second_part(&data), 1707);
    }

    #[test]
    fn second_part_main() {
        let data = parse(MAIN.into()).unwrap();
        assert_eq!(second_part(&data), 2223);
    }
}
