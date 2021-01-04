use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;

use itertools::Itertools;

use commons::Problem;

const WIDTH: usize = 25;
const HEIGHT: usize = 6;

pub struct Day;

impl Problem for Day {
    type Input = Image;
    type Err = anyhow::Error;
    const TITLE: &'static str = "Day 8: Space Image Format";

    fn solve(data: Self::Input) -> Result<(), Self::Err> {
        let mut image = data;
        let (_, w, t) = image.check_sum();
        image.build();
        println!("Image checksum is {} * {} =  {}", w, t, w * t);
        println!("{}", image);

        Ok(())
    }
}

#[derive(Clone)]
pub struct Image {
    layers: Vec<[[u8; WIDTH]; HEIGHT]>,
    built_image: Option<[[u8; WIDTH]; HEIGHT]>,
}

impl Display for Image {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some(layer) = self.built_image {
            write!(f, "Image:\n{}", Self::layer_representation(layer))
        } else {
            write!(f, "Image is not yet built !")
        }
    }
}

impl Debug for Image {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let repr = self
            .layers
            .iter()
            .map(|l| Self::layer_representation(*l))
            .join("\n\n");
        write!(f, "Layers:\n{}", repr)
    }
}

impl FromStr for Image {
    type Err = &'static str;

    fn from_str(data: &str) -> Result<Self, Self::Err> {
        let size_hint = data.len() / WIDTH / HEIGHT;
        let mut chars = data.chars();
        let layers: Option<Vec<_>> = (0..size_hint)
            .map(move |_| {
                let mut layer = [[0u8; WIDTH]; HEIGHT];
                for line in layer.iter_mut() {
                    for pixel in line.iter_mut() {
                        let digit: u8 = chars.next()?.to_digit(10)? as u8;
                        *pixel = digit;
                    }
                }

                Some(layer)
            })
            .collect();

        Ok(Self::new(match layers {
            Some(ok) => ok,
            None => return Err("Could not build the Image."),
        }))
    }
}

impl Image {
    /// Build a new Image from the given layers.
    pub fn new(layers: Vec<[[u8; WIDTH]; HEIGHT]>) -> Self {
        Self {
            layers,
            built_image: None,
        }
    }

    /// Counts the number of pixels of each type (black, white transparent) in the checksum layer
    pub fn check_sum(&self) -> (usize, usize, usize) {
        let checked_layer = self.check_sum_layer();
        checked_layer
            .iter()
            .flatten()
            .fold((0, 0, 0), |acc, next| match *next {
                0 => (acc.0 + 1, acc.1, acc.2),
                1 => (acc.0, acc.1 + 1, acc.2),
                _ => (acc.0, acc.1, acc.2 + 1),
            })
    }

    /// Built the final image by flattening all layers into a single one
    /// (the top most not transparent layer wins for each pixel)
    pub fn build(&mut self) {
        let mut flattened = [[0u8; WIDTH]; HEIGHT];
        for (i, line) in flattened.iter_mut().enumerate() {
            for (j, pixel) in line.iter_mut().enumerate() {
                *pixel = self
                    .layers
                    .iter()
                    .map(|elt| elt[i][j])
                    .find(|current| *current < 2)
                    .unwrap_or(2);
            }
        }

        self.built_image = Some(flattened);
    }

    /// Formats a layer to a String.
    fn layer_representation(layer: [[u8; WIDTH]; HEIGHT]) -> String {
        layer
            .iter()
            .map(|line| {
                line.iter()
                    .map(|pixel| match pixel {
                        0 => ' ',
                        1 => '#',
                        _ => '?',
                    })
                    .join("")
            })
            .join("\n")
    }

    /// Returns the layer with the fewest pixels being 0 (black pixel) for the checksum
    fn check_sum_layer(&self) -> [[u8; WIDTH]; HEIGHT] {
        let mut min = WIDTH * HEIGHT;
        let mut best = [[0; WIDTH]; HEIGHT];
        for layer in self.layers.iter() {
            let result = layer.iter().flatten().filter(|x| **x == 0).count();
            if result < min {
                min = result;
                best = *layer;
            }
        }

        best
    }
}

#[cfg(test)]
mod tests;
