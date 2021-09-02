use std::cmp::Ordering;
use std::collections::BinaryHeap;

use color_eyre::eyre::{eyre, Result};
use hashbrown::HashMap;

use commons::Problem;
use data::{Cavern, Point, Tool};

/// The data structures to represent the cavern
mod data;

pub struct Day;

impl Problem for Day {
    type Input = Cavern;
    const TITLE: &'static str = "Day 22: Mode Maze";

    fn solve(mut cavern: Self::Input) -> Result<()> {
        let risk = cavern.risk_level();
        println!("The total danger level in the maze is {}", risk);

        let shortest = shortest_path(&mut cavern)
            .ok_or_else(|| eyre!("Could not find the shortest path to the target"))?;
        println!("The shortest path to the target is {}", shortest);

        Ok(())
    }
}

/// Find the length of the shortest path to reach a position with the given tool
fn shortest_path(cavern: &mut Cavern) -> Option<u32> {
    let dest = cavern.target;
    let size = (dest.0 as usize + 1) * (dest.1 as usize + 1);

    // Point -> (current distance, current tool)
    let mut mappings = HashMap::with_capacity(size);
    let mut queue = BinaryHeap::with_capacity(size);
    {
        let start = (0, 0);
        let estimate = data::distance(start, dest);
        mappings.insert((Tool::Torch, start), 0);
        mappings.insert((Tool::ClimbingGear, start), 7);
        queue.push(WeightedPoint(start, Tool::Torch, estimate));
        queue.push(WeightedPoint(start, Tool::ClimbingGear, estimate + 7));
    }

    while let Some(WeightedPoint(point, tool, _)) = queue.pop() {
        if let Some(distance) = mappings.get(&(tool, point)).copied() {
            if tool == Tool::Torch && point == dest {
                return Some(distance);
            }

            for_each_neighbour(point, |neighbour| {
                let tile = cavern.get_or_insert(neighbour);
                // Check if we can travel there, if we can't skip it
                if !tool.can_be_used(tile) {
                    return;
                }

                let estimate = data::distance(neighbour, dest);
                let mut push = |tool: Tool, cost: u32| {
                    let score = distance + cost;
                    let prev_score = mappings
                        .entry((tool, neighbour))
                        .or_insert_with(|| u32::MAX);

                    if score < *prev_score {
                        *prev_score = score;
                        queue.push(WeightedPoint(neighbour, tool, score + estimate));
                    }
                };

                push(tool, 1);
                push(tool.switch_tool(tile), 8);
            });
        }
    }

    None
}

/// Execute an action for each neighbour of a point
fn for_each_neighbour((x, y): Point, mut action: impl FnMut(Point)) {
    action((x + 1, y));
    action((x, y + 1));
    if let Some(x_dec) = x.checked_sub(1) {
        action((x_dec, y))
    };
    if let Some(y_dec) = y.checked_sub(1) {
        action((x, y_dec))
    };
}

/// A struct to hold the current state of a path inside a BinaryHeap as a priority queue
/// The elements are ordered according to the 3rd element
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct WeightedPoint(Point, Tool, u32);

impl Ord for WeightedPoint {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reversed (normally self comp other) to make the binary heap a min heap
        other.2.cmp(&self.2)
    }
}

impl PartialOrd for WeightedPoint {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests;
