use itertools::Itertools;

use commons::eyre::{eyre, Result};

pub const TITLE: &str = "Day 20: Trench Map";

pub fn run(raw: String) -> Result<()> {
    let (enhancer, mut image) = parse(&raw)?;
    let (first, second) = enhance(&mut image, &enhancer);
    println!("1. After two steps: {first}");
    println!("2. After fifty steps: {second}");

    Ok(())
}

fn parse(s: &str) -> Result<(Enhancer, Image)> {
    let (enhancer, image) = commons::parse::sep_by_empty_lines(s)
        .collect_tuple()
        .ok_or_else(|| eyre!("Missing sections in {}", s))?;

    Ok((Enhancer::parse(enhancer), Image::parse(image)))
}

/// Enhance the image 50x, returning the pixels after 2 and 50 enhancements
fn enhance(image: &mut Image, alg: &Enhancer) -> (usize, usize) {
    let mut buffer = Image::default();
    image.compute_next(alg, &mut buffer);
    buffer.compute_next(alg, image);
    let first = image.pixels();
    for _ in 0..48 {
        image.compute_next(alg, &mut buffer);
        std::mem::swap(image, &mut buffer);
    }
    (first, image.pixels())
}

/// The current image
#[derive(Debug, Default)]
struct Image {
    background: bool,
    center: Vec<bool>,
    width: isize,
}

impl Image {
    /// Compute the next image
    fn compute_next(&self, alg: &Enhancer, into: &mut Self) {
        let height = self.center.len() as isize / self.width;
        into.width = self.width + 2;
        into.background = alg.get([self.background; 9]);
        into.center.clear();
        into.center
            .reserve((into.width as usize) * (height as usize + 2));
        (-1..(height + 1)).for_each(|y| {
            (-1..(self.width + 1)).for_each(|x| {
                into.center.push(alg.get(self.square(x, y)));
            });
        });
    }

    /// The number of lit pixels in the image
    fn pixels(&self) -> usize {
        if self.background {
            usize::MAX
        } else {
            self.center.iter().filter(|v| **v).count()
        }
    }

    /// The pixel value of this index
    fn pixel(&self, x: isize, y: isize) -> bool {
        if x < 0 || x >= self.width || y < 0 {
            self.background
        } else if let Some(t) = self.center.get((y * self.width + x) as usize) {
            *t
        } else {
            self.background
        }
    }

    /// The 3x3 square centered on this index
    fn square(&self, x: isize, y: isize) -> [bool; 9] {
        [
            self.pixel(x - 1, y - 1),
            self.pixel(x, y - 1),
            self.pixel(x + 1, y - 1),
            self.pixel(x - 1, y),
            self.pixel(x, y),
            self.pixel(x + 1, y),
            self.pixel(x - 1, y + 1),
            self.pixel(x, y + 1),
            self.pixel(x + 1, y + 1),
        ]
    }

    /// Parse the image from the current paragraph
    fn parse(s: &str) -> Self {
        let width = s.lines().next().map_or(0, |l| l.chars().count());
        let mut center = Vec::with_capacity(width * width);
        s.lines().for_each(|line| {
            line.chars()
                .chain(std::iter::repeat('.'))
                .take(width)
                .for_each(|c| center.push(c == '#'));
        });

        Self {
            background: false,
            center,
            width: width as isize,
        }
    }
}

/// The enhancing algorithm expressed as a bit set
struct Enhancer([u8; 64]);

impl Enhancer {
    /// Get the correct pixel for an index
    fn get(&self, value: [bool; 9]) -> bool {
        let index = value.into_iter().fold(0, |a, b| (a << 1) + b as usize);
        match self.0.get(index >> 3) {
            Some(value) => (*value & (1 << (index & 0b111))) != 0,
            None => false,
        }
    }

    /// Parse the enhancing algorithm from the current line
    fn parse(s: &str) -> Self {
        let mut result = [0u8; 64];
        let mut chars = s.chars();
        result.iter_mut().for_each(|set| {
            (0..8)
                .zip(&mut chars)
                .filter(|(_, c)| *c == '#')
                .for_each(|(pos, _)| *set |= 1 << pos);
        });

        Self(result)
    }
}

#[cfg(test)]
mod tests;
