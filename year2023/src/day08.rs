use itertools::Itertools;

use commons::error::Result;
use commons::math::lcm;
use commons::parse::sep_by_empty_lines;
use commons::{err, Report, WrapErr};

pub const TITLE: &str = "Day 8: Haunted Wasteland";

pub fn run(raw: String) -> Result<()> {
    let data = parse(raw.as_str())?;
    let first = first_part(&data)?;
    println!("1. It takes {first} steps to reach ZZZ from AAA");
    let second = second_part(&data)?;
    println!("2. It takes {second} steps to reach all Z nodes from all A nodes");

    Ok(())
}

#[derive(Debug)]
struct Map {
    path: Vec<Direction>,
    nodes: Vec<Node>,
    intersections: Vec<Option<(Node, Node)>>,
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    Left,
    Right,
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct Node(u16);

impl Map {
    fn get_steps(&self, start: Node, is_end: impl Fn(Node) -> bool) -> Option<usize> {
        self.path
            .iter()
            .cycle()
            .scan(start, |current, dir| {
                match (self.intersections.get(current.index()), dir) {
                    (Some(Some((left, _))), Direction::Left) => *current = *left,
                    (Some(Some((_, right))), Direction::Right) => *current = *right,
                    _ => return None,
                };
                Some(*current)
            })
            .position(is_end)
            .map(|i| i + 1)
    }
}

impl Node {
    const ZZZ: Node = Self::new(b'Z', b'Z', b'Z');

    const fn new(a: u8, b: u8, c: u8) -> Self {
        Self((a - b'A') as u16 + 26 * ((b - b'A') as u16 + 26 * ((c - b'A') as u16)))
    }

    const fn index(self) -> usize {
        self.0 as usize
    }

    const fn is_a_node(self) -> bool {
        self.0 / (26 * 26) == 0
    }

    const fn is_z_node(self) -> bool {
        self.0 / (26 * 26) == 25
    }
}

impl std::str::FromStr for Node {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.bytes()
            .collect_tuple::<(_, _, _)>()
            .wrap_err_with(|| format!("expected 3 characters in {s:?}"))
            .and_then(|elt| match elt {
                (a @ b'A'..=b'Z', b @ b'A'..=b'Z', c @ b'A'..=b'Z') => Ok(Self::new(a, b, c)),
                _ => Err(err!("not all characters were ascii uppercase in {s:?}")),
            })
    }
}

impl std::fmt::Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use std::fmt::Write;
        (0..3)
            .try_fold(self.0, |rest, _| {
                f.write_char((b'A' + (rest % 26) as u8) as char)?;
                Ok(rest / 26)
            })
            .map(|_| ())
    }
}

fn first_part(map: &Map) -> Result<usize> {
    map.get_steps(Node::new(b'A', b'A', b'A'), |node| node == Node::ZZZ)
        .wrap_err("did not find the ZZZ node by starting at the AAA node")
}

fn second_part(map: &Map) -> Result<i64> {
    map.nodes
        .iter()
        .filter(|n| n.is_a_node())
        .try_fold(None, |acc, n| {
            map.get_steps(*n, |n| n.is_z_node())
                .wrap_err_with(|| format!("did not find a Z node by starting at {n:?}"))
                .map(|steps| {
                    let steps = steps as i64;
                    Some(acc.map_or(steps, |prev| lcm(prev, steps)))
                })
        })?
        .wrap_err("no A nodes in the map")
}

fn parse(s: &str) -> Result<Map> {
    let mut sections = sep_by_empty_lines(s);
    sections
        .next()
        .and_then(|a| Some((a, sections.next()?)))
        .wrap_err("could not split input in directions + intersetions")
        .and_then(|(directions, intersections)| {
            let d = directions.trim().chars().map(|c| match c {
                'L' => Ok(Direction::Left),
                'R' => Ok(Direction::Right),
                bad => Err(bad),
            });
            let path: Vec<_> = itertools::process_results(d, |d| d.collect())
                .map_err(|bad| err!("'{bad}' is not a L or R in {directions:?}"))?;

            let mut intersections_by_start = vec![None; Node::ZZZ.index() + 1];
            let nodes = intersections.lines().map(|line| {
                line.split_once('=')
                    .and_then(|(from, to)| Some((from, to.split_once(',')?)))
                    .and_then(|(from, (left, right))| {
                        let from = from.trim().parse::<Node>();
                        let left = left.trim().strip_prefix('(')?.parse::<Node>();
                        let right = right.trim().strip_suffix(')')?.parse::<Node>();
                        Some((from, left, right))
                    })
                    .wrap_err("could not split intersection in from = (left, right)")
                    .and_then(|(from, left, right)| {
                        let from = from?;
                        let left = left?;
                        let right = right?;
                        intersections_by_start[from.0 as usize] = Some((left, right));
                        Ok(from)
                    })
                    .wrap_err_with(|| format!("for intersection {line:?}"))
            });

            Ok(Map {
                path,
                nodes: nodes.collect::<Result<Vec<Node>>>()?,
                intersections: intersections_by_start,
            })
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = include_str!("../examples/08_1.txt");
    const EXAMPLE_2: &str = include_str!("../examples/08_2.txt");
    const EXAMPLE_3: &str = include_str!("../examples/08_3.txt");
    const MAIN: &str = include_str!("../inputs/08.txt");

    #[test]
    fn nodes() {
        assert!(Node::new(b'A', b'A', b'A').is_a_node());
        assert!(!Node::new(b'A', b'A', b'A').is_z_node());
        assert!(!Node::ZZZ.is_a_node());
        assert!(Node::ZZZ.is_z_node());
        assert!(Node::new(b'B', b'B', b'A').is_a_node());
        assert!(!Node::new(b'B', b'B', b'A').is_z_node());
        assert!(!Node::new(b'J', b'J', b'Z').is_a_node());
        assert!(Node::new(b'J', b'J', b'Z').is_z_node());
    }

    #[test]
    fn first_part_example_1() {
        let data = parse(EXAMPLE_1).unwrap();
        assert_eq!(first_part(&data).unwrap(), 2);
    }

    #[test]
    fn first_part_example_2() {
        let data = parse(EXAMPLE_2).unwrap();
        assert_eq!(first_part(&data).unwrap(), 6);
    }

    #[test]
    fn first_part_main() {
        let data = parse(MAIN).unwrap();
        assert_eq!(first_part(&data).unwrap(), 12_643);
    }

    #[test]
    fn second_part_example_3() {
        let data = parse(EXAMPLE_3).unwrap();
        assert_eq!(second_part(&data).unwrap(), 6);
    }

    #[test]
    fn second_part_main() {
        let data = parse(MAIN).unwrap();
        assert_eq!(second_part(&data).unwrap(), 13_133_452_426_987);
    }
}
