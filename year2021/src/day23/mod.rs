use std::cmp::Reverse;
use std::collections::BinaryHeap;

use std::collections::HashMap;

use commons::{err, Result};

pub const TITLE: &str = "Day 23: Amphipod";

pub fn run(raw: String) -> Result<()> {
    let rows = parse(&raw)?;
    let first = Positions::from(rows).a_star_search().unwrap_or(usize::MAX);
    println!("1. Min cost: {first}");
    let second = add_rows(&rows).a_star_search().unwrap_or(usize::MAX);
    println!("2. Min cost with added units: {second}");

    Ok(())
}

type Tile = u8;
const A: Tile = 0;
const B: Tile = 1;
const C: Tile = 2;
const D: Tile = 3;
const EMPTY: Tile = 4;
const COSTS: [usize; 4] = [1, 10, 100, 1000];

#[derive(Debug, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
struct Positions<const DEPTH: usize> {
    hallway: [Tile; 11],
    rooms: [[Tile; DEPTH]; 4],
}

fn parse(s: &str) -> Result<[[Tile; 4]; 2]> {
    let mut done: [[Tile; 4]; 2] = [[EMPTY; 4]; 2];
    let count = s
        .lines()
        .filter_map(|line| {
            let mut result = [EMPTY; 4];
            let count = line
                .chars()
                .filter(|c| matches!(c, 'A' | 'B' | 'C' | 'D'))
                .map(|c| c as u8 - b'A')
                .zip(result.iter_mut())
                .map(|(tile, dest)| *dest = tile)
                .count();

            if count == result.len() {
                Some(result)
            } else {
                None
            }
        })
        .enumerate()
        .try_fold(0, |acc, (depth, tiles)| {
            if let Some(result) = done.get_mut(depth) {
                *result = tiles;
                Ok(acc + 1)
            } else {
                Err(err!("The rooms are too deep in {}", s))
            }
        })?;

    if count == 2 {
        Ok(done)
    } else {
        Err(err!("The rooms aren't deep enough in {}", s))
    }
}

fn add_rows(rows: &[[Tile; 4]; 2]) -> Positions<4> {
    let mut done: [[Tile; 4]; 4] = Default::default();
    done[0] = rows[0];
    done[1] = [D, C, B, A];
    done[2] = [D, B, A, C];
    done[3] = rows[1];
    Positions::from(done)
}

impl<const DEPTH: usize> Positions<DEPTH> {
    /// Parse the rows of units into the position set
    fn from(rows: [[Tile; 4]; DEPTH]) -> Self {
        let hallway = [EMPTY; 11];
        let mut rooms = [[EMPTY; DEPTH]; 4];
        rows.into_iter().enumerate().for_each(|(depth, tiles)| {
            tiles.iter().zip(rooms.iter_mut()).for_each(|(t, dest)| {
                dest[depth] = *t;
            })
        });

        Self { hallway, rooms }
    }

    /// Find the furthest depth of this room that is not empty
    /// At the same time Checks whether the remaining tiles are the correct ones
    fn check_room(&self, room: usize) -> (Option<(usize, Tile)>, bool) {
        let mut first_non_empty = None;
        for (i, &t) in self.rooms[room].iter().enumerate() {
            if t != EMPTY {
                if first_non_empty.is_none() {
                    first_non_empty = Some((i, t));
                }
                if t as usize != room {
                    return (first_non_empty, false);
                }
            }
        }

        (first_non_empty, true)
    }

    /// The index of the entrance of a room in the hallway
    fn room_entrance(room: usize) -> usize {
        2 * room + 2
    }

    /// The path between the two hallway indexes
    fn hallway_path(first: usize, second: usize) -> std::ops::Range<usize> {
        if first < second {
            first..(second + 1)
        } else {
            second..(first + 1)
        }
    }

    /// Check if the move from this room entrance is possible to the given hallway spot
    ///
    /// If it is possible, return `Some(steps)`, otherwise `None`
    fn steps_between(&self, room: usize, hallway: usize) -> Option<usize> {
        let path = Self::hallway_path(Self::room_entrance(room), hallway);
        let steps = path.end - path.start;
        for i in path {
            if i != hallway && self.hallway[i] != EMPTY {
                return None;
            }
        }
        Some(steps)
    }

