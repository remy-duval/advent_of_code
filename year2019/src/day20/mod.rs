use std::collections::VecDeque;

use hashbrown::{HashMap, HashSet};

use commons::eyre::{eyre, Result};
use commons::grid::{Direction, Point};

pub const TITLE: &str = "Day 20: Donut Maze";

pub fn run(raw: String) -> Result<()> {
    let data = Maze::parse(&raw);
    println!(
        "The distance from AA to ZZ without recursion is {}",
        first_part(&data)?
    );

    println!(
        "The distance from AA to ZZ with recursion is {}",
        second_part(&data)?
    );

    Ok(())
}

fn first_part(maze: &Maze) -> Result<usize> {
    maze.bfs("AA", "ZZ", true)
        .ok_or_else(|| eyre!("Breadth first search error for traversal without recursion"))
}

fn second_part(maze: &Maze) -> Result<usize> {
    maze.bfs("AA", "ZZ", false)
        .ok_or_else(|| eyre!("Breadth first search error for traversal with recursion"))
}

/// Represent the maze to traverse in this problem.
#[derive(Debug, Clone)]
struct Maze {
    graph: HashMap<Point, Vec<Transition>>,
    portals: HashMap<String, Portal>,
}

impl Maze {
    /// A breadth first search from `start` to `end` in the maze.
    /// # Arguments
    /// * `ignore_recursion` If this boolean is set the exploration will ignore the
    /// maze recursive properties (portals will lead to same level)
    fn bfs(&self, start: &str, end: &str, ignore_recursion: bool) -> Option<usize> {
        // We always start on an outer portal.
        let start = *self.portals.get(start)?.outer()?;
        let end = *self.portals.get(end)?.outer()?;

        let mut visited: HashSet<(Point, u8)> = HashSet::new();
        let mut next: VecDeque<(Point, usize, u8)> = VecDeque::from(vec![(start, 0, 0)]);

        while let Some((visit, steps, recursion)) = next.pop_front() {
            // To reach the end we need to not be in a recursive part of the maze
            if recursion == 0 && visit == end {
                return Some(steps);
            } else if !visited.contains(&(visit, recursion)) {
                let neighbors = self.graph.get(&visit)?;
                next.extend(neighbors.iter().filter_map(|transition| {
                    if ignore_recursion {
                        Some((transition.destination, steps + 1, 0))
                    } else if transition.is_traversable(recursion) {
                        Some((
                            transition.destination,
                            steps + 1,
                            transition.change_recursion(recursion),
                        ))
                    } else {
                        None
                    }
                }));
                visited.insert((visit, recursion));
            }
        }

        None
    }

    /// Parse the maze from a String slice.
    fn parse(from: &str) -> Self {
        let mut dimensions: (i64, i64) = (0, 0);
        // Convert the maze to a Vec of Vec of chars for parsing easily the portal tags
        let vectored: Vec<Vec<char>> = from
            .lines()
            .map(|line| {
                let vec_line: Vec<char> = line.chars().collect();
                dimensions.0 = dimensions.0.max(vec_line.len() as i64);
                dimensions.1 += 1;
                vec_line
            })
            .collect();

        let mut portals: HashMap<String, Portal> = HashMap::new();
        let mut portal_points: HashMap<Point, String> = HashMap::new();
        let mut points: HashSet<Point> = HashSet::new();
        for (y, line) in vectored.iter().enumerate() {
            for (x, c) in line.iter().enumerate() {
                if *c == '.' {
                    points.insert(Point::new(x as i64, y as i64));
                } else if c.is_ascii_uppercase() {
                    if let Some((name, point)) = Self::parse_portal_label(&vectored[..], (x, y)) {
                        portal_points.insert(point, name.clone());

                        portals
                            .entry(name)
                            .and_modify(|current| {
                                if let Some(combined) =
                                    current.clone() + Portal::new(point, dimensions)
                                {
                                    *current = combined;
                                }
                            })
                            .or_insert_with(|| Portal::new(point, dimensions));
                    }
                }
            }
        }

        Self {
            graph: Self::graph(points, portal_points, &portals),
            portals,
        }
    }

    /// Build the maze graph by taking each point and computing its neighbors (including the portal ones)
    fn graph(
        mut points: HashSet<Point>,            // The Points in the maze
        portal_points: HashMap<Point, String>, // Points which have a portal on them
        portals: &HashMap<String, Portal>,     // The full definition of portals
    ) -> HashMap<Point, Vec<Transition>> {
        let mut graph: HashMap<Point, Vec<Transition>> = points
            .iter()
            .map(|start| {
                // Find neighbors by looking in each direction for points in the maze
                let mut neighbors: Vec<Transition> = Direction::all()
                    .iter()
                    .filter_map(|dir| {
                        let end = start.moved(*dir);
                        if points.contains(&end) {
                            Some(Transition::from_hallway(end))
                        } else {
                            None
                        }
                    })
                    .collect();
                // Find if the point is also connected to a portal to add that to the neighbors
                let portal_transition = portal_points
                    .get(start)
                    .and_then(|name| portals.get(name))
                    .and_then(|portal| portal.transition_from(start));

                if let Some(exit) = portal_transition {
                    neighbors.push(exit);
                }

                (*start, neighbors)
            })
            .collect();

        // Simplify the graph by removing dead-ends (just like Day 18)
        // This speeds-up the second part significantly as most path become straight
        let mut changes: u32 = 1;
        while changes != 0 {
            changes = 0;
            graph.retain(|point, neighbors| {
                neighbors.retain(|transition| points.contains(&transition.destination));
                if neighbors.len() >= 2 || portal_points.contains_key(point) {
                    true
                } else {
                    changes += 1;
                    false
                }
            });
            points.retain(|point| graph.contains_key(point))
        }

        graph
    }

