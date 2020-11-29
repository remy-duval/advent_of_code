use std::collections::HashMap;
use std::error::Error;

use itertools::Itertools;

use aoc::generator::data_from_cli;
use aoc::grid::{Direction, Point};
use aoc::int_code::{parse_int_code, IntCodeError, Processor, Status};

const TITLE: &str = "Day 11: Space Police";
const DATA: &str = include_str!("../resources/day11.txt");

fn main() -> Result<(), Box<dyn Error>> {
    let data = data_from_cli(TITLE, DATA);
    println!("{}", aoc::CLEAR_COMMAND);
    let memory = parse_int_code(&data)?;
    println!("{}", TITLE);
    // First part
    let mut hull: HashMap<Point, u8> = HashMap::new();
    let first_paint: String = paint_hull(&memory, &mut hull)?;
    println!("{}{}", aoc::TO_TOP, first_paint);
    println!("The robot painted {} tiles", hull.len());

    // Second part
    println!("{}", aoc::CLEAR_COMMAND);
    hull.clear();
    hull.insert(Point::new(0, 0), 1);
    let second_paint: String = paint_hull(&memory, &mut hull)?;
    println!("{}{}", aoc::TO_TOP, second_paint);

    Ok(())
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

    Ok(hull_representation(&hull, position, min, max))
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
mod test {

    use super::*;

    const EXPECTED: &str = " ###  #### ###  #### ###  ###  #  #  ##    \n \
                            #  #    # #  # #    #  # #  # # #  #  #   \n \
                            #  #   #  #  # ###  #  # #  # ##   #      \n \
                            ###   #   ###  #    ###  ###  # #  #      \n \
                            #    #    # #  #    #    # #  # #  #  # @ \n \
                            #    #### #  # #    #    #  # #  #  ##    ";

    #[test]
    fn solve_test() -> Result<(), Box<dyn Error>> {
        let memory = parse_int_code(&DATA)?;
        let mut hull: HashMap<Point, u8> = HashMap::new();
        hull.insert(Point::new(0, 0), 1);
        let second_paint: String = paint_hull(&memory, &mut hull)?;

        assert_eq!(EXPECTED, &second_paint);

        Ok(())
    }
}
