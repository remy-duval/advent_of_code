//! This solution is really really messy
//!
//! It took me the entire afternoon to complete the problem though, so that will have to do

use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str::FromStr;

use hashbrown::HashMap;
use itertools::Itertools;

use crate::parse::sep_by_empty_lines;
use crate::Problem;

const IMAGE_WIDTH: usize = 12;

const SEA_MONSTER: [&'static str; 3] = [
    "                  # ",
    "#    ##    ##    ###",
    " #  #  #  #  #  #   ",
];

const SEA_MONSTER_LEN: usize = SEA_MONSTER[0].len();

pub struct Day;

impl Problem for Day {
    type Input = JigsawImage;
    type Err = anyhow::Error;
    const TITLE: &'static str = "Day 20: Jurassic Jigsaw";

    fn solve(data: Self::Input) -> Result<(), Self::Err> {
        let tiles = data.tiles;
        let image = match_tiles(tiles, IMAGE_WIDTH)
            .ok_or_else(|| anyhow::anyhow!("Could not build the image"))?;

        println!(
            "The corners ID product is {}",
            first_part(&image, IMAGE_WIDTH)
                .ok_or_else(|| anyhow::anyhow!("Could not find the corners of the image"))?
        );

        println!(
            "The water roughness (every square that is not a monster) is {}",
            second_part(FullImage::assemble(image, IMAGE_WIDTH))
        );
        Ok(())
    }
}

/// From an ordered array of tiles, get the four corners and multiply their ID
fn first_part(image: &[Tile], width: usize) -> Option<usize> {
    let a = image.get(0)?.id as usize;
    let b = image.get(width - 1)?.id as usize;
    let c = image.get(width * (width - 1))?.id as usize;
    let d = image.get(width * width - 1)?.id as usize;

    Some(a * b * c * d)
}

/// From an assembled image, try to match a sea monster everywhere it is possible to do to
/// compute the amount of squares that are not part of one
///
/// There is only one version of the image that will have some, so we have to flip and rotate it
/// until we find one that has at least 1 match
fn second_part(image: FullImage) -> usize {
    let matcher = match_sea_monster(SEA_MONSTER);

    // Every possible permutation of the image should be represented here
    let images = [
        image.clone(),
        image.clone().flipped(),
        image.rotated_right(),
        image.rotated_right().flipped(),
        image.rotated_right().rotated_right(),
        image.rotated_right().rotated_right().flipped(),
        image.rotated_right().rotated_right().rotated_right(),
        image
            .rotated_right()
            .rotated_right()
            .rotated_right()
            .flipped(),
    ];

    // The amount of '#' in the image
    let square_count: usize = image
        .data
        .iter()
        .map(|line| line.iter().filter(|c| **c == '#').count())
        .sum();

    // The amount of '#' per sea monster
    let see_monster_parts: usize = SEA_MONSTER
        .iter()
        .map(|line| line.chars().filter(|c| *c == '#').count())
        .sum();

    images
        .iter()
        .find_map(|image| {
            let mut seen: usize = 0;
            for j in 0..(image.width - 3) {
                for i in 0..(image.width - SEA_MONSTER_LEN) {
                    let area = [
                        &image.data[j][i..(i + SEA_MONSTER_LEN)],
                        &image.data[j + 1][i..(i + SEA_MONSTER_LEN)],
                        &image.data[j + 2][i..(i + SEA_MONSTER_LEN)],
                    ];
                    if matcher(area) {
                        seen += 1;
                    }
                }
            }

            // If there was at least one match we found the correct image for our result
            if seen != 0 {
                Some(square_count - seen * see_monster_parts)
            } else {
                None
            }
        })
        .unwrap_or_default()
}

/// Create a mapping of tile to their neighbour, then assemble an ordered array of them
///
/// Such that the array represents the full image
fn match_tiles(tiles: Vec<Tile>, width: usize) -> Option<Vec<Tile>> {
    let neighbours = find_neighbours(&tiles, width)?;

    let mut image: Vec<Tile> = Vec::with_capacity(width * width);
    for j in 0..width {
        let start = if j == 0 {
            // At j = 0 Initialize the array with the top left corner of our mappings
            neighbours
                .iter()
                .find_map(|(key, value)| {
                    if value[0].is_none()
                        && value[2].is_none()
                        && value[1].is_some()
                        && value[3].is_some()
                    {
                        Some(key) // This should be the top left corner
                    } else {
                        None
                    }
                })?
                .clone()
        } else {
            // Else find the bottom neighbour of the last row first tile
            let neighbours = &neighbours[&image[(j - 1) * width]];
            neighbours[1].as_ref()?.clone()
        };
        image.push(start);

        // Then for each tile of the row, get the right neighbour of the previous tile
        for i in 1..width {
            let neighbours = &neighbours[&image[j * width + i - 1]];
            image.push(neighbours[3].as_ref()?.clone());
        }
    }

    Some(image)
}

