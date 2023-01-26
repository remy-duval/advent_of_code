use std::{
    collections::{HashMap, HashSet},
    fmt::{Display, Error, Formatter},
    io::{stdout, BufWriter, Write},
    str::FromStr,
};

use itertools::Itertools;

use commons::eyre::{eyre, Result};
use commons::grid::{Direction, Point};

use super::int_code::{IntCodeInput, Processor, Status};

pub const TITLE: &str = "Day 17: Set and Forget";

pub fn run(raw: String) -> Result<()> {
    let memory = parse(&raw)?.data;
    let scaffold = Scaffold::from_camera_program(&memory, true)
        .ok_or_else(|| eyre!("The camera program should have worked !"))?;

    // First part
    let calibration = scaffold.intersections_sum();
    println!("The calibration sum is {calibration}");

    // Second part
    let path = scaffold.straight_ahead_path();
    println!("The path is {}", path.iter().join(","));
    let (main, a, b, c) =
        compression(&path, (5, 20)).ok_or_else(|| eyre!("The compression should succeed !"))?;
    println!("We can send it as {main} with \nA = {a}\nB = {b} \nC = {c}");

    // Run the robot with the path
    let mut robot: Processor = {
        let mut robot_mem = memory;
        robot_mem[0] = 2;
        robot_mem[..].into()
    };

    let mut stdout = BufWriter::new(stdout());
    let _ = robot.run_with_ascii_callbacks(
        [&main, &a, &b, &c, "n"].iter(),
        |iterator| Some(format!("{}\n", iterator.next()?)),
        |_, line| {
            stdout
                .write_all(line.as_bytes())
                .map_err(|_| Status::Halted)
        },
    );
    stdout.flush()?;

    println!("The robot finished working, see above for last output.");

    Ok(())
}

fn parse(s: &str) -> Result<IntCodeInput> {
    Ok(s.parse()?)
}

/// Try to compress a full path into a combination of 3 smaller paths as a main path
fn compression(
    full_path: &[Path],
    (min_length, max_length): (usize, usize),
) -> Option<(String, String, String, String)> {
    // Build the main routine from the used patterns and the full pattern map
    fn reconstruct_main(
        used: &[&[Path]], // The slice containing exactly 3 patterns (A, B and C)
        path: &[&[Path]], // The slice containing all patterns use instance in the use order
    ) -> Option<(String, String, String, String)> {
        let mut patterns = used.iter();
        let a = patterns.next()?.iter().join(",");
        let b = patterns.next()?.iter().join(",");
        let c = patterns.next()?.iter().join(",");
        let main_routine = path
            .iter()
            .map(|path| {
                let converted = path.iter().join(",");
                if converted == a {
                    'A'
                } else if converted == b {
                    'B'
                } else {
                    'C'
                }
            })
            .join(",");
        Some((main_routine, a, b, c))
    }

    // The main function here :
    // Try to find a matching 3 elements pattern for the compression algorithm
    fn find_matching_patterns<'a>(
        patterns: &HashMap<usize, Vec<&'a [Path]>>,
        start: usize,
        end: usize,
        used: &[&'a [Path]],
        path: &[&'a [Path]],
    ) -> Option<(String, String, String, String)> {
        if start >= end {
            // If we have reached the end we are done and can rebuild the path
            reconstruct_main(used, path)
        } else if let Some(possible) = patterns.get(&start) {
            // If some length remains we go in turn through each possibility starting at this pos
            possible.iter().find_map(|additional| {
                let length = additional.len();
                if used.contains(additional) {
                    // If the possible pattern is already used, perfect !
                    let mut new_path = path.to_owned();
                    new_path.push(additional);
                    find_matching_patterns(patterns, start + length, end, used, &new_path)
                } else if used.len() < 3 {
                    // If it is not already used but we have not reached 3, we can try it
                    let mut new_used = used.to_owned();
                    new_used.push(additional);
                    let mut new_path = path.to_owned();
                    new_path.push(additional);
                    find_matching_patterns(patterns, start + length, end, &new_used, &new_path)
                } else {
                    // But in the other cases this attempt is a miss
                    None
                }
            })
        } else {
            // This case should never happen as we have computed all patterns from 0 to end - 1
            None
        }
    }

    let main_length = full_path.len();
    // Build the pattern map between all positions and the possible patterns starting there
    let patterns: HashMap<usize, Vec<&[Path]>> = (0..main_length)
        .map(|start| {
            let max_sub_length = (start + max_length / 2).min(main_length) + 1;
            let possibilities: Vec<&[Path]> = ((start + 1)..max_sub_length)
                .rev()
                .filter_map(|end| {
                    let slice = &full_path[start..end];
                    let sub_length = slice.iter().map(|path| path.len()).sum::<usize>() - 1;
                    // Evict possibilities that are not in the wanted bounds
                    if min_length <= sub_length && sub_length <= max_length {
                        Some(slice)
                    } else {
                        None
                    }
                })
                .collect();
            (start, possibilities)
        })
        .collect();

    // Recursively go through each possibility, pruning as early as possible dead-ends
    find_matching_patterns(&patterns, 0, main_length, &Vec::new(), &Vec::new())
}

#[derive(Debug, Eq, PartialEq)]
struct Scaffold {
    path: HashSet<Point>,
    robot: (Point, Direction),
}

impl FromStr for Scaffold {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut path: HashSet<Point> = HashSet::new();
        let mut robot: Option<(Point, Direction)> = None;

        for (y, line) in s.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let point = Point::new(x as i64, y as i64);
                if c == '#' {
                    path.insert(point);
                } else if c == '^' {
                    robot = Some((point, Direction::North));
                    path.insert(point);
                } else if c == 'v' {
                    robot = Some((point, Direction::South));
                    path.insert(point);
                } else if c == '>' {
                    robot = Some((point, Direction::East));
                    path.insert(point);
                } else if c == '<' {
                    robot = Some((point, Direction::West));
                    path.insert(point);
                }
            }
        }

        Ok(Self {
            path,
            robot: robot.ok_or(())?,
        })
    }
}

