use color_eyre::eyre::{eyre, Result};

use commons::Problem;

pub struct Day;

impl Problem for Day {
    type Input = Binaries;
    const TITLE: &'static str = "Day 3: Binary Diagnostic";

    fn solve(data: Self::Input) -> Result<()> {
        println!("1: Power consumption is {}", first_part(&data));
        println!("2: Life support is {}", second_part(&data)?);

        Ok(())
    }
}

/// Parsed values from the input
#[derive(Clone, Debug)]
pub struct Binaries {
    /// All the binary ratings that were parsed from the input
    pub data: Vec<Binary>,
    /// The number of bits for each of the binaries we have
    pub bits: usize,
}

/// A number that is parsed from a binary
#[derive(Copy, Clone, Debug)]
pub struct Binary(pub u16);

impl Binary {
    /// Check if the given bit is zero
    pub fn is_zero_bit(&self, bit: usize) -> bool {
        self.0 & (1 << bit) == 0
    }
}

impl std::str::FromStr for Binaries {
    type Err = core::num::ParseIntError;

    fn from_str(s: &str) -> core::result::Result<Self, Self::Err> {
        let mut lines = s.lines().peekable();
        let bits = lines.peek().map_or(0, |line| line.chars().count());
        let data = lines
            .map(|l| u16::from_str_radix(l, 2).map(Binary))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Binaries { data, bits })
    }
}

/// Find the gamma and epsilon in the data and multiply them
fn first_part(data: &Binaries) -> u32 {
    // For each bit position, (number of ones in the binaries, number of zeroes in the data)
    let mut counts = vec![(0, 0); data.bits];
    data.data.iter().for_each(|next| {
        counts
            .iter_mut()
            .rev()
            .enumerate()
            .for_each(|(i, (ones, zeroes))| {
                if next.is_zero_bit(i) {
                    *zeroes += 1;
                } else {
                    *ones += 1;
                }
            });
    });

    // Now that we have the correct counts per bit, reconstruct the gamma and epsilon numbers
    // Gamma is the number formed only of all the most common bits
    // Epsilon is the number formed only of all the least commons bit
    let (gamma, epsilon) = counts
        .into_iter()
        .fold((0, 0), |(gamma, epsilon), (ones, zeroes)| {
            if ones >= zeroes {
                (gamma * 2 + 1, epsilon * 2)
            } else {
                (gamma * 2, epsilon * 2 + 1)
            }
        });

    gamma * epsilon
}

/// Find the oxygen rating and carbon ratings in the data and multiply them
fn second_part(data: &Binaries) -> Result<u32> {
    let oxygen = rating(data, false)?;
    let carbon = rating(data, true)?;

    Ok(oxygen.0 as u32 * carbon.0 as u32)
}

/// Filter the data for the more / least common bit, from the rightmost bit, until there is one left
fn rating(data: &Binaries, oxygen: bool) -> Result<Binary> {
    let mut remaining = data.data.clone();
    for current_bit in (0..data.bits).rev() {
        // Count the number of ones and zeroes for this specific bit position
        let mut ones = 0;
        let mut zeroes = 0;
        remaining.iter().for_each(|bin| {
            if bin.is_zero_bit(current_bit) {
                zeroes += 1;
            } else {
                ones += 1;
            }
        });

        // For the oxygen, keep only values with the most common bit (1 if tie)
        // For the carbon, keep only values with the least common bit (0 if tie)
        let keep_zeroes = if oxygen {
            zeroes > ones
        } else {
            zeroes <= ones
        };
        remaining.retain(|bin| bin.is_zero_bit(current_bit) == keep_zeroes);

        // If we have one element remaining: stop here
        if remaining.len() == 1 {
            return Ok(remaining[0]);
        }
    }

    Err(eyre!(
        "Rating not found after sieving, there remains the following {:?}",
        remaining
    ))
}

#[cfg(test)]
mod tests;