fn match_sea_monster(array: [&str; 3]) -> impl Fn([&[char]; 3]) -> bool {
    let indices = |str: &str| {
        str.chars()
            .enumerate()
            .filter_map(|(idx, char)| if char == '#' { Some(idx) } else { None })
            .collect_vec()
    };

    let check = |str: &[char], wanted: &[usize]| {
        wanted.iter().all(|idx| str.get(*idx).copied() == Some('#'))
    };

    let all = [indices(array[0]), indices(array[1]), indices(array[2])];

    move |to_match: [&[char]; 3]| {
        to_match
            .iter()
            .zip(&all)
            .all(|(str, wanted)| check(*str, wanted))
    }
}

/// Compute the mapping of each tile to its neighbours
///
/// ### Arguments
/// * `tiles` - a slice that contains all possible permutations of each tile
/// * `width` - The width of the image to assemble as a hint for the mapping capacity
///
/// ### Returns
/// A Map from Tile to an array of its four neighbours (with the correct orientation)
///
/// In the process all the permutations of each tile will be reduced to 1
fn find_neighbours(tiles: &[Tile], width: usize) -> Option<HashMap<Tile, [Option<Tile>; 4]>> {
    let mut mappings = HashMap::with_capacity(width * width);
    let mut stack = Vec::with_capacity(width * width);
    stack.push(tiles.get(0).cloned()?);

    while let Some(tile) = stack.pop() {
        let filtered = |other: &&Tile| other.id != tile.id;
        let mut neighbour = |from: usize, to: usize| {
            let wanted = tile.borders[from];
            let found = tiles
                .iter()
                .filter(filtered)
                .find(|o| o.borders[to] == wanted)
                .cloned()?;

            if !mappings.contains_key(&found) {
                stack.push(found.clone());
            }
            Some(found)
        };

        let neighbours = [
            neighbour(0, 1),
            neighbour(1, 0),
            neighbour(2, 3),
            neighbour(3, 2),
        ];
        mappings.insert(tile, neighbours);
    }

    Some(mappings)
}

/// The assembled Image, should be just a String but Vec<Vec<char>> is easier to manipulate
#[derive(Debug, Clone, Eq, PartialEq)]
struct FullImage {
    data: Vec<Vec<char>>,
    width: usize,
}

impl FullImage {
    /// Build the image from the ordered tiles and the row width
    fn assemble(ordered_tiles: Vec<Tile>, width: usize) -> Self {
        let mut result = Vec::with_capacity(width * 8);

        assert_eq!(ordered_tiles.len(), width * width);
        for y in 0..width {
            for data_part in 0..8 {
                let mut row = Vec::with_capacity(width * 8);
                for x in 0..width {
                    let line = ordered_tiles[y * width + x].data[data_part];
                    for pixel in 0..8 {
                        row.push(line.char_at(pixel));
                    }
                }
                result.push(row);
            }
        }

        Self {
            data: result,
            width: width * 8,
        }
    }

    /// This image, but flipped
    pub fn flipped(mut self) -> Self {
        self.data.iter_mut().for_each(|line| line.reverse());
        self
    }

    /// This image, but rotated right 90 degree
    pub fn rotated_right(&self) -> Self {
        let mut copy = self.data.clone();
        for y in 0..self.width {
            for x in 0..self.width {
                copy[y][x] = self.data[x][self.width - y - 1];
            }
        }

        Self {
            data: copy,
            width: self.width,
        }
    }
}

impl Display for FullImage {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        use std::fmt::Write;

        for line in &self.data {
            for char in line {
                f.write_char(*char)?;
            }
            f.write_char('\n')?;
        }

        Ok(())
    }
}

/// This type exists only to parse the input String, with a FromStr implementation
pub struct JigsawImage {
    tiles: Vec<Tile>,
}

