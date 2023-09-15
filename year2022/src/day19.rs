use std::str::FromStr;

use itertools::Itertools;

use commons::error::Result;
use commons::{Report, WrapErr};

pub const TITLE: &str = "Day 19: Not Enough Minerals";

type Count = u32;

pub fn run(raw: String) -> Result<()> {
    let data = parse(raw.into())?;
    let first = first_part(&data);
    println!("1. Quality levels sum of blueprints is {first}");
    let second = second_part(&data);
    println!("2. Product of maximum geodes produces is {second}");

    Ok(())
}

fn first_part(blueprints: &[Blueprint]) -> u32 {
    blueprints
        .iter()
        .map(|bp: &Blueprint| find_max_geodes(bp, 24))
        .map(|(id, geodes)| id as u32 * geodes)
        .sum()
}

fn second_part(blueprints: &[Blueprint]) -> u32 {
    blueprints
        .iter()
        .take(3)
        .map(|bp: &Blueprint| find_max_geodes(bp, 32).1)
        .product()
}

fn find_max_geodes(bp: &Blueprint, turns: u8) -> (u8, u32) {
    let mut stack = vec![(turns, State::START, PreviousTurn::BuiltNonGeode)];
    let mut max_geodes = 0;
    while let Some((remaining, state, last)) = stack.pop() {
        let next = state.next_step();
        let remaining = remaining - 1;
        if remaining == 0 {
            max_geodes = max_geodes.max(next.resources.geode);
            continue;
        }

        // If a geode robot can be built, this is optimal (as geode are the goal)
        if let Some(next) = bp.build_geode_robot(&state) {
            stack.push((remaining, next, PreviousTurn::BuiltGeode));
        } else {
            // Building something the turn after having skipped it is sub-optimal
            let mut skip_ore = last.skipped_ore() || bp.enough_ore_robots(&state);
            let mut skip_clay = last.skipped_clay() || bp.enough_clay_robots(&state);
            let mut skip_obs = last.skipped_obsidian() || bp.enough_obsidian_robots(&state);

            if !skip_obs {
                if let Some(next) = bp.build_obsidian_robot(&state) {
                    skip_obs = true;
                    stack.push((remaining, next, PreviousTurn::BuiltNonGeode));
                }
            }
            if !skip_clay {
                if let Some(next) = bp.build_clay_robot(&state) {
                    skip_clay = true;
                    stack.push((remaining, next, PreviousTurn::BuiltNonGeode));
                }
            }
            if !skip_ore {
                if let Some(next) = bp.build_ore_robot(&state) {
                    skip_ore = true;
                    stack.push((remaining, next, PreviousTurn::BuiltNonGeode));
                }
            }

            let current = PreviousTurn::new(skip_ore, skip_clay, skip_obs);
            // Try to avoid passing turns when we could have done something productive
            if current == PreviousTurn::SkippedAll && last != PreviousTurn::BuiltNonGeode {
                continue;
            }
            stack.push((remaining, next, current));
        }
    }

    (bp.id, max_geodes)
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum PreviousTurn {
    BuiltNonGeode,
    BuiltGeode,
    SkippedAll,
    SkippedOre,
    SkippedClay,
    SkippedObsidian,
    SkippedOreClay,
    SkippedOreObsidian,
    SkippedClayObsidian,
}

impl PreviousTurn {
    fn new(skip_ore: bool, skip_clay: bool, skip_obsidian: bool) -> Self {
        match (skip_ore, skip_clay, skip_obsidian) {
            (true, true, true) => Self::SkippedAll,
            (true, false, true) => Self::SkippedOreObsidian,
            (true, true, false) => Self::SkippedOreClay,
            (false, true, true) => Self::SkippedClayObsidian,
            (true, false, false) => Self::SkippedOre,
            (false, true, false) => Self::SkippedClay,
            (false, false, true) => Self::SkippedObsidian,
            (false, false, false) => Self::BuiltNonGeode,
        }
    }

    fn skipped_ore(self) -> bool {
        matches!(
            self,
            Self::SkippedOre | Self::SkippedOreClay | Self::SkippedOreObsidian | Self::SkippedAll
        )
    }

    fn skipped_clay(self) -> bool {
        matches!(
            self,
            Self::SkippedClay | Self::SkippedOreClay | Self::SkippedClayObsidian | Self::SkippedAll
        )
    }

    fn skipped_obsidian(self) -> bool {
        matches!(
            self,
            Self::SkippedObsidian
                | Self::SkippedOreObsidian
                | Self::SkippedClayObsidian
                | Self::SkippedAll
        )
    }
}

#[derive(Debug)]
struct Blueprint {
    id: u8,
    ore_robot_ore_cost: u8,
    clay_robot_ore_cost: u8,
    obsidian_robot_ore_cost: u8,
    obsidian_robot_clay_cost: u8,
    geode_robot_ore_cost: u8,
    geode_robot_obsidian_cost: u8,
    max_ore_cost: u8,
}

impl Blueprint {
    fn enough_ore_robots(&self, state: &State) -> bool {
        state.robots.ore >= self.max_ore_cost as Count
    }

    fn enough_clay_robots(&self, state: &State) -> bool {
        state.robots.clay >= self.obsidian_robot_clay_cost as Count
    }

    fn enough_obsidian_robots(&self, state: &State) -> bool {
        state.robots.obsidian >= self.geode_robot_obsidian_cost as Count
    }

    fn build_ore_robot(&self, state: &State) -> Option<State> {
        let cost = self.ore_robot_ore_cost as Count;
        if state.resources.ore < cost {
            return None;
        }

        let mut next = state.next_step();
        next.resources.ore -= cost;
        next.robots.ore += 1;
        Some(next)
    }

    fn build_clay_robot(&self, state: &State) -> Option<State> {
        let cost = self.clay_robot_ore_cost as Count;
        if state.resources.ore < cost {
            return None;
        }

        let mut next = state.next_step();
        next.resources.ore -= cost;
        next.robots.clay += 1;
        Some(next)
    }

    fn build_obsidian_robot(&self, state: &State) -> Option<State> {
        let ore_cost = self.obsidian_robot_ore_cost as Count;
        let clay_cost = self.obsidian_robot_clay_cost as Count;
        if state.resources.ore < ore_cost || state.resources.clay < clay_cost {
            return None;
        }

        let mut next = state.next_step();
        next.resources.ore -= ore_cost;
        next.resources.clay -= clay_cost;
        next.robots.obsidian += 1;
        Some(next)
    }

    fn build_geode_robot(&self, state: &State) -> Option<State> {
        let ore_cost = self.geode_robot_ore_cost as Count;
        let obsidian_cost = self.geode_robot_obsidian_cost as Count;
        if state.resources.ore < ore_cost || state.resources.obsidian < obsidian_cost {
            return None;
        }

        let mut next = state.next_step();
        next.resources.ore -= ore_cost;
        next.resources.obsidian -= obsidian_cost;
        next.robots.geode += 1;
        Some(next)
    }
}

#[derive(Clone)]
struct State {
    resources: Counts,
    robots: Counts,
}

impl State {
    const START: Self = Self {
        resources: Counts {
            ore: 0,
            clay: 0,
            obsidian: 0,
            geode: 0,
        },
        robots: Counts {
            ore: 1,
            clay: 0,
            obsidian: 0,
            geode: 0,
        },
    };

    fn next_step(&self) -> Self {
        let mut next = self.clone();
        next.resources.ore += next.robots.ore;
        next.resources.clay += next.robots.clay;
        next.resources.obsidian += next.robots.obsidian;
        next.resources.geode += next.robots.geode;
        next
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Counts {
    ore: Count,
    clay: Count,
    obsidian: Count,
    geode: Count,
}

impl FromStr for Blueprint {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self> {
        let (name, recipes) = s.trim().split_once(':').wrap_err("missing ':'")?;
        let id = name
            .strip_prefix("Blueprint")
            .wrap_err("missing blueprint ID")?;
        let id: u8 = id.trim().parse().wrap_err("bad blueprint ID")?;
        let (ore, clay, obsidian, geode) = recipes
            .splitn(4, '.')
            .map(|s| s.trim_end_matches(['.', ' ']).trim())
            .collect_tuple::<(_, _, _, _)>()
            .wrap_err("missing recipe")?;

        let ore = ore
            .strip_prefix("Each ore robot costs")
            .and_then(|s| s.strip_suffix("ore"))
            .wrap_err("bad ore recipe format")?
            .trim()
            .parse::<u8>()
            .wrap_err("bad ore recipe number")?;
        let clay = clay
            .strip_prefix("Each clay robot costs")
            .and_then(|s| s.strip_suffix("ore"))
            .wrap_err("bad clay recipe format")?
            .trim()
            .parse::<u8>()
            .wrap_err("bad clay recipe number")?;
        let (obsidian_ore, obsidian_clay) = obsidian
            .strip_prefix("Each obsidian robot costs")
            .and_then(|s| {
                let (ore, clay) = s.split_once("and")?;
                let ore = ore.trim().strip_suffix("ore")?.trim().parse::<u8>();
                let clay = clay.trim().strip_suffix("clay")?.trim().parse::<u8>();
                match (ore, clay) {
                    (Ok(ore), Ok(clay)) => Some(Ok((ore, clay))),
                    (Err(e), _) | (_, Err(e)) => Some(Err(e)),
                }
            })
            .wrap_err("bad obsidian recipe")?
            .wrap_err("bad obsidian recipe number")?;
        let (geode_ore, geode_obsidian) = geode
            .strip_prefix("Each geode robot costs")
            .and_then(|s| {
                let (ore, obsidian) = s.split_once("and")?;
                let ore = ore.trim().strip_suffix("ore")?.trim().parse::<u8>();
                let obsidian = obsidian
                    .trim()
                    .strip_suffix("obsidian")?
                    .trim()
                    .parse::<u8>();
                match (ore, obsidian) {
                    (Ok(ore), Ok(obsidian)) => Some(Ok((ore, obsidian))),
                    (Err(e), _) | (_, Err(e)) => Some(Err(e)),
                }
            })
            .wrap_err("bad geode recipe")?
            .wrap_err("bad geode recipe number")?;

        Ok(Self {
            id,
            ore_robot_ore_cost: ore,
            clay_robot_ore_cost: clay,
            obsidian_robot_ore_cost: obsidian_ore,
            obsidian_robot_clay_cost: obsidian_clay,
            geode_robot_ore_cost: geode_ore,
            geode_robot_obsidian_cost: geode_obsidian,
            max_ore_cost: ore.max(clay).max(obsidian_ore),
        })
    }
}

fn parse(s: std::borrow::Cow<'static, str>) -> Result<Vec<Blueprint>> {
    s.match_indices("Blueprint")
        .chain(std::iter::once((s.len(), "")))
        .tuple_windows::<(_, _)>()
        .filter_map(|((start, _), (end, _))| s.get(start..end))
        .map(|s| s.parse().wrap_err_with(|| format!("bad blueprint:\n{s}")))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../examples/19.txt");
    const MAIN: &str = include_str!("../inputs/19.txt");

    #[test]
    fn first_part_example() {
        let data = parse(EXAMPLE.into()).unwrap();
        assert_eq!(first_part(&data), 33);
    }

    #[test]
    fn first_part_main() {
        let data = parse(MAIN.into()).unwrap();
        assert_eq!(first_part(&data), 1659);
    }

    #[test]
    fn second_part_example() {
        let data = parse(EXAMPLE.into()).unwrap();
        assert_eq!(second_part(&data), 56 * 62);
    }

    #[test]
    fn second_part_main() {
        let data = parse(MAIN.into()).unwrap();
        assert_eq!(second_part(&data), 6804);
    }
}
