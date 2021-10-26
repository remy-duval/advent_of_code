use color_eyre::eyre::Result;
use hashbrown::HashMap;
use itertools::Itertools;

use commons::grid::{Direction, Point};
use commons::Problem;

use super::int_code::{IntCodeError, IntCodeInput, Processor, Status};

pub struct Day;

impl Problem for Day {
    type Input = IntCodeInput;
    const TITLE: &'static str = "Day 11: Space Police";

    fn solve(data: Self::Input) -> Result<()> {
        let memory = data.data;
        let mut hull: HashMap<Point, u8> = HashMap::new();
        println!("\n{}\n", paint_hull(&memory, &mut hull)?);
        println!("The robot painted {} tiles\n\n", hull.len());

        hull.clear();
        hull.insert(Point::new(0, 0), 1);
        println!(
            "The robot painted something:\n{}",
            paint_hull(&memory, &mut hull)?
        );

        Ok(())
    }
}

/// Run the IntCode processor given to paint the hull
fn paint_hull(memory: &[i64], hull: &mut HashMap<Point, u8>) -> Result<String, IntCodeError> {
    let (mut min, mut max) = ((0, 0), (0, 0));
    let mut position = Point::new(0, 0);
    let mut direction = Direction::North;
    let mut program: Processor = memory.into();
    let mut outputs = [0; 2];

    loop {
        program.write_int(*hull.get(&position).unwrap_or(&0) as i64);
        let (read, status) = program.read_next_array(&mut outputs, 2);

        // Check if the status is now Halted
        if let Some(Status::Halted) = status {
            break;
        }

        // Collect the robot output
        if read != 2 {
            return Err(IntCodeError::new(
                "Could not collect the required two outputs !",
            ));
        }
        let (color, movement) = (outputs[0] as u8, outputs[1]);

        // Update the hull and the robot position
        hull.insert(position, color);
        match movement {
            0 => direction = direction.left(),
            _ => direction = direction.right(),
        };
        position = position.moved(direction);

        // Update the hull bounds
        min.0 = min.0.min(position.x);
        min.1 = min.1.min(position.y);
        max.0 = max.0.max(position.x);
        max.1 = max.1.max(position.y);
    }

    Ok(hull_representation(hull, position, min, max))
}

/// Computes the String representation of the current painting job
fn hull_representation(
    hull: &HashMap<Point, u8>,
    robot: Point,
    min: (i64, i64),
    max: (i64, i64),
) -> String {
    let from_x = min.0;
    let from_y = min.1;
    let to_x = max.0 + 1;
    let to_y = max.1 + 1;
    (from_y..to_y)
        .map(|y| {
            (from_x..to_x)
                .map(|x| {
                    let position = Point::new(x, y);
                    if position == robot {
                        '@'
                    } else {
                        match hull.get(&position).unwrap_or(&0) {
                            1 => '#',
                            _ => ' ',
                        }
                    }
                })
                .join("")
        })
        .join("\n")
}

#[cfg(test)]
mod tests;