/// An error during the parsing of a tile
#[derive(Debug, thiserror::Error)]
pub enum ImageParseError {
    #[error("Did not find the ID field of a tile in:\n{0}")]
    NoIdFound(Box<str>),
    #[error("Could not parse the tile ID ({1}) from:\n{0}")]
    IdParseError(Box<str>, #[source] std::num::ParseIntError),
    #[error("Too many elements for a tile (expected 10 * 10, got {0:?}) in a tile line:\n{1}")]
    WrongDimensions((usize, usize), Box<str>),
}

impl FromStr for JigsawImage {
    type Err = ImageParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let results: Vec<_> = sep_by_empty_lines(s)
            .map(|blk| {
                let mut lines = blk.lines();
                let id = lines
                    .next()
                    .and_then(|line| line.trim().strip_prefix("Tile "))
                    .and_then(|line| line.strip_suffix(":"))
                    .ok_or_else(|| ImageParseError::NoIdFound(s.into()))?;

                let id: u16 = id
                    .parse()
                    .map_err(|e| ImageParseError::IdParseError(s.into(), e))?;
                let mut data = [[false; 10]; 10];
                for (y, line) in lines.enumerate() {
                    for (x, char) in line.chars().enumerate() {
                        match data.get_mut(y).and_then(|row| row.get_mut(x)) {
                            Some(current) => *current = char == '#',
                            None => return Err(ImageParseError::WrongDimensions((x, y), s.into())),
                        };
                    }
                }

                Ok(Tile::all_possibilities(id, data))
            })
            .try_collect()?;

        let mut tiles = Vec::with_capacity(results.len() * 8);
        for result in results {
            tiles.extend_from_slice(&result);
        }
        Ok(Self { tiles })
    }
}

/// A Tile of the image, the content is compressed as much as possible since it will be cloned a lot
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Tile {
    /// The Image ID, there might be multiple other with the same ID that are the permutations of it
    id: u16,
    /// The 'content' of the image => the 8 * 8 characters in the middle
    data: [Row; 8],
    /// The border of the image => the characters used for the matching but not part of the image
    /// * `0` - Top border
    /// * `1` - Bottom border
    /// * `2` - Left border
    /// * `3` - Right border
    borders: [Border; 4],
}

impl Tile {
    /// Produce a new tile from an ID and some data
    ///
    /// ### Arguments
    /// * `id` - The image ID
    /// * `data` - An grid of booleans: true means the character is '#', false means '.'
    pub fn new(id: u16, data: &[[bool; 10]; 10]) -> Self {
        let mut left = [false; 10];
        let mut right = [false; 10];
        for i in 0..10 {
            left[i] = data[i][0];
            right[i] = data[i][9];
        }

        // This is really ugly, but it is a lot more compressed than storing the raw String
        Self {
            id,
            data: [
                Row::new(&data[1][1..9]),
                Row::new(&data[2][1..9]),
                Row::new(&data[3][1..9]),
                Row::new(&data[4][1..9]),
                Row::new(&data[5][1..9]),
                Row::new(&data[6][1..9]),
                Row::new(&data[7][1..9]),
                Row::new(&data[8][1..9]),
            ],
            borders: [
                Border::new(&data[0]),
                Border::new(&data[9]),
                Border::new(&left),
                Border::new(&right),
            ],
        }
    }

    /// Produce an array of all the arrangement of a tile by flipping it and rotating it
    pub fn all_possibilities(id: u16, data: [[bool; 10]; 10]) -> [Self; 8] {
        // Flip a data array
        fn flip(mut data: [[bool; 10]; 10]) -> [[bool; 10]; 10] {
            data.iter_mut().for_each(|line| line.reverse());
            data
        }

        // Rotate a data array to the right
        fn right(data: [[bool; 10]; 10]) -> [[bool; 10]; 10] {
            let mut next = [[false; 10]; 10];
            for y in 0..10 {
                for x in 0..10 {
                    next[y][x] = data[x][9 - y];
                }
            }
            next
        }

        [
            Self::new(id, &data),
            Self::new(id, &right(data)),
            Self::new(id, &right(right(data))),
            Self::new(id, &right(right(right(data)))),
            Self::new(id, &flip(data)),
            Self::new(id, &flip(right(data))),
            Self::new(id, &flip(right(right(data)))),
            Self::new(id, &flip(right(right(right(data))))),
        ]
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        writeln!(f, "Tile {}:", self.id)?;
        writeln!(f, "{}", self.borders[0])?;
        for i in 0..8 {
            writeln!(
                f,
                "{}{}{}",
                self.borders[2].char_at(i + 1),
                self.data[i],
                self.borders[3].char_at(i + 1)
            )?;
        }
        writeln!(f, "{}", self.borders[1])?;
        Ok(())
    }
}

/// A compressed row of the data part of a tile
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct Row(u8);

impl Row {
    fn new(array: &[bool]) -> Self {
        let mut acc = 0;
        for i in 0..8 {
            acc += if array[i] { 1 << i } else { 0 };
        }

        Self(acc)
    }

