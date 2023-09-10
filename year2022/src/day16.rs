use std::collections::{HashMap, HashSet, VecDeque};
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
    let mut cache = HashMap::new();
    let opened = ValveSet::EMPTY.add(dist.start);
    dist.compute(&mut cache, dist.start, 31, opened, opened)
}

fn second_part(dist: &Distances) -> u32 {
    let mut cache = HashMap::new();
    let all = (0..dist.to_open.len()).fold(ValveSet::EMPTY, |acc, i| acc.add(i));
    (1..dist.to_open.len())
        .flat_map(|i| (0..dist.to_open.len()).combinations(i))
        .map(|e| e.into_iter().fold(ValveSet::EMPTY, |acc, i| acc.add(i)))
        .map(|excluded| {
            let first = dist.compute(
                &mut cache,
                dist.start,
                27,
                ValveSet::EMPTY.add(dist.start),
                excluded,
            );
            let second = dist.compute(
                &mut cache,
                dist.start,
                27,
                ValveSet::EMPTY.add(dist.start),
                excluded.xor(all),
            );
            first + second
        })
        .max()
        .unwrap_or_default()
}

#[derive(Debug)]
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

    fn compute(
        &self,
        cache: &mut HashMap<(usize, u32, ValveSet, ValveSet), u32>,
        pos: usize,
        time_left: u32,
        opened: ValveSet,
        excluded: ValveSet,
    ) -> u32 {
        match cache.get(&(pos, time_left, opened, excluded)).copied() {
            Some(cached) => cached,
            None => {
                let mut max = 0;
                if let Some(time_left) = time_left.checked_sub(1) {
                    let flow = self.to_open[pos].flow_rate * time_left;
                    max = flow;
                    for &(next, distance) in self.distances[pos].iter() {
                        if opened.contains(next) || excluded.contains(next) {
                            continue;
                        }
                        let opened = opened.add(next);
                        if let Some(time_left) = time_left.checked_sub(distance) {
                            let after = self.compute(cache, next, time_left, opened, excluded);
                            max = max.max(after + flow);
                        }
                    }
                }

                cache.insert((pos, time_left, opened, excluded), max);
                max
            }
        }
    }
}

#[derive(Debug, Clone)]
struct Valve {
    name: Id,
    flow_rate: u32,
    tunnels: Vec<Id>,
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct ValveSet(u64);

impl ValveSet {
    const EMPTY: ValveSet = ValveSet(0);

    fn add(self, index: usize) -> ValveSet {
        Self(self.0 | (1 << index as u64))
    }

    fn contains(self, index: usize) -> bool {
        self.0 & (1 << index as u64) != 0
    }

    fn xor(self, other: Self) -> Self {
        Self(self.0 ^ other.0)
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