impl Scaffold {
    /// Read a Scaffold from the camera program given.
    pub fn from_camera_program(memory: &[i64], show: bool) -> Option<Self> {
        let mut processor: Processor = memory.into();
        let mut data = String::new();

        while let (line, None) = processor.read_next_line() {
            data.push_str(&line);
        }

        if show {
            print!("{}", &data);
        }
        data.parse().ok()
    }

    /// The sum of coordinates of all intersections in the path.
    pub fn intersections_sum(&self) -> i64 {
        self.path
            .iter()
            .map(|point| {
                let is_intersection = Direction::all()
                    .iter()
                    .map(|dir| point.moved(*dir))
                    .all(|other| self.path.contains(&other));

                if is_intersection {
                    point.x * point.y
                } else {
                    0
                }
            })
            .sum()
    }

    /// Tries to make the path that goes through every tile by just walking straight ahead
    /// (This specific problem will always have a path like that)
    pub fn straight_ahead_path(&self) -> Vec<Path> {
        let mut visited: HashSet<Point> = HashSet::new();
        let (mut current, mut direction) = self.robot;

        visited.insert(current);
        let mut ahead_number = 0;
        let mut path: Vec<Path> = Vec::with_capacity(80);
        loop {
            // Try going ahead first
            let straight = current.moved(direction);
            if self.path.contains(&straight) {
                current = straight;
                ahead_number += 1;
                visited.insert(current);

                // Check if we arrived at the last position, in that case we insert it and quit
                if visited == self.path {
                    path.push(Path::Ahead(ahead_number));
                    return path;
                }
            } else {
                // We cannot move straight so we have to turn.
                // We then add the number of tiles we moved straight to the path before continuing
                if ahead_number > 0 {
                    path.push(Path::Ahead(ahead_number));
                    ahead_number = 0;
                }

                // Try turning right then left
                let right = current.moved(direction.right());
                let left = current.moved(direction.left());
                if self.path.contains(&right) {
                    path.push(Path::Right);
                    direction = direction.right();
                } else if self.path.contains(&left) {
                    path.push(Path::Left);
                    direction = direction.left();
                }
            }
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Path {
    Ahead(usize),
    Right,
    Left,
}

impl Display for Path {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match *self {
            Path::Ahead(number) => number.fmt(f),
            Path::Right => 'R'.fmt(f),
            Path::Left => 'L'.fmt(f),
        }
    }
}

impl Path {
    /// The length of this path as an ascii value (including the following ,)
    pub fn len(&self) -> usize {
        fn number_of_decimal_digit(int: usize, acc: usize) -> usize {
            if int >= 10 {
                number_of_decimal_digit(int / 10, acc + 1)
            } else {
                acc + 1
            }
        }
        match *self {
            Path::Right | Path::Left => 2,
            Path::Ahead(size) => number_of_decimal_digit(size, 1),
        }
    }
}

#[cfg(test)]
mod tests;
