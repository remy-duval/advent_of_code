use std::rc::Rc;

use commons::eyre::{eyre, Result};
use hashbrown::HashMap;

use commons::parse::LineSep;
use commons::Problem;

type PlanetName = Rc<String>;

const COM: &str = "COM";

pub struct Day;

impl Problem for Day {
    type Input = LineSep<String>;
    const TITLE: &'static str = "Day 6: Universal Orbit Map";

    fn solve(data: Self::Input) -> Result<()> {
        let orbits = parse_map(&data.data);
        let from_origin = depth_first_search(COM, orbits).ok_or_else(|| eyre!("DFS error !"))?;
        let first = check_sum(&from_origin);
        let second = shortest_path(&from_origin, "YOU", "SAN")
            .ok_or_else(|| eyre!("YOU or SAN not found"))?;

        println!("The orbit check sum is {}", first);
        println!("The shortest path from YOU to SAN is {}", second);
        Ok(())
    }
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
    let to_start = from_origin.get(&start.to_string())?;
    let to_end = from_origin.get(&end.to_string())?;
    let common_part = to_start.iter().zip(to_end).filter(|x| x.0 == x.1).count();

    Some(to_start.len() + to_end.len() - 2 * common_part)
}

/// Starts from the given center and builds a map of Planet -> Path to planet from center via DFS
fn depth_first_search(
    center: &str,
    orbits: HashMap<PlanetName, Vec<PlanetName>>,
) -> Option<HashMap<PlanetName, Vec<PlanetName>>> {
    let center: PlanetName = Rc::new(center.to_string());
    let mut origins: HashMap<PlanetName, Vec<PlanetName>> = HashMap::new();
    origins.insert(center.clone(), vec![]);

    let mut stack: Vec<PlanetName> = vec![center];
    while let Some(elt) = stack.pop() {
        if let Some(orbits) = orbits.get(&elt) {
            for next in orbits {
                stack.push(Rc::clone(next)); // Enqueue next to visit
                let next_road = {
                    let mut base = origins.get(&elt)?.clone();
                    base.push(elt.clone());
                    base
                };
                origins.insert(Rc::clone(next), next_road);
            }
        }
    }

    Some(origins)
}

/// Parse the data into a map of Planets -> All planets orbiting it directly
fn parse_map(data: &[String]) -> HashMap<PlanetName, Vec<PlanetName>> {
    let raw_values: Vec<(PlanetName, PlanetName)> = data
        .iter()
        .filter_map(|orbit_definition| {
            let mut split = orbit_definition.split(')').take(2);
            let center = split.next()?;
            let planet = split.next()?;

            Some((Rc::new(center.to_string()), Rc::new(planet.to_string())))
        })
        .collect();

    let map = raw_values
        .iter()
        .map(|(key, _)| {
            let all_orbiting = raw_values
                .iter()
                .filter_map(|(a, b)| if a == key { Some(Rc::clone(b)) } else { None })
                .collect::<Vec<PlanetName>>();
            (Rc::clone(key), all_orbiting)
        })
        .collect::<HashMap<_, _>>();

    map
}

#[cfg(test)]
mod tests;