    /// Try to make a unit exit its current room and stand in the hallway
    fn exit(&self, room: usize, hallway: usize) -> Option<(Self, usize)> {
        let (first_non_empty, rest_correct) = self.check_room(room);
        let (depth, moved) = first_non_empty?;
        if rest_correct {
            return None;
        }

        let steps = self.steps_between(room, hallway)?;
        let mut next = self.clone();
        next.rooms[room][depth] = EMPTY;
        next.hallway[hallway] = moved;

        let cost = COSTS[moved as usize] * (steps + depth);
        Some((next, cost))
    }

    /// Try to make a unit in the hallway enter the correct room if possible
    fn enter(&self, room: usize, hallway: usize) -> Option<(Self, usize)> {
        let (first_non_empty, rest_correct) = self.check_room(room);
        if !rest_correct {
            return None;
        }

        let depth = first_non_empty.map_or(DEPTH, |(i, _)| i).checked_sub(1)?;
        let steps = self.steps_between(room, hallway)?;
        let mut next = self.clone();
        next.hallway[hallway] = EMPTY;
        next.rooms[room][depth] = room as u8;

        Some((next, COSTS[room] * (steps + depth)))
    }

    /// Estimate the remaining cost to the destination as the minimum distance for all elements
    fn heuristic(&self) -> usize {
        let mut total = 0;

        // The number of steps to go down in the correct room
        // Increases when there are more elements that need to go down
        let mut down = [1; 4];

        // From hallway to the correct room entrance
        // The estimate cost is hallway travel + going down
        for (i, &t) in self.hallway.iter().enumerate() {
            if t != EMPTY {
                let t = t as usize;
                let path = Self::hallway_path(i, Self::room_entrance(t));
                total += (path.end - path.start + down[t]) * COSTS[t];
                down[t] += 1;
            }
        }

        // From a bad room to a correct room entrance
        // The estimate cost is going up to the hallway + hallway travel + going down
        for (i, room) in self.rooms.iter().enumerate() {
            for (up, &t) in room.iter().enumerate() {
                if t != EMPTY && i != t as usize {
                    let t = t as usize;
                    let path = Self::hallway_path(Self::room_entrance(i), Self::room_entrance(t));
                    total += (up + path.end - path.start + down[t]) * COSTS[t];
                    down[t] += 1;
                }
            }
        }

        total
    }

    /// Find the minimum cost to move all units to their correct positions
    fn a_star_search(self) -> Option<usize> {
        let mut queue = BinaryHeap::new();
        let mut cache = HashMap::new();
        cache.insert(self.clone(), 0);
        queue.push((Reverse(1), 0, self));

        while let Some((Reverse(estimate), cost, positions)) = queue.pop() {
            // On a correct position the heuristic is 0
            // Since we are doing a A* the first time we see a correct position, it is the best
            if estimate == cost {
                return Some(cost);
            }
            if cache.get(&positions).map_or(false, |c| *c < cost) {
                continue;
            }

            let mut produce = |next: Positions<DEPTH>, cost: usize| {
                let min = *cache
                    .entry(next.clone())
                    .and_modify(|c| *c = cost.min(*c))
                    .or_insert(cost);

                if min <= cost {
                    queue.push((Reverse(cost + next.heuristic()), cost, next));
                }
            };

            for hallway in [0, 1, 3, 5, 7, 9, 10] {
                // If the hallway spot is empty, try to move an unit into it
                // Otherwise try to move its unit into its correct room
                match positions.hallway[hallway] {
                    EMPTY => {
                        (0..4)
                            .filter_map(|room| positions.exit(room, hallway))
                            .for_each(|(next, c)| produce(next, cost + c));
                    }
                    occupied => {
                        if let Some((next, c)) = positions.enter(occupied as usize, hallway) {
                            produce(next, cost + c);
                        }
                    }
                }
            }
        }

        None
    }
}

#[cfg(test)]
mod tests;