    /// Check if the `idx` character is '#'
    fn is_set(&self, idx: usize) -> bool {
        assert!(idx < 8, "index out of bound");
        (self.0 & (1 << idx)) != 0
    }

    /// The original character at `idx`
    fn char_at(&self, idx: usize) -> char {
        if self.is_set(idx) {
            '#'
        } else {
            '.'
        }
    }
}

impl Display for Row {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        use std::fmt::Write;
        for idx in 0..8 {
            f.write_char(self.char_at(idx))?;
        }
        Ok(())
    }
}

/// A compressed border of a tile
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct Border(u16);

impl Border {
    fn new(array: &[bool; 10]) -> Self {
        let mut acc = 0;
        for i in 0..10 {
            acc += if array[i] { 1 << i } else { 0 };
        }

        Self(acc)
    }

    /// Check if the `idx` character is '#'
    fn is_set(&self, idx: usize) -> bool {
        assert!(idx < 10, "index out of bound");
        (self.0 & (1 << idx)) != 0
    }

    /// The original character at `idx`
    fn char_at(&self, idx: usize) -> char {
        if self.is_set(idx) {
            '#'
        } else {
            '.'
        }
    }
}

impl Display for Border {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        use std::fmt::Write;
        for idx in 0..10 {
            f.write_char(self.char_at(idx))?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("test_resources/20-example.txt");
    const EXAMPLE_IMAGE: &str = include_str!("test_resources/20-example-image.txt");
    const MAIN: &str = include_str!("test_resources/20-main.txt");

    #[test]
    fn row_test() {
        let data = [true, false, false, true, true, true, true, true];
        let row = Row::new(&data);
        assert!(row.is_set(0));
        assert!(!row.is_set(1));
        assert!(!row.is_set(2));
        assert!(row.is_set(3));
        assert!(row.is_set(4));
        assert!(row.is_set(5));
        assert!(row.is_set(6));
        assert!(row.is_set(7));
        assert_eq!(row.to_string(), "#..#####");
    }

    #[test]
    fn border_test() {
        let data = [
            true, false, true, false, true, true, true, true, true, false,
        ];
        let border = Border::new(&data);
        assert!(border.is_set(0));
        assert!(!border.is_set(1));
        assert!(border.is_set(2));
        assert!(!border.is_set(3));
        assert!(border.is_set(4));
        assert!(border.is_set(5));
        assert!(border.is_set(6));
        assert!(border.is_set(7));
        assert!(border.is_set(8));
        assert!(!border.is_set(9));
        assert_eq!(border.to_string(), "#.#.#####.");
    }

    #[test]
    fn first_part_example() {
        let tiles = Day::parse(EXAMPLE).unwrap().tiles;
        let image = match_tiles(tiles, 3).unwrap();
        let corners = first_part(&image, 3).unwrap();
        assert_eq!(corners, 20_899_048_083_289);
    }

    #[test]
    fn first_part_main() {
        let tiles = Day::parse(MAIN).unwrap().tiles;
        let image = match_tiles(tiles, IMAGE_WIDTH).unwrap();
        let corners = first_part(&image, IMAGE_WIDTH).unwrap();
        assert_eq!(corners, 140_656_720_229_539);
    }

    #[test]
    fn assemble_image_example() {
        let tiles = Day::parse(EXAMPLE).unwrap().tiles;
        let image = match_tiles(tiles, 3).unwrap();
        let mut image = FullImage::assemble(image, 3);
        let wanted = FullImage {
            data: EXAMPLE_IMAGE
                .lines()
                .map(|line| line.chars().collect_vec())
                .collect_vec(),
            width: 24,
        };

        if image != wanted {
            assert!((0..4).any(|_| {
                if image.clone().flipped() == wanted {
                    return true;
                }

                image = image.rotated_right();
                if image == wanted {
                    return true;
                }

                false
            }));
        }
    }

    #[test]
    fn second_part_example() {
        let tiles = Day::parse(EXAMPLE).unwrap().tiles;
        let image = match_tiles(tiles, 3).unwrap();
        let image = FullImage::assemble(image, 3);
        let roughness = second_part(image);
        assert_eq!(roughness, 273);
    }

    #[test]
    fn second_part_main() {
        let tiles = Day::parse(MAIN).unwrap().tiles;
        let image = match_tiles(tiles, IMAGE_WIDTH).unwrap();
        let image = FullImage::assemble(image, IMAGE_WIDTH);
        let roughness = second_part(image);
        assert_eq!(roughness, 1_885);
    }
}
