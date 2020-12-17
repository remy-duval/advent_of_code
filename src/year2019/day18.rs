use std::ops::Add;

use crate::commons::grid::Point;
use crate::Problem;

pub struct Day;

impl Problem for Day {
    type Input = String;
    type Err = std::convert::Infallible;
    const TITLE: &'static str = "Day 18: Many-Worlds Interpretation";

    fn solve(data: Self::Input) -> Result<(), Self::Err> {
        // First part
        let (start, keys, map) = parsers::parse_and_optimize_map(&data);
        println!("Map size : {}", map.len());
        let shortest = shortest_path::find_shortest_path(&map, start, keys);
        println!("Shortest path is {} steps long", shortest);

        // Second part
        let split = parsers::split_maze_in_four(&data, start, true);
        let shortest: usize = split
            .iter()
            .map(|data| {
                let (start, keys, map) = parsers::parse_and_optimize_map(data);
                // We need to subtract because for we add the start point on the middle line
                // Which makes the path longer by 1 step for each robot
                shortest_path::find_shortest_path(&map, start, keys) - 1
            })
            .sum();

        println!("Shortest path is {} steps long", shortest);

        Ok(())
    }
}

/// A hallway in the maze
#[derive(Debug, Clone, Default)]
pub struct HallWay {
    char: char,
    required: Keys,
    contains: Keys,
    connections: Vec<(Point, usize)>,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Default, Hash)]
pub struct Keys(u32);

impl Keys {
    const FULL: u32 = (1 << 26) - 1;

    /// Gets if all keys in the right hand side are present in this
    pub fn contains(self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }

    /// Combine the keys of this and the keys of the argument
    pub fn combine(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }

    /// True if this has no keys
    pub fn is_empty(self) -> bool {
        self.0 == 0
    }

    /// True if this contains all the keys from 0 to 25
    pub fn is_full(self) -> bool {
        self.0 == Self::FULL
    }
}

impl Add for Keys {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        self.combine(rhs)
    }
}

/// All the methods for getting the shortest path on a maze
pub mod shortest_path {
    use hashbrown::{HashMap, HashSet};

    use crate::commons::grid::Point;

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
}

/// All the methods for interpreting the map in a suitable way for the shortest path finding
pub mod parsers {
    use std::convert::TryFrom;
    use std::fmt::{Display, Formatter};

    use hashbrown::HashMap;
    use itertools::Itertools;

    use crate::commons::grid::{Direction, Point};

    use super::{HallWay, Keys};

    /// Splits the maze in four at the middle point given.
    pub fn split_maze_in_four(data: &str, middle: Point, add_starts: bool) -> [String; 4] {
        let mut split = [
            String::with_capacity(data.len() / 4),
            String::with_capacity(data.len() / 4),
            String::with_capacity(data.len() / 4),
            String::with_capacity(data.len() / 4),
        ];
        let half_y = middle.y as usize;
        let half_x = middle.x as usize;
        for (y, line) in data.lines().enumerate() {
            match y {
                less if less < half_y => {
                    split[0].push_str(&format!("{}#\n", &line[..half_x]));
                    split[1].push_str(&format!("#{}\n", &line[(half_x + 1)..]));
                }
                equals if equals == half_y => {
                    if add_starts {
                        split[0].push_str(&format!("{}@#\n", &line[..(half_x - 1)]));
                        split[1].push_str(&format!("#@{}\n", &line[(half_x + 2)..]));
                        split[2].push_str(&format!("{}@#\n", &line[..(half_x - 1)]));
                        split[3].push_str(&format!("#@{}\n", &line[(half_x + 2)..]));
                    }
                }
                _ => {
                    split[2].push_str(&format!("{}#\n", &line[..half_x]));
                    split[3].push_str(&format!("#{}\n", &line[(half_x + 1)..]));
                }
            }
        }

        split
    }

