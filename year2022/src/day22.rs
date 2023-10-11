use std::collections::VecDeque;

use itertools::Itertools;

use commons::error::Result;
use commons::grid::{Direction, Point};
use commons::parse::sep_by_empty_lines;
use commons::{ensure, err, WrapErr};

pub const TITLE: &str = "Day 22: Monkey Map";

type Pos = Point<isize>;

pub fn run(raw: String) -> Result<()> {
    let data = parse(raw.into())?;
    let first = first_part(&data);
    println!("1. The password is {first}");
    let second = second_part(&data)?;
    println!("2. The cube password is {second}");

    Ok(())
}

fn first_part(map: &Map) -> isize {
    map.walk(|pos, dir| map.warp_2d(pos, dir).map(|pos| (pos, dir)))
}

fn second_part(map: &Map) -> Result<isize> {
    let cube = Cube::build(map)?;
    let warp_cube = |pos, dir| match cube.warp(pos, dir) {
        Some((pos, dir)) if matches!(map.get(pos), Tile::Open) => Some((pos, dir)),
        _ => None,
    };
    Ok(map.walk(warp_cube))
}

/// All the directions in the order of the ByDirection array
const DIRECTIONS: [Direction; 4] = [
    Direction::East,
    Direction::South,
    Direction::West,
    Direction::North,
];

#[inline]
fn direction_index(dir: Direction) -> usize {
    match dir {
        Direction::East => 0,
        Direction::South => 1,
        Direction::West => 2,
        Direction::North => 3,
    }
}

/// An array of 4 elements indexed by a direction
#[derive(Debug, Copy, Clone, Eq, PartialEq, Default)]
struct ByDirection<T>([T; 4]);

impl<T> std::ops::Index<Direction> for ByDirection<T> {
    type Output = T;

    #[inline]
    fn index(&self, dir: Direction) -> &Self::Output {
        &self.0[direction_index(dir)]
    }
}

impl<T> std::ops::IndexMut<Direction> for ByDirection<T> {
    #[inline]
    fn index_mut(&mut self, dir: Direction) -> &mut Self::Output {
        &mut self.0[direction_index(dir)]
    }
}

/// An array of 6 elements indexed by a cube side
#[derive(Debug, Copy, Clone, Eq, PartialEq, Default)]
struct BySide<T>([T; 6]);

impl<T> std::ops::Index<Side> for BySide<T> {
    type Output = T;

    #[inline]
    fn index(&self, side: Side) -> &Self::Output {
        &self.0[side.index()]
    }
}

impl<T> std::ops::IndexMut<Side> for BySide<T> {
    #[inline]
    fn index_mut(&mut self, side: Side) -> &mut Self::Output {
        &mut self.0[side.index()]
    }
}

#[derive(Copy, Clone)]
enum PathToken {
    MoveForward(u8),
    TurnLeft,
    TurnRight,
}

#[derive(Copy, Clone)]
enum Tile {
    Open,
    Wall,
    OutsideGrid,
}

struct Map {
    tiles: Vec<Tile>,
    width: isize,
    height: isize,
    path: Vec<PathToken>,
}

impl Map {
    /// Get the tile at the given point (defaulting to the outside tile)
    fn get(&self, Point { x, y }: Pos) -> Tile {
        if x < 0 || y < 0 || x >= self.width {
            Tile::OutsideGrid
        } else if let Some(tile) = self.tiles.get((x + y * self.width) as usize) {
            *tile
        } else {
            Tile::OutsideGrid
        }
    }

    /// Walk from the start position using the given warp strategy
    fn walk(&self, warp: impl Fn(Pos, Direction) -> Option<(Pos, Direction)>) -> isize {
        // The start position is the first position that is an open tile
        let mut pos = Point::new(0, 0);
        while !matches!(self.get(pos), Tile::Open) {
            pos.x += 1;
            if pos.x >= self.width {
                return 0;
            }
        }
        let mut dir = Direction::East;
        self.path.iter().for_each(|path| match *path {
            PathToken::TurnRight => dir = dir.right(),
            PathToken::TurnLeft => dir = dir.left(),
            PathToken::MoveForward(n) => {
                for _ in 0..n {
                    let next = pos.moved(dir);
                    match self.get(next) {
                        Tile::Open => pos = next,
                        Tile::Wall => break,
                        Tile::OutsideGrid => match warp(pos, dir) {
                            Some(next) => (pos, dir) = next,
                            _ => break,
                        },
                    }
                }
            }
        });

        1000 * (pos.y + 1) + 4 * (pos.x + 1) + direction_index(dir) as isize
    }