    /// Try to parse a portal label and point from the maze at a position where we found part of it
    fn parse_portal_label(maze: &[Vec<char>], (x, y): (usize, usize)) -> Option<(String, Point)> {
        // Read an element from the maze safely, returning None if nothing is found
        fn get(maze: &[Vec<char>], (x, y): (usize, usize)) -> Option<((usize, usize), char)> {
            let sub = maze.get(y)?;
            let c = *sub.get(x)?;
            Some(((x, y), c))
        }

        // Characters directly neighboring this position
        // We should find both '.' the portal and the remaining character of the label there
        let neighbors: [Option<((usize, usize), char)>; 4] = [
            if x != 0 { get(maze, (x - 1, y)) } else { None },
            if y != 0 { get(maze, (x, y - 1)) } else { None },
            get(maze, (x + 1, y)),
            get(maze, (x, y + 1)),
        ];
        // The character we found at the start
        let second: char = maze[y][x];
        // The second character around this one to complete the label
        let first: char = neighbors.iter().find_map(|&c| match c {
            Some((_, c)) if c.is_ascii_uppercase() => Some(c),
            _ => None,
        })?;
        // The '.' point neighboring the label, this is where the portal is situated
        let door: Point = neighbors.iter().find_map(|&c| match c {
            Some((pos, '.')) => Some(Point::new(pos.0 as i64, pos.1 as i64)),
            _ => None,
        })?;

        // The name is formed from first and second char, but the order depends on the orientation
        // If y difference is 1 we need to reverse the order of the name (it is upside down)
        let orientation = door - Point::new(x as i64, y as i64);
        let name = match (orientation.x, orientation.y) {
            (-1, 0) | (0, -1) => format!("{}{}", second, first),
            _ => format!("{}{}", first, second),
        };

        Some((name, door))
    }
}

/// Represent a transition to a neighbor in the maze.
#[derive(Debug, Clone)]
struct Transition {
    recursion: Recursion,
    destination: Point,
}

impl Transition {
    /// Build a new Transition to the given point using a portal.
    pub fn from_portal(destination: Point, from_inner: bool) -> Self {
        Self {
            recursion: if from_inner {
                Recursion::Recurse
            } else {
                Recursion::Unwind
            },
            destination,
        }
    }

    /// Build a new Transition to the given point.
    pub fn from_hallway(destination: Point) -> Self {
        Self {
            recursion: Recursion::Stay,
            destination,
        }
    }

    /// Return the new recursion level after the transition
    fn change_recursion(&self, level: u8) -> u8 {
        match self.recursion {
            Recursion::Recurse => level + 1,
            Recursion::Unwind => level - 1,
            Recursion::Stay => level,
        }
    }

    /// True if this transition is usable
    fn is_traversable(&self, level: u8) -> bool {
        match self.recursion {
            Recursion::Unwind => level != 0,
            _ => true,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Recursion {
    Recurse,
    Unwind,
    Stay,
}

/// Represent a portal in the maze
#[derive(Debug, Clone)]
enum Portal {
    Outer(Point),
    Inner(Point),
    Recursive { outer: Point, inner: Point },
}

impl Portal {
    /// Build a new single point portal.
    fn new(point: Point, dimensions: (i64, i64)) -> Self {
        let is_outer = point.x <= 2
            || point.x >= dimensions.0 - 3
            || point.y <= 2
            || point.y >= dimensions.1 - 3;

        if is_outer {
            Portal::Outer(point)
        } else {
            Portal::Inner(point)
        }
    }

    fn connected(inner: Point, outer: Point) -> Self {
        Portal::Recursive { inner, outer }
    }

    /// Combine this portal with another to connect them.
    fn combine(self, rhs: Self) -> Option<Self> {
        match (self, rhs) {
            (Self::Outer(outer), Self::Inner(inner)) => Some(Self::connected(inner, outer)),
            (Self::Inner(inner), Self::Outer(outer)) => Some(Self::connected(inner, outer)),
            _ => None,
        }
    }

    /// The outer point of a portal
    fn outer(&self) -> Option<&Point> {
        match self {
            Self::Outer(point) => Some(point),
            Self::Recursive { outer: point, .. } => Some(point),
            _ => None,
        }
    }

    /// The transition using this portal from the given entry.
    fn transition_from(&self, entry: &Point) -> Option<Transition> {
        match self {
            Self::Recursive { inner, outer } => {
                if *entry == *outer {
                    Some(Transition::from_portal(*inner, false))
                } else if *entry == *inner {
                    Some(Transition::from_portal(*outer, true))
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

impl std::ops::Add for Portal {
    type Output = Option<Self>;
    fn add(self, rhs: Self) -> Self::Output {
        self.combine(rhs)
    }
}

#[cfg(test)]
mod tests;
