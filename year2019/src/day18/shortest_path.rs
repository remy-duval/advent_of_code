//! All the methods for getting the shortest path on a maze

use commons::grid::Point;
use hashbrown::{HashMap, HashSet};

use super::{HallWay, Keys};

/// Finds the shortest with the following starting point in the map
#[allow(clippy::implicit_hasher)]
pub fn find_shortest_path(map: &HashMap<Point, HallWay>, start: Point, keys: Keys) -> usize {
    let mut cache = SolvingCache::new(map.clone());
    cache.compute_paths();
    cache.find_minimum(start, keys)
}

#[derive(Debug, Default, Copy, Clone)]
struct Trip {
    required: Keys,
    obtained: Keys,
    distance: usize,
}

impl Trip {
    /// Copy this trip, adding the given distance to it
    fn added_distance(self, distance: usize) -> Self {
        Self {
            distance: self.distance + distance,
            ..self
        }
    }
}

#[derive(Debug)]
struct SolvingCache {
    map: HashMap<Point, HallWay>,
    edges: HashMap<Point, Keys>,
    shortest: HashMap<(Point, Point), Trip>,
    minimums: HashMap<(Point, Keys), usize>,
}

impl SolvingCache {
    fn new(map: HashMap<Point, HallWay>) -> Self {
        // The map of all points we need to go through during the path solving
        // This includes the starting point ('@') and all the keys
        let edges: HashMap<_, _> = map
            .iter()
            .filter_map(|(point, hall)| {
                if hall.char.is_ascii_lowercase() {
                    Some((*point, hall.contains))
                } else if hall.char == '@' {
                    Some((*point, Keys::default()))
                } else {
                    None
                }
            })
            .collect();

        Self {
            map,
            edges,
            shortest: HashMap::new(),
            minimums: HashMap::new(),
        }
    }

    /// Computes all paths between keys and store them in the shortest map
    fn compute_paths(&mut self) {
        for (start, _) in self.edges.iter() {
            for (end, _) in self.edges.iter() {
                if *start != *end {
                    let trip = shortest(&self.map, *start, *end).unwrap();
                    self.shortest.insert((*start, *end), trip);
                }
            }
        }
    }

    /// Finds the minimums of paths.
    #[allow(clippy::map_entry)]
    #[allow(clippy::needless_collect)]
    fn find_minimum(&mut self, start: Point, start_keys: Keys) -> usize {
        if start_keys.is_full() {
            0
        } else if !self.minimums.contains_key(&(start, start_keys)) {
            let accessible: Vec<_> = self
                .edges
                .iter()
                .filter(|(_, &keys)| !start_keys.contains(keys))
                .filter_map(|(&end, _)| {
                    let trip = self.shortest.get(&(start, end))?;
                    if start_keys.contains(trip.required) {
                        let new_keys = trip.obtained + start_keys;
                        let partial = trip.distance;
                        Some((end, new_keys, partial))
                    } else {
                        None
                    }
                })
                .collect();

            let min = accessible
                .into_iter()
                .map(|(end, keys, partial)| partial + self.find_minimum(end, keys))
                .min()
                .unwrap();

            self.minimums.insert((start, start_keys), min);
            min
        } else {
            *self.minimums.get(&(start, start_keys)).unwrap()
        }
    }
}

/// Finds the shortest trip between two points.
fn shortest(map: &HashMap<Point, HallWay>, start: Point, end: Point) -> Option<Trip> {
    let mut visited: HashSet<Point> = HashSet::new();
    visited.insert(start);
    let mut next: Vec<(Point, Trip)> = vec![(start, Trip::default())];

    while let Some((current, mut trip)) = next.pop() {
        if let Some(hall) = map.get(&current) {
            trip = Trip {
                required: trip.required + hall.required,
                obtained: trip.obtained + hall.contains,
                distance: trip.distance,
            };

            // Check if we arrived
            if current == end {
                return Some(trip);
            }

            visited.insert(current);
            next.extend(hall.connections.iter().filter_map(|(p, d)| {
                if !visited.contains(p) {
                    Some((*p, trip.added_distance(*d)))
                } else {
                    None
                }
            }));
            next.sort_unstable_by_key(|(_, key)| -(key.distance as isize));
        }
    }

    None
}