    /// Parse a Maze definition and optimizes its layout for shortest path search
    pub fn parse_and_optimize_map(maze: &str) -> (Point, Keys, HashMap<Point, HallWay>) {
        let (raw_map, (max_x, max_y)) = parse_raw_map(maze);

        // The keys we need to collect before the path is complete
        let mut keys = Keys::default();
        raw_map
            .iter()
            .for_each(|(_, tile)| keys = keys + tile.keys_contained());

        // The reverse of that : the keys we need to already start with to have 26 total
        let base_key: Keys = Keys(Keys::FULL ^ keys.0);

        // The starting point of the robots
        let start = *raw_map
            .iter()
            .find(|(_, tile)| tile.char() == '@')
            .expect("The maze should have a starting position !")
            .0;

        // Optimize the map
        let map = enhance_map(&raw_map);
        let map = prune_dead_ends(raw_map, map);
        let (map, original) = fuse_paths(map);

        // Show the parsed map
        println!(
            "\n{}\n",
            Maze {
                map: original,
                dimensions: (max_x, max_y),
            }
        );

        (start, base_key, map)
    }

    /// Parses a the Maze definition as a String into the Map of Point -> Tiles
    fn parse_raw_map(maze: &str) -> (HashMap<Point, Tile>, (usize, usize)) {
        // To keep track of the size of the maze
        let (mut max_x, mut max_y) = (0, 0);

        // The raw map parsed from the string.
        let raw_map: HashMap<Point, Tile> = maze
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                max_y = y;
                max_x = max_x.max(line.len() - 1);
                line.chars().enumerate().filter_map(move |(x, c)| {
                    let tile = Tile::try_from(c).ok()?;
                    Some((Point::new(x as i64, y as i64), tile))
                })
            })
            .collect();

        (raw_map, (max_x + 1, max_y + 1))
    }

    /// Reads the raw map given and produce an enhanced but unoptimized map Point -> Hallway
    fn enhance_map(raw_map: &HashMap<Point, Tile>) -> HashMap<Point, HallWay> {
        raw_map
            .iter()
            .map(|(point, tile)| {
                let connections: Vec<(Point, usize)> = Direction::all()
                    .iter()
                    .filter_map(|dir| {
                        let other = point.moved(*dir);
                        if raw_map.contains_key(&other) {
                            Some((other, 1))
                        } else {
                            None
                        }
                    })
                    .collect();

                (
                    *point,
                    HallWay {
                        char: tile.char(),
                        required: tile.required_keys(),
                        contains: tile.keys_contained(),
                        connections,
                    },
                )
            })
            .collect()
    }

    /// Takes the raw map and its enhanced counterpart and computes a dead-end less map
    fn prune_dead_ends(
        mut raw_map: HashMap<Point, Tile>, // The raw map is used as a temporary buffer
        mut map: HashMap<Point, HallWay>,  // This map is modified in place and then returned
    ) -> HashMap<Point, HallWay> {
        let mut changes: usize = 1;
        while changes != 0 {
            // We keep going as long as we removed at least one dead-end last time
            changes = 0;
            raw_map.retain(|point, _| map.contains_key(point));
            map.retain(|_, hallway| {
                // Clean the dead connections
                hallway
                    .connections
                    .retain(|(point, _)| raw_map.contains_key(point));

                if !hallway.contains.is_empty()
                    || hallway.connections.len() >= 2
                    || hallway.char == '@'
                {
                    true
                } else {
                    changes += 1;
                    false
                }
            });
        }

        map
    }

    /// Fuses all pathways connecting only two points to simplify the map
    /// # Returns
    /// * `first map` The optimized map Point -> Hallway
    /// * `second map` The original map but with all points on fused paths to char ' '
    fn fuse_paths(
        map: HashMap<Point, HallWay>,
    ) -> (HashMap<Point, HallWay>, HashMap<Point, HallWay>) {
        // True for points on the path that are the junction on two other points only
        fn not_significant(hall: &HallWay) -> bool {
            hall.connections.len() == 2
                && hall.required.is_empty()
                && hall.contains.is_empty()
                && hall.char != '@'
        }
        // Updates the connections to change the from point to the to values
        fn update(connections: &mut [(Point, usize)], from: Point, to: (Point, usize)) {
            if let Some(origin) = connections.iter_mut().find(|(p, _)| *p == from) {
                *origin = to;
            }
        }

        let mut result: HashMap<Point, HallWay> = map.clone();
        for (middle, _) in map.iter().filter(|&(_, hall)| not_significant(hall)) {
            if let Some(origin) = result.get_mut(middle) {
                let pathway = &mut origin.connections;
                let (first, first_distance) = pathway[0];
                let (second, second_distance) = pathway[1];
                pathway.clear();
                origin.char = ' ';

                let total_distance = first_distance + second_distance;
                if let Some(start) = result.get_mut(&first) {
                    update(&mut start.connections, *middle, (second, total_distance));
                }
                if let Some(start) = result.get_mut(&second) {
                    update(&mut start.connections, *middle, (first, total_distance));
                }
                if let Some(origin) = result.get_mut(middle) {
                    origin.connections.clear();
                    origin.char = ' ';
                }
            }
        }

        let with_pathways: HashMap<Point, HallWay> = result.clone();
        result = result
            .into_iter()
            .filter(|(_, hall)| !hall.connections.is_empty())
            .collect();

        (result, with_pathways)
    }

    /// The maze itself
    #[derive(Debug, Clone)]
    pub struct Maze {
        map: HashMap<Point, HallWay>,
        dimensions: (usize, usize),
    }

    impl Display for Maze {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            let (max_x, max_y) = self.dimensions;
            let display = (0..max_y)
                .map(|y| {
                    (0..max_x)
                        .map(|x| {
                            let point = Point::new(x as i64, y as i64);
                            match self.map.get(&point) {
                                Some(hallway) => hallway.char,
                                None => '#',
                            }
                        })
                        .join("")
                })
                .join("\n");

            write!(f, "{}", display)
        }
    }

    /// A tile in the maze (not a wall)
    #[derive(Debug, Copy, Clone)]
    enum Tile {
        Empty(char),
        Door(char),
        Key(char),
    }

    impl TryFrom<char> for Tile {
        type Error = ();

        fn try_from(c: char) -> Result<Self, Self::Error> {
            match c {
                '.' | '@' => Ok(Tile::Empty(c)),
                c if c.is_ascii_uppercase() => Ok(Tile::Door(c)),
                c if c.is_ascii_lowercase() => Ok(Tile::Key(c)),
                _ => Err(()),
            }
        }
    }

    impl From<u8> for Keys {
        fn from(byte: u8) -> Self {
            Self(1 << byte)
        }
    }

    impl Tile {
        /// The keys (one in fact) contained in this tile
        fn keys_contained(self) -> Keys {
            match self {
                Tile::Key(key) => (key as u8 - 0x61).into(),
                _ => Keys::default(),
            }
        }

        /// The keys (one in fact) required for this tile
        fn required_keys(self) -> Keys {
            match self {
                Tile::Door(lock) => (lock as u8 - 0x41).into(),
                _ => Keys::default(),
            }
        }

        /// The char describing this Tile.
        fn char(self) -> char {
            match self {
                Tile::Door(lock) => lock,
                Tile::Key(key) => key,
                Tile::Empty(c) => c,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_ONE: &str = include_str!("test_resources/day18_1.txt");
    const TEST_TWO: &str = include_str!("test_resources/day18_2.txt");
    const TEST_THREE: &str = include_str!("test_resources/day18_3.txt");
    const TEST_FOUR: &str = include_str!("test_resources/day18_4.txt");
    const DATA: &str = include_str!("test_resources/day18_data.txt");

    #[test]
    fn shortest_test_one_player() {
        fn assertion(data: &str, expected_steps: usize) {
            let (start, keys, map) = parsers::parse_and_optimize_map(&data);
            let shortest = shortest_path::find_shortest_path(&map, start, keys);

            assert_eq!(expected_steps, shortest)
        }

        assertion(TEST_ONE, 132);
        assertion(TEST_TWO, 136);
        assertion(TEST_THREE, 81);
        assertion(DATA, 5392);
    }

    #[test]
    fn shortest_test_split_a() {
        let split = parsers::split_maze_in_four(&TEST_FOUR, Point::new(7, 3), false);
        let shortest: usize = split
            .iter()
            .map(|data| {
                let (start, keys, map) = parsers::parse_and_optimize_map(data);
                shortest_path::find_shortest_path(&map, start, keys)
            })
            .sum();

        assert_eq!(24, shortest);
    }

    #[test]
    fn shortest_test_split_b() {
        let (start, _, _) = parsers::parse_and_optimize_map(DATA);
        let split = parsers::split_maze_in_four(DATA, start, true);
        let shortest: usize = split
            .iter()
            .map(|data| {
                let (start, keys, map) = parsers::parse_and_optimize_map(data);
                shortest_path::find_shortest_path(&map, start, keys) - 1
            })
            .sum();

        assert_eq!(1684, shortest);
    }
}
