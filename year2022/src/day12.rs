use std::cmp::Reverse;
use std::collections::BinaryHeap;

use commons::error::Result;
use commons::grid::Point;
use commons::{err, WrapErr};

pub const TITLE: &str = "Day 12: Hill Climbing Algorithm";

pub fn run(raw: String) -> Result<()> {
    let data = parse(raw.into())?;
    let first = first_part(&data).wrap_err("unreachable end")?;
    println!("1. The end can be reached from S in {first} steps");
    let second = second_part(&data).wrap_err("unreachable end")?;
    println!("2. The end can be reached from any a in {second} steps");

    Ok(())
}

fn first_part(maze: &Maze) -> Option<u16> {
    dijkstra(maze, vec![maze.start.y * maze.width + maze.start.x])
}

fn second_part(maze: &Maze) -> Option<u16> {
    let starts = maze
        .points
        .iter()
        .enumerate()
        .filter(|(_, e)| **e == 0)
        .map(|(i, _)| i)
        .collect();
    dijkstra(maze, starts)
}

#[derive(Debug)]
struct Maze {
    start: Point<usize>,
    end: Point<usize>,
    width: usize,
    points: Vec<u8>,
}

fn dijkstra(maze: &Maze, starts: Vec<usize>) -> Option<u16> {
    let width = maze.width;
    let total = maze.points.len();
    let mut stack: BinaryHeap<Reverse<(u16, usize)>> = BinaryHeap::new();
    let mut seen: Vec<bool> = vec![false; total];
    for start in starts {
        stack.push(Reverse((0, start)));
        seen[start] = true;
    }

    let end = maze.end.y * width + maze.end.x;
    while let Some(Reverse((distance, point))) = stack.pop() {
        let elevation = maze.points[point];
        let mut check_next = |j: usize| {
            if seen[j] || elevation + 1 < maze.points[j] {
                false
            } else if j == end {
                true
            } else {
                stack.push(Reverse((distance + 1, j)));
                seen[j] = true;
                false
            }
        };

        let x = point % width;
        let found_goal = (x != 0 && check_next(point - 1))
            || (x != width - 1 && check_next(point + 1))
            || (point >= width && check_next(point - width))
            || (point < total - width && check_next(point + width));

        if found_goal {
            return Some(distance + 1);
        }
    }

    None
}

fn parse(s: std::borrow::Cow<'static, str>) -> Result<Maze> {
    let width = s.lines().next().wrap_err("empty input")?.len();
    let mut start = None;
    let mut end = None;
    let mut points = Vec::with_capacity(width);
    for (y, line) in s.lines().enumerate() {
        for (x, c) in line.bytes().enumerate() {
            let value = match c {
                b'a'..=b'z' => c,
                b'S' => {
                    start = Some(Point::new(x, y));
                    b'a'
                }
                b'E' => {
                    end = Some(Point::new(x, y));
                    b'z'
                }
                other => return Err(err!("no alphabetic character: {other}")),
            };

            points.push(value - b'a');
        }
    }

    Ok(Maze {
        start: start.wrap_err("missing start position")?,
        end: end.wrap_err("missing start position")?,
        width,
        points,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../examples/12.txt");
    const MAIN: &str = include_str!("../inputs/12.txt");

    #[test]
    fn first_part_example() {
        let data = parse(EXAMPLE.into()).unwrap();
        assert_eq!(first_part(&data), Some(31));
    }

    #[test]
    fn first_part_main() {
        let data = parse(MAIN.into()).unwrap();
        assert_eq!(first_part(&data), Some(534));
    }

    #[test]
    fn second_part_example() {
        let data = parse(EXAMPLE.into()).unwrap();
        assert_eq!(second_part(&data), Some(29));
    }

    #[test]
    fn second_part_main() {
        let data = parse(MAIN.into()).unwrap();
        assert_eq!(second_part(&data), Some(525));
    }
}
