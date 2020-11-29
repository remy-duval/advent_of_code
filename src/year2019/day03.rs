use std::collections::HashMap;

use anyhow::anyhow;

use crate::commons::grid::{Direction, Point};
use crate::parse::{CommaSep, LineSep};

pub struct Day03;

impl crate::Problem for Day03 {
    type Input = LineSep<CommaSep<String>>;
    type Err = anyhow::Error;
    const TITLE: &'static str = "Day 3: Crossed Wires";

    fn solve(data: Self::Input) -> Result<(), Self::Err> {
        let crossed = parse_all(data, 2).ok_or_else(|| anyhow!("Parse error !"))?;
        let closest = closest(&crossed[..]).ok_or_else(|| anyhow!("Could not find closest !"))?;
        let (shortest, length) =
            shortest(&crossed[..]).ok_or_else(|| anyhow!("Could not find shortest !"))?;

        println!(
            "Closest cross to origin : {} with distance {}",
            closest,
            closest.manhattan_distance()
        );
        println!(
            "Shortest cross to origin : {} with length {}",
            shortest, length
        );

        Ok(())
    }
}

/// Return the crossing point closest to the origin (according to manhattan distance)
fn closest(crossed: &[(Point, i64)]) -> Option<Point> {
    let mut min: Option<Point> = None;
    let mut distance = std::i64::MAX;
    for (point, _) in crossed.iter() {
        let current = point.manhattan_distance();
        if current < distance {
            min = Some(*point);
            distance = current;
        }
    }
    min
}

/// Return the crossing point with the shortest length (second member of the tuple)
fn shortest(crossed: &[(Point, i64)]) -> Option<(Point, i64)> {
    let mut min: Option<Point> = None;
    let mut distance = std::i64::MAX;
    for (point, length) in crossed.iter() {
        if *length < distance {
            min = Some(*point);
            distance = *length;
        }
    }
    min.map(move |x| (x, distance))
}

/// Parse all the data into the crossing points of the grid.
fn parse_all(data: LineSep<CommaSep<String>>, cable_number: i8) -> Option<Vec<(Point, i64)>> {
    fn parse_cable(cable: CommaSep<String>) -> Option<HashMap<Point, i64>> {
        let mut current = Point::default();
        let mut from_origin = 0;
        let mut acc: HashMap<Point, i64> = HashMap::new();
        for movement in cable.data.iter() {
            let mut chars = movement.chars();
            let direction = match chars.next() {
                Some('R') => Direction::East,
                Some('U') => Direction::North,
                Some('D') => Direction::South,
                Some('L') => Direction::West,
                _ => return None,
            };
            let end = chars.collect::<String>().parse::<i64>().ok()?;
            for _ in 0..end {
                current = current.moved(direction);
                from_origin += 1;
                acc.insert(current, from_origin);
            }
        }
        Some(acc)
    }

    // The values are : (number of overlapping cables, combined distance from origin)
    let mut result: HashMap<Point, (i8, i64)> = HashMap::new();
    for raw_cable in data.data {
        if let Some(cable) = parse_cable(raw_cable) {
            for (point, from_origin) in cable {
                result
                    .entry(point)
                    .and_modify(|x| {
                        x.0 += 1;
                        x.1 += from_origin;
                    })
                    .or_insert((1, from_origin));
            }
        };
    }
    let crossed: Vec<(Point, i64)> = result
        .iter()
        .filter_map(|item| {
            let (point, (overlap, distance)) = item;
            if *overlap >= cable_number {
                Some((*point, *distance))
            } else {
                None
            }
        })
        .collect();
    Some(crossed)
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = include_str!("test_resources/day03.txt");

    const A: &str = "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83";
    const B: &str =
        "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7";

    #[test]
    fn first_part() {
        let first = parse_all(A.parse().unwrap(), 2).expect("Parse error !");
        let second = parse_all(B.parse().unwrap(), 2).expect("Parse error !");

        let result = closest(&first[..])
            .expect("Could not find closest !")
            .manhattan_distance();
        assert_eq!(result, 159);
        let result = closest(&second[..])
            .expect("Could not find closest !")
            .manhattan_distance();
        assert_eq!(result, 135);
    }

    #[test]
    fn second_part() {
        let first = parse_all(A.parse().unwrap(), 2).expect("Parse error !");
        let second = parse_all(B.parse().unwrap(), 2).expect("Parse error !");

        let result = shortest(&first[..]).expect("Could not find shortest !").1;
        assert_eq!(result, 610);
        let result = shortest(&second[..]).expect("Could not find shortest !").1;
        assert_eq!(result, 410);
    }

    #[test]
    fn solve_test() {
        let crossed = parse_all(DATA.parse().unwrap(), 2).expect("Parse error !");
        let closest = closest(&crossed[..]).expect("Could not find closest !");
        let length = shortest(&crossed[..]).expect("Could not find shortest !").1;

        assert_eq!(529, closest.manhattan_distance());
        assert_eq!(20_386, length);
    }
}