    /// The warp for the first part of the problem: wrap-around the map
    fn warp_2d(&self, start @ Point { x, y }: Pos, dir: Direction) -> Option<Pos> {
        let (offset, mut pos) = match dir {
            Direction::North => (Point::new(0, -1), Point::new(x, self.height - 1)),
            Direction::South => (Point::new(0, 1), Point::new(x, 0)),
            Direction::East => (Point::new(1, 0), Point::new(0, y)),
            Direction::West => (Point::new(-1, 0), Point::new(self.width - 1, y)),
        };
        loop {
            match self.get(pos) {
                Tile::OutsideGrid if pos != start => pos += offset,
                Tile::Open => return Some(pos),
                _ => return None,
            }
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Side {
    Front,
    Top,
    Bottom,
    Left,
    Right,
    Back,
}

impl Side {
    /// All the sides in the order of the BySide array
    const ALL: [Self; 6] = [
        Self::Front,
        Self::Top,
        Self::Bottom,
        Self::Left,
        Self::Right,
        Self::Back,
    ];
    const fn opposite(self) -> Self {
        match self {
            Self::Front => Self::Back,
            Self::Back => Self::Front,
            Self::Left => Self::Right,
            Self::Right => Self::Left,
            Self::Top => Self::Bottom,
            Self::Bottom => Self::Top,
        }
    }

    const fn index(self) -> usize {
        match self {
            Self::Front => 0,
            Self::Top => 1,
            Self::Bottom => 2,
            Self::Left => 3,
            Self::Right => 4,
            Self::Back => 5,
        }
    }

    const fn next_side_left(self, other: Self) -> Option<Self> {
        match (self, other) {
            (Self::Front, Self::Top) => Some(Self::Left),
            (Self::Front, Self::Left) => Some(Self::Bottom),
            (Self::Front, Self::Bottom) => Some(Self::Right),
            (Self::Front, Self::Right) => Some(Self::Top),
            (Self::Left, Self::Top) => Some(Self::Back),
            (Self::Left, Self::Back) => Some(Self::Bottom),
            (Self::Left, Self::Bottom) => Some(Self::Front),
            (Self::Left, Self::Front) => Some(Self::Top),
            (Self::Right, Self::Top) => Some(Self::Front),
            (Self::Right, Self::Front) => Some(Self::Bottom),
            (Self::Right, Self::Bottom) => Some(Self::Back),
            (Self::Right, Self::Back) => Some(Self::Top),
            (Self::Back, Self::Top) => Some(Self::Right),
            (Self::Back, Self::Right) => Some(Self::Bottom),
            (Self::Back, Self::Bottom) => Some(Self::Left),
            (Self::Back, Self::Left) => Some(Self::Top),
            (Self::Top, Self::Front) => Some(Self::Right),
            (Self::Top, Self::Right) => Some(Self::Back),
            (Self::Top, Self::Back) => Some(Self::Left),
            (Self::Top, Self::Left) => Some(Self::Front),
            (Self::Bottom, Self::Front) => Some(Self::Left),
            (Self::Bottom, Self::Left) => Some(Self::Back),
            (Self::Bottom, Self::Back) => Some(Self::Right),
            (Self::Bottom, Self::Right) => Some(Self::Front),
            _ => None,
        }
    }
}

struct Cube {
    sides: BySide<CubeSide>,
}

impl Cube {
    /// Assemble the cube from the map in a brute-forcy way
    fn build(map: &Map) -> Result<Self> {
        // First find the top-left points of each of the sides of the cube
        let size = commons::math::gcd(map.width, map.height);
        let mut sides: [Pos; 6] = Default::default();
        let mut filled = 0;
        for y in 0..(map.height / size) {
            for x in 0..(map.width / size) {
                let pos = Point::new(x, y).multiply(size);
                if !matches!(map.get(pos), Tile::OutsideGrid) {
                    ensure!(filled < sides.len(), "too many sides for the cube");
                    sides[filled] = pos;
                    filled += 1;
                }
            }
        }

        ensure!(filled == sides.len(), "not enough faces for the cube");
        // Second: chose an arbitrary side to be the front side, then explore the cube with a BFS
        let mut builder: BySide<Option<CubeSide>> = Default::default();
        builder[Side::Front] = Some(CubeSide {
            pos: sides[0],
            size,
            adjacent: ByDirection([Side::Right, Side::Bottom, Side::Left, Side::Top]),
        });
        let mut queue = VecDeque::from([Side::Front]);
        while let Some(side) = queue.pop_front() {
            if let Some(CubeSide { pos, adjacent, .. }) = builder[side] {
                for direction in DIRECTIONS {
                    let target_side = adjacent[direction];
                    let target_pos = match direction {
                        Direction::North => Point::new(pos.x, pos.y - size),
                        Direction::South => Point::new(pos.x, pos.y + size),
                        Direction::East => Point::new(pos.x + size, pos.y),
                        Direction::West => Point::new(pos.x - size, pos.y),
                    };
                    if !sides.contains(&target_pos) || builder[target_side].is_some() {
                        continue;
                    }
                    let next_left = side.next_side_left(target_side).wrap_err_with(|| {
                        format!("no left direction for ({side:?} -> {target_side:?})")
                    })?;

                    let mut target_adjacent = ByDirection([Side::Front; 4]);
                    target_adjacent[direction] = side.opposite();
                    target_adjacent[direction.back()] = side;
                    target_adjacent[direction.left()] = next_left;
                    target_adjacent[direction.right()] = next_left.opposite();
                    builder[target_side] = Some(CubeSide {
                        pos: target_pos,
                        size,
                        adjacent: target_adjacent,
                    });
                    queue.push_back(target_side);
                }
            }
        }

        // Finally check that all sides of the cube have been mapped
        ensure!(
            builder.0.iter().all(|opt| opt.is_some()),
            "could not map all the sides of the cube"
        );
        let sides = BySide(Side::ALL.map(|side| builder[side].take().unwrap()));
        Ok(Self { sides })
    }

    /// Warp from one side of the cube to another
    fn warp(&self, pos: Pos, dir: Direction) -> Option<(Pos, Direction)> {
        // Find which side of the cube the position is on
        let side = Side::ALL
            .into_iter()
            .find(|&s| self.sides[s].contains(pos))?;

        // Get both sides of the warp (and the destination direction)
        let start = &self.sides[side];
        let end = &self.sides[start.adjacent[dir]];
        let other_dir = DIRECTIONS.into_iter().find(|&d| end.adjacent[d] == side)?;

        // Find the two segments of the cube, and mirror the position to end end one
        let start = start.segment(dir);
        let end = end.segment(other_dir);
        let destination = start.mirror_to_other(&end, &pos);
        Some((destination, other_dir.back()))
    }
}

/// The top-left position of a side of the cube in the planar map + its neighbouring sides
struct CubeSide {
    pos: Pos,
    size: isize,
    adjacent: ByDirection<Side>,
}

impl CubeSide {
    fn contains(&self, pos: Pos) -> bool {
        (self.pos.x..(self.pos.x + self.size)).contains(&pos.x)
            && (self.pos.y..(self.pos.y + self.size)).contains(&pos.y)
    }

    fn segment(&self, direction: Direction) -> CubeSegment {
        match direction {
            Direction::North => CubeSegment {
                start: self.pos,
                end: self.pos + Point::new(self.size - 1, 0),
            },
            Direction::South => CubeSegment {
                start: self.pos + Point::new(self.size - 1, self.size - 1),
                end: self.pos + Point::new(0, self.size - 1),
            },
            Direction::East => CubeSegment {
                start: self.pos + Point::new(self.size - 1, 0),
                end: self.pos + Point::new(self.size - 1, self.size - 1),
            },
            Direction::West => CubeSegment {
                start: self.pos + Point::new(0, self.size - 1),
                end: self.pos,
            },
        }
    }
}

struct CubeSegment {
    start: Pos,
    end: Pos,
}

impl CubeSegment {
    fn increment(&self) -> Pos {
        if self.start.x == self.end.x {
            Point::new(0, if self.start.y > self.end.y { 1 } else { -1 })
        } else {
            Point::new(if self.start.x > self.end.x { 1 } else { -1 }, 0)
        }
    }

    fn mirror_to_other(&self, other: &Self, pos: &Pos) -> Pos {
        let diff = Self::distance_between(&self.start, pos) as isize;
        other.end + other.increment().multiply(diff)
    }

    fn distance_between(a: &Pos, b: &Pos) -> usize {
        a.x.abs_diff(b.x) + a.y.abs_diff(b.y)
    }
}

fn parse(s: std::borrow::Cow<'static, str>) -> Result<Map> {
    let (grid, path) = sep_by_empty_lines(s.as_ref())
        .collect_tuple::<(_, _)>()
        .wrap_err("missing the two parts (grid and path)")?;

    let mut height = 0;
    let mut width = 0;
    let rows = grid
        .lines()
        .enumerate()
        .map(|(y, line)| {
            if let Some(line) = line
                .bytes()
                .map(|b| match b {
                    b'.' => Some(Tile::Open),
                    b'#' => Some(Tile::Wall),
                    b' ' => Some(Tile::OutsideGrid),
                    _ => None,
                })
                .collect::<Option<Vec<Tile>>>()
            {
                height += 1;
                width = width.max(line.len() as isize);
                Ok(line)
            } else {
                Err(err!("Bad line #{y}:\n{line}"))
            }
        })
        .collect::<Result<Vec<_>>>()?;

    let tiles = rows
        .into_iter()
        .flat_map(|row| {
            let end_padding = (width - row.len() as isize).unsigned_abs();
            row.into_iter()
                .chain(std::iter::repeat(Tile::OutsideGrid).take(end_padding))
        })
        .collect::<Vec<Tile>>();

    let path = path
        .trim()
        .chars()
        .peekable()
        .batching(|chars| match chars.next()? {
            'L' => Some(Ok(PathToken::TurnLeft)),
            'R' => Some(Ok(PathToken::TurnRight)),
            num if num.is_ascii_digit() => {
                let mut forward = num.to_digit(10).unwrap_or_default() as u8;
                while let Some(num) = chars.next_if(char::is_ascii_digit) {
                    forward = forward * 10 + num.to_digit(10).unwrap_or_default() as u8;
                }
                Some(Ok(PathToken::MoveForward(forward)))
            }
            other => Some(Err(err!("Unknown path token: {other}"))),
        })
        .collect::<Result<Vec<_>>>()?;

    Ok(Map {
        tiles,
        width,
        height,
        path,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../examples/22.txt");
    const MAIN: &str = include_str!("../inputs/22.txt");

    #[test]
    fn first_part_example() {
        let data = parse(EXAMPLE.into()).unwrap();
        assert_eq!(first_part(&data), 6_032);
    }

    #[test]
    fn first_part_main() {
        let data = parse(MAIN.into()).unwrap();
        assert_eq!(first_part(&data), 190_066);
    }

    #[test]
    fn second_part_warps() {
        let cube = Cube::build(&parse(EXAMPLE.into()).unwrap()).unwrap();
        assert_eq!(
            cube.warp(Point::new(11, 5), Direction::East),
            Some((Point::new(14, 8), Direction::South))
        );
        assert_eq!(
            cube.warp(Point::new(10, 11), Direction::South),
            Some((Point::new(1, 7), Direction::North))
        );
        assert_eq!(
            cube.warp(Point::new(6, 4), Direction::North),
            Some((Point::new(8, 2), Direction::East))
        );
    }

    #[test]
    fn second_part_example() {
        let data = parse(EXAMPLE.into()).unwrap();
        assert_eq!(second_part(&data).unwrap(), 5_031);
    }

    #[test]
    fn second_part_main() {
        let data = parse(MAIN.into()).unwrap();
        assert_eq!(second_part(&data).unwrap(), 134_170);
    }
}
