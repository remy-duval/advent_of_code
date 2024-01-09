use std::collections::{HashMap, VecDeque};

use commons::error::Result;
use commons::grid::{Direction, Grid, Point};
use commons::{err, WrapErr};

pub const TITLE: &str = "Day 23: A Long Walk";

pub fn run(raw: String) -> Result<()> {
    let data = parse(raw.into())?;
    let first = first_part(&data);
    println!("1. The longest path will be {first}");
    let second = second_part(data);
    println!("2. The improved longest path will be {second}");

    Ok(())
}

fn first_part(graph: &Graph) -> usize {
    graph.longest_path()
}

fn second_part(mut graph: Graph) -> usize {
    graph
        .nodes
        .iter_mut()
        .flatten()
        .for_each(|p| p.blocked = false);

    graph.longest_path()
}

#[derive(Debug)]
struct Graph {
    nodes: Vec<Vec<Path>>,
    end: usize,
    additional_distance: usize,
}

#[derive(Debug)]
struct Path {
    distance: usize,
    destination: usize,
    blocked: bool,
}

#[derive(Debug, Copy, Clone)]
struct Visited(u64);

impl Visited {
    const START: Visited = Visited(1);

    fn add(self, node: usize) -> Self {
        Self(self.0 | (1 << node))
    }

    fn has(self, node: usize) -> bool {
        (self.0 & (1 << node)) != 0
    }
}

impl Graph {
    fn longest_path(&self) -> usize {
        let mut max = 0;
        let mut stack = vec![(0, 0, Visited::START)];
        while let Some((node, distance, visited)) = stack.pop() {
            if node == self.end {
                max = max.max(distance);
            } else if let Some(paths) = self.nodes.get(node) {
                paths
                    .iter()
                    .filter(|p| !p.blocked && !visited.has(p.destination))
                    .for_each(|p| {
                        stack.push((
                            p.destination,
                            distance + p.distance,
                            visited.add(p.destination),
                        ));
                    })
            }
        }

        max + self.additional_distance
    }

    fn new(grid: Grid<u8>, start: Point<isize>, end: Point<isize>) -> Result<Self> {
        fn bit(direction: Direction) -> u8 {
            match direction {
                Direction::North => 1,
                Direction::South => 2,
                Direction::East => 4,
                Direction::West => 8,
            }
        }

        let mut end_index = None;
        let mut additional_distance = 0;
        let mut nodes = vec![];
        let mut node_positions = HashMap::from([(start, (0, bit(Direction::North)))]);
        let mut next_node = 0;
        let mut queue = VecDeque::from([(start, Direction::North, None, 0, false)]);
        let mut next_directions = Vec::with_capacity(4);
        while let Some((point, from, mut prev, mut distance, mut one_way)) = queue.pop_front() {
            next_directions.extend(Direction::ALL.into_iter().filter_map(|direction| {
                if direction == from {
                    return None;
                }
                let next = point.moved(direction);
                let tile = *grid.get(next.tupled())?;
                if tile == b'#' {
                    return None;
                }
                let slope = tile != b'.';
                let available = match tile {
                    b'.' => true,
                    b'>' => direction == Direction::East,
                    b'<' => direction == Direction::West,
                    b'^' => direction == Direction::North,
                    _ => direction == Direction::South,
                };
                Some((available, slope, direction, next))
            }));

            // This is a crossroad or the end, create a node
            if next_directions.len() > 1 {
                let (node, explored) = node_positions.entry(point).or_insert_with(|| {
                    nodes.push(vec![]);
                    next_node += 1;
                    (next_node - 1, bit(from))
                });
                if let Some(prev) = prev.replace(*node) {
                    nodes[*node].push(Path {
                        distance,
                        destination: prev,
                        blocked: one_way,
                    });
                    nodes[prev].push(Path {
                        distance,
                        destination: *node,
                        blocked: false,
                    });
                } else {
                    additional_distance += distance;
                }
                distance = 0;
                one_way = false;
                next_directions.retain(|(_, _, direction, _)| {
                    let bit = bit(*direction);
                    if *explored & bit != 0 {
                        return false;
                    }
                    *explored |= bit;
                    true
                })
            }
            if point == end {
                additional_distance += distance;
                end_index = prev;
            }

            next_directions
                .drain(..)
                .filter(|(available, _, _, _)| *available)
                .for_each(|(_, slope, dir, next)| {
                    queue.push_back((next, dir.back(), prev, distance + 1, one_way || slope));
                });
        }

        Ok(Self {
            nodes,
            end: end_index.wrap_err("missing end")?,
            additional_distance,
        })
    }
}

fn parse(s: std::borrow::Cow<'static, str>) -> Result<Graph> {
    let width = s.lines().next().wrap_err("empty input")?.len();
    let start = std::cell::OnceCell::new();
    let end = std::cell::Cell::new(None);
    s.lines()
        .enumerate()
        .flat_map(|(y, line)| {
            let start = &start;
            let end = &end;
            line.trim().bytes().enumerate().map(move |(x, c)| match c {
                b'.' | b'#' | b'>' | b'<' | b'^' | b'v' => {
                    if c == b'.' {
                        start.get_or_init(|| Point::new(x as isize, y as isize));
                        end.set(Some(Point::new(x as isize, y as isize)));
                    }
                    Ok(c)
                }
                bad => Err(err!("unknown tile at ({x},{y}) {}", bad as char)),
            })
        })
        .collect::<Result<Vec<u8>>>()
        .and_then(|tiles| {
            let start = start.into_inner().wrap_err("missing start")?;
            let end = end.into_inner().wrap_err("missing end")?;
            Graph::new(Grid::from_vec(width, tiles), start, end)
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../examples/23.txt");
    const MAIN: &str = include_str!("../inputs/23.txt");

    #[test]
    fn first_part_example() {
        let data = parse(EXAMPLE.into()).unwrap();
        assert_eq!(first_part(&data), 94);
    }

    #[test]
    fn first_part_main() {
        let data = parse(MAIN.into()).unwrap();
        assert_eq!(first_part(&data), 2_194);
    }

    #[test]
    fn second_part_example() {
        let data = parse(EXAMPLE.into()).unwrap();
        assert_eq!(second_part(data), 154);
    }

    #[test]
    fn second_part_main() {
        let data = parse(MAIN.into()).unwrap();
        assert_eq!(second_part(data), 6_410);
    }
}
