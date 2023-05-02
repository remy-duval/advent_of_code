use std::collections::HashMap;

use commons::{err, Result};

pub const TITLE: &str = "Day 6: Universal Orbit Map";
type PlanetName<'a> = &'a str;
const COM: &str = "COM";

pub fn run(raw: String) -> Result<()> {
    let orbits = parse(&raw);
    let from_origin = depth_first_search(COM, orbits).ok_or_else(|| err!("DFS error !"))?;
    let first = check_sum(&from_origin);
    let second =
        shortest_path(&from_origin, "YOU", "SAN").ok_or_else(|| err!("YOU or SAN not found"))?;

    println!("The orbit check sum is {first}");
    println!("The shortest path from YOU to SAN is {second}");
    Ok(())
}

/// Parse the data into a map of Planets -> All planets orbiting it directly
fn parse(s: &str) -> HashMap<PlanetName, Vec<PlanetName>> {
    let raw_values: Vec<(PlanetName, PlanetName)> = s
        .lines()
        .filter_map(|orbit_definition| {
            let mut split = orbit_definition.split(')').take(2);
            let center = split.next()?;
            let planet = split.next()?;

            Some((center, planet))
        })
        .collect();

    let map = raw_values
        .iter()
        .map(|(key, _)| {
            let all_orbiting = raw_values
                .iter()
                .filter_map(|(a, b)| if a == key { Some(*b) } else { None })
                .collect::<Vec<_>>();
            (*key, all_orbiting)
        })
        .collect::<HashMap<_, _>>();

    map
}

/// Sums the length of all path in the DFS produced map.
fn check_sum(from_origin: &HashMap<PlanetName, Vec<PlanetName>>) -> usize {
    from_origin.values().map(|road| road.len()).sum()
}

/// Using the map produced from DFS, returns the length of the shortest path from start to end.
fn shortest_path(
    from_origin: &HashMap<PlanetName, Vec<PlanetName>>,
    start: &str,
    end: &str,
) -> Option<usize> {
    let to_start = from_origin.get(&start)?;
    let to_end = from_origin.get(&end)?;
    let common_part = to_start.iter().zip(to_end).filter(|x| x.0 == x.1).count();

    Some(to_start.len() + to_end.len() - 2 * common_part)
}

/// Starts from the given center and builds a map of Planet -> Path to planet from center via DFS
fn depth_first_search<'a>(
    center: &'a str,
    orbits: HashMap<PlanetName<'a>, Vec<PlanetName<'a>>>,
) -> Option<HashMap<PlanetName<'a>, Vec<PlanetName<'a>>>> {
    let center: PlanetName = center;
    let mut origins: HashMap<PlanetName, Vec<PlanetName>> = HashMap::new();
    origins.insert(center, vec![]);

    let mut stack: Vec<PlanetName> = vec![center];
    while let Some(elt) = stack.pop() {
        if let Some(orbits) = orbits.get(&elt) {
            for next in orbits {
                stack.push(*next); // Enqueue next to visit
                let next_road = {
                    let mut base = origins.get(&elt)?.clone();
                    base.push(elt);
                    base
                };
                origins.insert(*next, next_road);
            }
        }
    }

    Some(origins)
}

#[cfg(test)]
mod tests;
