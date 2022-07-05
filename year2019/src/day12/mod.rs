use commons::eyre::{eyre, Result};
use hashbrown::HashMap;

use commons::num::integer::lcm;

pub const TITLE: &str = "Day 12: The N-Body Problem";
const DIMENSIONS: usize = 3;
const MOONS: usize = 4;
const STEPS: usize = 1000;

pub fn run(raw: String) -> Result<()> {
    let mut moons = parse(&raw)?;
    // First part
    (0..STEPS).for_each(|_| moons.next());
    let total_energy = moons.energy();
    println!(
        "The total energy of the system after {} steps is {}",
        STEPS, total_energy
    );
    // Second part
    let period = find_periodicity(moons);
    println!("The moons periodicity is {}", period);

    Ok(())
}

fn parse(s: &str) -> Result<Moons> {
    let mut lines = s.lines();
    let mut next = move || match lines.next() {
        Some(line) => Ok(line),
        _ => Err(eyre!("Could not get one of the moon description")),
    };

    let moons = [
        Moons::single_moon_from_str(next()?)?,
        Moons::single_moon_from_str(next()?)?,
        Moons::single_moon_from_str(next()?)?,
        Moons::single_moon_from_str(next()?)?,
    ];

    Ok(Moons::new(moons))
}

/// Finds the period of the system movement.
fn find_periodicity(mut moons: Moons) -> i64 {
    moons.clear();
    let mut step_counter: i64 = 0;
    let mut x: Option<i64> = None;
    let mut y: Option<i64> = None;
    let mut z: Option<i64> = None;
    while x.is_none() || y.is_none() || z.is_none() {
        moons.next();
        step_counter += 1;
        if x.is_none() && moons.is_initial_x() {
            x = Some(step_counter);
        }
        if y.is_none() && moons.is_initial_y() {
            y = Some(step_counter);
        }
        if z.is_none() && moons.is_initial_z() {
            z = Some(step_counter);
        }
    }

    moons.clear();
    if let Some(((a, b), c)) = x.zip(y).zip(z) {
        lcm(lcm(a, b), c)
    } else {
        println!("Could not find the values of x, y and z");
        0
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Moons {
    x: [i32; MOONS],
    y: [i32; MOONS],
    z: [i32; MOONS],
    v_x: [i32; MOONS],
    v_y: [i32; MOONS],
    v_z: [i32; MOONS],
    initial_x: [i32; MOONS],
    initial_y: [i32; MOONS],
    initial_z: [i32; MOONS],
}

impl Moons {
    /// Build a new Moons system.
    pub fn new(moons: [[i32; DIMENSIONS]; MOONS]) -> Moons {
        fn single_dimension(moons: &[[i32; DIMENSIONS]; MOONS], dim: usize) -> [i32; MOONS] {
            let mut iter = moons.iter().map(|x| x[dim]);
            [
                iter.next().unwrap(),
                iter.next().unwrap(),
                iter.next().unwrap(),
                iter.next().unwrap(),
            ]
        }

        let x = single_dimension(&moons, 0);
        let y = single_dimension(&moons, 1);
        let z = single_dimension(&moons, 2);
        Self {
            x,
            y,
            z,
            v_x: [0; MOONS],
            v_y: [0; MOONS],
            v_z: [0; MOONS],
            initial_x: x,
            initial_y: y,
            initial_z: z,
        }
    }

    /// Advance the system step by 1 for x dimension
    pub fn next(&mut self) {
        for i in 0..MOONS {
            // X dimension
            let first = self.x[i];
            let sum = self.x.iter().map(|y| (*y - first).signum()).sum::<i32>();
            self.v_x[i] += sum;

            // Y dimension
            let first = self.y[i];
            let sum = self.y.iter().map(|y| (*y - first).signum()).sum::<i32>();
            self.v_y[i] += sum;

            // Z dimension
            let first = self.z[i];
            let sum = self.z.iter().map(|y| (*y - first).signum()).sum::<i32>();
            self.v_z[i] += sum;
        }
        for i in 0..MOONS {
            self.x[i] += self.v_x[i];
            self.y[i] += self.v_y[i];
            self.z[i] += self.v_z[i];
        }
    }

    /// Clears the moon advance
    pub fn clear(&mut self) {
        self.x = self.initial_x;
        self.y = self.initial_y;
        self.z = self.initial_z;
        self.v_x = [0; MOONS];
        self.v_y = [0; MOONS];
        self.v_z = [0; MOONS];
    }

    /// True if x dimensions is at initial state
    pub fn is_initial_x(&self) -> bool {
        self.x == self.initial_x && self.v_x == [0; MOONS]
    }

    /// True if y dimensions is at initial state
    pub fn is_initial_y(&self) -> bool {
        self.y == self.initial_y && self.v_y == [0; MOONS]
    }

    /// True if z dimensions is at initial state
    pub fn is_initial_z(&self) -> bool {
        self.z == self.initial_z && self.v_z == [0; MOONS]
    }

    /// The total energy of the system
    pub fn energy(&self) -> u32 {
        let mut kinetic = [0u32; MOONS];
        let mut potent = [0u32; MOONS];
        for i in 0..MOONS {
            potent[i] += self.x[i].unsigned_abs();
            potent[i] += self.y[i].unsigned_abs();
            potent[i] += self.z[i].unsigned_abs();
            kinetic[i] += self.v_x[i].unsigned_abs();
            kinetic[i] += self.v_y[i].unsigned_abs();
            kinetic[i] += self.v_z[i].unsigned_abs();
        }

        kinetic
            .iter()
            .zip(potent.iter())
            .map(|(kin, pot)| *kin * *pot)
            .sum()
    }

    /// Parse a single moon from a string slice.
    fn single_moon_from_str(data: &str) -> Result<[i32; DIMENSIONS]> {
        let values: HashMap<char, i32> = data
            .split(',')
            .filter_map(|key_value| {
                let mut key_value = key_value.split('=');
                let key: char = key_value
                    .next()?
                    .chars()
                    .find(|c| *c != '<' && !c.is_whitespace())?;
                let value = key_value
                    .next()?
                    .chars()
                    .filter(|c| *c != '>' && !c.is_whitespace())
                    .collect::<String>()
                    .parse::<i32>()
                    .ok()?;

                Some((key, value))
            })
            .collect();

        match (values.get(&'x'), values.get(&'y'), values.get(&'z')) {
            (Some(x), Some(y), Some(z)) => Ok([*x, *y, *z]),
            _ => Err(eyre!("Could not parse x, y or z from the moon: {}", data)),
        }
    }
}

#[cfg(test)]
mod tests;
