use std::ops::Add;

use std::collections::{HashMap, HashSet};

use commons::grid::Point;
use commons::Result;

pub const TITLE: &str = "Day 20: A Regular Map";

pub fn run(regex: String) -> Result<()> {
    let map = build_map(&regex);
    println!("The furthest room is {} doors away", first_part(&map));
    println!("{} rooms are more than 1000 doors away", second_part(&map));

    Ok(())
}

/// Find the furthest room in the facility, returns the length of its path
fn first_part(map: &HashMap<Point, u16>) -> u16 {
    map.values()
        .max_by_key(|room| **room)
        .map_or(0, |room| *room)
}

/// Count the number of rooms that are further than at least 1000 doors from the start
fn second_part(map: &HashMap<Point, u16>) -> usize {
    map.values().filter(|room| **room >= 1000).count()
}

/// Explore the entire regex given to build a map of the facility
fn build_map(regex: &str) -> HashMap<Point, u16> {
    let mut rooms: HashMap<Point, u16> = HashMap::with_capacity(regex.len());

    let mut stack = vec![]; // Store the previous groups data
    let mut current = vec![Point::new(0, 0)]; // The points to explore with
    let mut starts = HashSet::new(); // The points from before the group started
    let mut ends = HashSet::new(); // The end points of both parts of the group

    starts.insert(Point::new(0, 0));
    rooms.insert(Point::new(0, 0), 0);
    regex.chars().for_each(|char| {
        if char == '(' {
            // Push the previous group data on the stack
            stack.push(std::mem::take(&mut starts));
            stack.push(std::mem::take(&mut ends));
            // Save the initial position for when the second part starts
            starts.extend(current.iter().copied());
        } else if char == '|' {
            // Remember the ending positions after the first part
            ends.extend(current.drain(..));
            // Return to the saved initial position, while saving the current end
            current.extend(starts.iter().copied());
        } else if char == ')' {
            // Compute the ending positions after the second part, using them as the new current
            ends.extend(current.drain(..));
            current.extend(ends.drain());
            // Retrieve the previous group data
            ends = stack.pop().unwrap_or_default();
            starts = stack.pop().unwrap_or_default();
        } else {
            let offset = match char {
                'E' => Point::new(1, 0),
                'W' => Point::new(-1, 0),
                'N' => Point::new(0, -1),
                'S' => Point::new(0, 1),
                _ => return,
            };

            current.iter_mut().for_each(|point| {
                let distance = rooms.get(point).copied().unwrap_or_default() + 1;
                *point = point.add(&offset);
                rooms
                    .entry(*point)
                    .and_modify(|room| *room = distance.min(*room))
                    .or_insert_with(|| distance);
            });
        }
    });

    rooms
}

#[cfg(test)]
mod tests;
