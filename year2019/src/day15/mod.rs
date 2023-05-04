use std::collections::VecDeque;

use itertools::Itertools;
use std::collections::{HashMap, HashSet};

use commons::grid::{Direction, Point};
use commons::{err, Result};
use commons::{WrapErr, TO_TOP};

use super::int_code::{IntCodeInput, Processor, Status};

pub const TITLE: &str = "Day 15: Oxygen System";
const FRAME_DELAY: u64 = 0;

pub fn run(raw: String) -> Result<()> {
    let memory = parse(&raw)?.data;
    let map = explore_map(&memory, true);

    // First part
    let (oxygen, path_length) = first_part(&map)?;
    println!("The shortest path to the oxygen {oxygen} takes {path_length} steps");

    // Second part
    let path_length = second_part(&map, oxygen)?;
    println!("The longest path from the oxygen is {path_length} steps long");

    Ok(())
}

fn parse(s: &str) -> Result<IntCodeInput> {
    Ok(s.parse()?)
}

fn first_part(map: &HashMap<Point, Tile>) -> Result<(Point, usize)> {
    let path = Vec::from(
        bfs(Point::default(), map, |p, _| {
            matches!(map.get(&p), Some(Tile::OxygenSystem))
        })
        .wrap_err("Breadth first search failed for path to the oxygen")?,
    );
    let oxygen = Direction::compute_movement(Point::default(), &path);

    Ok((oxygen, path.len()))
}

fn second_part(map: &HashMap<Point, Tile>, oxygen: Point) -> Result<usize> {
    let walkable_tiles = map.iter().filter(|(_, tile)| **tile != Tile::Wall).count();
    let path = bfs(oxygen, map, |_, visited| visited.len() >= walkable_tiles).ok_or_else(|| {
        err!("Breadth first search failed for the longest path to fill with oxyge")
    })?;

    Ok(path.len())
}

/// The robot explores the maze until it finds no unexplored tiles adjacent to explored ones
fn explore_map(memory: &[i64], show: bool) -> HashMap<Point, Tile> {
    fn convert_direction(direction: Direction) -> i64 {
        match direction {
            Direction::North => 1,
            Direction::South => 2,
            Direction::West => 3,
            Direction::East => 4,
        }
    }

    let mut robot: Processor = memory.into();
    let mut map: HashMap<Point, Tile> = HashMap::new();
    let ((mut min_x, mut min_y), (mut max_x, mut max_y)) = ((0, 0), (0, 0));

    let mut current = Point::default();
    let mut direction = Direction::North;
    let mut next: VecDeque<Direction> = VecDeque::new();
    robot.write_int(convert_direction(direction));
    map.insert(current, Tile::Empty);
    loop {
        match robot.run() {
            Ok(Status::WithOutput(code)) => {
                let explored = current.moved(direction);
                let tile: Tile = code.into();
                map.insert(explored, tile);

                if tile == Tile::Wall {
                    // We hit a wall, we should not continue our current path further
                    next.clear();
                } else {
                    // We did not hit a wall, we assume we moved to the explored tile
                    current = explored;
                }

                // This code snippet is used to display the full map during the exploration.
                if show {
                    min_x = min_x.min(explored.x);
                    min_y = min_y.min(explored.y);
                    max_x = max_x.max(explored.x);
                    max_y = max_y.max(explored.y);
                    print_map(current, &map, (min_x, min_y), (max_x, max_y));
                    std::thread::sleep(std::time::Duration::from_millis(FRAME_DELAY));
                }
            }
            Ok(Status::RequireInput) => {
                if let Some(next_direction) = next.pop_front() {
                    direction = next_direction;
                    robot.write_int(convert_direction(direction));
                } else {
                    // Use a BFS to look for the nearest not explored point
                    match bfs(current, &map, |p, _| !map.contains_key(&p)) {
                        Some(path) => next = path,
                        None => return map,
                    }
                }
            }
            _ => unreachable!(),
        }
    }
}

/// A function to perform a breadth-first search on a maze with a given terminating condition
/// # Arguments
/// * `start` The point from which we start the BFS
/// * `map` The known map of the maze (Point -> Tile)
/// * `done` The function that examines the current points to say if found the path we seek
/// # Returns
/// Option of the first path that satisfies the `done` function (None if we reach the end before)
fn bfs<Done>(start: Point, map: &HashMap<Point, Tile>, done: Done) -> Option<VecDeque<Direction>>
where
    Done: Fn(Point, &HashSet<Point>) -> bool,
{
    let mut queue: VecDeque<(VecDeque<Direction>, Point)> = VecDeque::new();
    let mut visited = HashSet::new();
    queue.push_back((VecDeque::new(), start));
    visited.insert(start);

    let directions = Direction::all();
    while let Some((path, current)) = queue.pop_front() {
        visited.insert(current);
        if done(current, &visited) {
            return Some(path);
        }

        directions
            .iter()
            .filter(|dir| {
                let moved = current.moved(**dir);
                if let Some(&tile) = map.get(&moved) {
                    tile != Tile::Wall && !visited.contains(&moved)
                } else {
                    true
                }
            })
            .for_each(|dir| {
                let mut new_path = path.clone();
                new_path.push_back(*dir);
                queue.push_back((new_path, current.moved(*dir)));
            })
    }

    None
}

/// Prints the map to the console.
fn print_map(current: Point, map: &HashMap<Point, Tile>, min: (i64, i64), max: (i64, i64)) {
    let to_x = max.0 + 1;
    let to_y = max.1 + 1;

    let display = (min.0..to_y)
        .map(|y| {
            (min.1..to_x)
                .map(|x| {
                    let point = Point::new(x, y);
                    if point == current {
                        '@'
                    } else {
                        match map.get(&point) {
                            Some(&tile) => tile.char(),
                            None => ' ',
                        }
                    }
                })
                .join("")
        })
        .join("\n");

    println!("{TO_TOP}{display}");
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Tile {
    Empty,
    Wall,
    OxygenSystem,
}

impl From<i64> for Tile {
    fn from(int: i64) -> Self {
        match int {
            0 => Tile::Wall,
            1 => Tile::Empty,
            2 => Tile::OxygenSystem,
            _ => unreachable!("The robot should never output anything but 0, 1 or 2"),
        }
    }
}

impl Tile {
    pub fn char(self) -> char {
        match self {
            Tile::Empty => '.',
            Tile::Wall => '#',
            Tile::OxygenSystem => 'O',
        }
    }
}

#[cfg(test)]
mod tests;
