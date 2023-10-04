use std::cmp::{Ordering, Reverse};
use std::collections::BinaryHeap;

use commons::error::Result;
use commons::grid::Point;
use commons::{ensure, err, WrapErr};

pub const TITLE: &str = "Day 24: Blizzard Basin";

pub fn run(raw: String) -> Result<()> {
    let data = parse(raw.into())?;
    let first = first_part(&data).wrap_err("could not reach the end")?;
    println!("1. The path to the end took {} minutes", first.to_end);
    let second = second_part(&data, first).wrap_err("could not backtrack")?;
    println!("2. The backtracking path took {second} minutes");

    Ok(())
}

fn first_part(map: &Input) -> Option<Step> {
    shortest_path(map, map.start, map.end, Time::default())
}

fn second_part(map: &Input, first: Step) -> Option<u16> {
    let second = shortest_path(map, first.step, map.start, first.t)?;
    let third = shortest_path(map, second.step, first.step, second.t)?;
    Some(first.to_end + second.to_end + third.to_end)
}

fn shortest_path(map: &Input, start: Point<u16>, end: Point<u16>, t: Time) -> Option<Step> {
    let mut steps: BinaryHeap<Reverse<Step>> = BinaryHeap::new();
    let mut seen: SeenStates = SeenStates::new(map);
    steps.push(Reverse(Step {
        to_end: map.distance,
        step: start,
        t,
    }));

    while let Some(Reverse(Step { to_end, step, t })) = steps.pop() {
        if step == end {
            return Some(Step { to_end, step, t });
        }

        let t = t.next(map.width, map.height);
        let mut visit = |x: u16, y: u16| {
            if map.is_tile_available(x, y, &t) && !seen.already_visited(&t, x, y) {
                let to_end = match x
                    .abs_diff(end.x)
                    .cmp(&step.x.abs_diff(end.x))
                    .then_with(|| y.abs_diff(end.y).cmp(&step.y.abs_diff(end.y)))
                {
                    Ordering::Less => to_end,
                    Ordering::Equal => to_end + 1,
                    Ordering::Greater => to_end + 2,
                };
                let t = t.clone();
                let step = Point::new(x, y);
                steps.push(Reverse(Step { to_end, step, t }));
            }
        };

        visit(step.x + 1, step.y);
        visit(step.x, step.y + 1);
        visit(step.x, step.y);
        if step.x > 0 {
            visit(step.x - 1, step.y);
        }
        if step.y > 0 {
            visit(step.x, step.y - 1)
        }
    }

    None
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Blizzard {
    Empty,
    Up,
    Down,
    Left,
    Right,
}

/// Offset to move each blizzard initial position to get their current position
#[derive(Debug, Clone, Default, Ord, PartialOrd, Eq, PartialEq)]
struct Time {
    right: u16,
    left: u16,
    up: u16,
    down: u16,
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
struct Step {
    /// Heuristic for A* shortest path: minimum time to reach the end from this position
    to_end: u16,
    step: Point<u16>,
    t: Time,
}

#[derive(Debug)]
struct Input {
    blizzards: Vec<Blizzard>,
    width: u16,
    height: u16,
    start: Point<u16>,
    end: Point<u16>,
    distance: u16,
}

/// Set of all coordinates and times that were already reached during a search
struct SeenStates {
    set: Vec<bool>,
    width: usize,
    height: usize,
}

impl Time {
    fn next(&self, width: u16, height: u16) -> Self {
        Self {
            right: match self.right {
                0 => width - 1,
                x => x - 1,
            },
            left: match self.left {
                x if x < width - 1 => x + 1,
                _ => 0,
            },
            up: match self.up {
                y if y < height - 1 => y + 1,
                _ => 0,
            },
            down: match self.down {
                0 => height - 1,
                y => y - 1,
            },
        }
    }
}

impl Input {
    fn is_tile_available(&self, x: u16, y: u16, t: &Time) -> bool {
        if y == self.start.y {
            x == self.start.x
        } else if y == self.end.y {
            x == self.end.x
        } else if let Some(y) = y.checked_sub(1) {
            x < self.width
                && y < self.height
                && self.check_blizzard(x, y + t.up, Blizzard::Up)
                && self.check_blizzard(x, y + t.down, Blizzard::Down)
                && self.check_blizzard(x + t.left, y, Blizzard::Left)
                && self.check_blizzard(x + t.right, y, Blizzard::Right)
        } else {
            false
        }
    }

    fn check_blizzard(&self, mut x: u16, mut y: u16, direction: Blizzard) -> bool {
        if x >= self.width {
            x -= self.width;
        }
        if y >= self.height {
            y -= self.height;
        }
        let index = x as usize + y as usize * self.width as usize;
        match self.blizzards.get(index) {
            Some(t) => direction.ne(t),
            None => false,
        }
    }
}

impl SeenStates {
    fn new(input: &Input) -> Self {
        let width = input.width as usize;
        let height = input.height as usize + 2;
        Self {
            set: vec![false; width * width * height * height],
            width,
            height,
        }
    }

    fn already_visited(&mut self, time: &Time, x: u16, y: u16) -> bool {
        let time_index = time.left as usize + time.up as usize * self.width;
        let position_index = x as usize + y as usize * self.width;
        let index = time_index + position_index * self.height * self.height;
        match self.set.get_mut(index) {
            Some(missing @ false) => {
                *missing = true;
                false
            }
            _ => true,
        }
    }
}

fn parse(s: std::borrow::Cow<'static, str>) -> Result<Input> {
    let mut lines = s.lines().map(str::trim);
    let top = lines.next().wrap_err("missing top")?;
    let bottom = lines.next_back().wrap_err("missing bottom")?;
    ensure!(
        top.len() == bottom.len() && top.len() > 2,
        "inconsistent width between top and bottom"
    );
    let width = top.len() - 2;
    let height = s.lines().count() - 2;
    let start_x = top.find('.').wrap_err("no start")? as u16 - 1;
    let end_x = bottom.find('.').wrap_err("no end")? as u16 - 1;
    let mut blizzards = Vec::with_capacity(width * height);
    for line in lines {
        for tile in line.bytes() {
            match tile {
                b'.' => blizzards.push(Blizzard::Empty),
                b'>' => blizzards.push(Blizzard::Right),
                b'<' => blizzards.push(Blizzard::Left),
                b'^' => blizzards.push(Blizzard::Up),
                b'v' => blizzards.push(Blizzard::Down),
                b'#' => (),
                unknown => return Err(err!("unknown tile: {unknown}")),
            }
        }
    }

    Ok(Input {
        blizzards,
        width: width as u16,
        height: height as u16,
        start: Point::new(start_x, 0),
        end: Point::new(end_x, height as u16 + 1),
        distance: end_x.abs_diff(start_x) + height as u16 + 1,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../examples/24.txt");
    const MAIN: &str = include_str!("../inputs/24.txt");

    #[test]
    fn first_part_example() {
        let data = parse(EXAMPLE.into()).unwrap();
        assert_eq!(first_part(&data).unwrap().to_end, 18);
    }

    #[test]
    fn first_part_main() {
        let data = parse(MAIN.into()).unwrap();
        assert_eq!(first_part(&data).unwrap().to_end, 266);
    }

    #[test]
    fn second_part_example() {
        let data = parse(EXAMPLE.into()).unwrap();
        let first = first_part(&data).unwrap();
        assert_eq!(second_part(&data, first), Some(54));
    }

    #[test]
    fn second_part_main() {
        let data = parse(MAIN.into()).unwrap();
        let first = first_part(&data).unwrap();
        assert_eq!(second_part(&data, first), Some(853));
    }
}
