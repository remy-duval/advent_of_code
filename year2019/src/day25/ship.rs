//! The data for representing the RPG of day 25

use std::collections::VecDeque;
use std::fmt::{Display, Formatter, Result as FmtResult};

use hashbrown::HashMap;
use itertools::Itertools;

/// The status of the ship
#[derive(Debug, Default)]
pub struct Ship {
    pub checkpoint: Option<usize>,
    pub current: usize,
    pub rooms: Vec<Room>,
    pub items: Vec<Item>,
}

impl Ship {
    /// Find a path to any known room via breadth-first-search
    pub fn find_path(&self, to: usize) -> Option<Vec<Direction>> {
        if self.current == to {
            return Some(Vec::new());
        }

        let mut queue: VecDeque<_> = self.rooms[self.current]
            .explored_directions()
            .map(|(direction, room)| (room, vec![direction]))
            .collect();

        while let Some((room, path)) = queue.pop_front() {
            if room == to {
                return Some(path);
            }

            for (dir, next) in self.rooms[room].explored_directions() {
                if next != room {
                    let mut path = path.clone();
                    path.push(dir);
                    queue.push_back((next, path));
                }
            }
        }

        None
    }

    /// Find the path to the next unexplored room in the ship
    ///
    /// ### Returns
    /// * Some((path, last_explored)): the path to follow and the index of the last explored room
    /// * None: if no room remains to explore
    pub fn find_unexplored(&mut self) -> Option<(Vec<Direction>, usize)> {
        let new_index = self.rooms.len();
        if let Some(found) = self.rooms[self.current].explore_new_direction(new_index) {
            return Some((vec![found], self.current));
        } else {
            for other in 0..self.rooms.len() {
                if other == self.current {
                    continue;
                }

                if let Some(found) = self.rooms[other].explore_new_direction(new_index) {
                    let mut path = self.find_path(other)?;
                    path.push(found);
                    return Some((path, other));
                }
            }
        }

        None
    }

    /// Find the path to the checkpoint
    pub fn go_to_checkpoint(&self) -> Option<(Vec<Direction>, usize)> {
        self.checkpoint
            .and_then(|position| Some((self.find_path(position)?, position)))
    }

    /// Add the first room of the ship
    pub fn explore_first_room(&mut self, txt: &str) {
        let room = Room::new(txt, None);
        self.rooms.push(room);
        self.current = 0;
    }

    /// Add a new room the the explored rooms
    ///
    /// ### Arguments
    /// * `txt` - The room description
    /// * `from` The direction the room is entered from
    pub fn on_explored_room(&mut self, txt: &str, from: Direction) {
        let room = Room::new(txt, Some((from, self.current)));
        let checkpoint = room.checkpoint;
        self.rooms.push(room);
        self.current = self.rooms.len() - 1;
        if checkpoint {
            self.checkpoint = Some(self.current);
        }
    }

    /// True if there are items to pick up in this room
    pub fn has_items_to_pick(&self) -> bool {
        !self.rooms[self.current].items.is_empty()
    }

    /// Remove items in the current room and return them
    pub fn take_items(&mut self) -> Vec<Item> {
        std::mem::take(&mut self.rooms[self.current].items)
    }
}

#[derive(Debug, Clone)]
pub struct Room {
    checkpoint: bool,
    directions: HashMap<Direction, Option<usize>>,
    items: Vec<Item>,
}

impl Room {
    /// The name of the room right before the objective
    const CHECKPOINT: &'static str = "Security Checkpoint";

    /// Create a new room on the first visit to one
    ///
    /// ### Arguments
    /// * `text` - The text outputted by the game representing the room
    /// * `from` - Maybe the direction we come from (with the index of the previous room)
    ///
    /// ### Returns
    /// A new room, with the direction we are coming from already filled in
    pub fn new(text: &str, from: Option<(Direction, usize)>) -> Self {
        // Remove all text until the room description start '='
        let text = text.trim_start_matches(|char| char != '=');

        let checkpoint = text
            .split_terminator("\n\n")
            .next()
            .and_then(|description| Some(description.strip_prefix("== ")?.split_once(" ==")?.0))
            .map_or(false, |name| {
                println!("Discovered room: {}", name);
                name == Self::CHECKPOINT
            });

        let directions = text
            .split_terminator("\n\n")
            .find_map(Direction::read_block)
            .map_or_else(HashMap::new, |parsed| {
                let (from, index) = from.map_or_else(
                    || (Direction::North, None),
                    |(direction, index)| (direction, Some(index)),
                );

                parsed
                    .into_iter()
                    .map(|dir| {
                        if dir == from {
                            (dir, index)
                        } else {
                            (dir, None)
                        }
                    })
                    .collect()
            });

        println!("Available directions: {}", directions.keys().join(", "));

        let mut room = Self {
            checkpoint,
            directions,
            items: Vec::new(),
        };
        room.on_visit(text);
        room
    }

    /// When visiting a new room, lookup any non dangerous item on the floor
    pub fn on_visit(&mut self, text: &str) {
        // Remove all text until the room description start '='
        let text = text.trim_start_matches(|char| char != '=');

        if let Some(items) = Self::lookup_items(text) {
            self.items = items;
            println!("Available items: {}", self.items.iter().join(", "));
        }
    }

    /// An iterator over the direction which lead to explored rooms
    pub fn explored_directions(&self) -> impl Iterator<Item = (Direction, usize)> + '_ {
        self.directions
            .iter()
            .filter_map(|(direction, room)| room.map(|room| (*direction, room)))
    }

    /// Find the next unexplored direction in this room and mark it as explored
    ///
    /// If the room is the checkpoint it will return None so as not to explore the objective early
    ///
    /// ### Arguments
    /// * `new_index` - The index to identify the destination room with
    ///
    /// ### Returns
    /// * Some of a direction if one was unexplored, it will be marked as explored with `new_index`
    /// * None if no direction was unexplored, the index has not been used then
    pub fn explore_new_direction(&mut self, new_index: usize) -> Option<Direction> {
        if self.checkpoint {
            None
        } else {
            let found = self.first_direction_with_unknown_room()?;
            self.directions.insert(found, Some(new_index));
            Some(found)
        }
    }

    /// If this room is the checkpoint room, return the direction to get to the objective
    pub fn objective_direction(&self) -> Option<Direction> {
        if self.checkpoint {
            self.first_direction_with_unknown_room()
        } else {
            None
        }
    }

    /// Find the items fields in a room
    fn lookup_items(text: &str) -> Option<Vec<Item>> {
        text.split_terminator("\n\n").find_map(Item::read_block)
    }

    /// The first direction in this room that we don't have a known room ID for
    fn first_direction_with_unknown_room(&self) -> Option<Direction> {
        self.directions.iter().find_map(|(direction, room)| {
            if room.is_none() {
                Some(*direction)
            } else {
                None
            }
        })
    }
}

/// A direction to go from a room
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    /// Read a block of output from the RPG to see what directions are available
    pub fn read_block(s: &str) -> Option<Vec<Self>> {
        s.strip_prefix("Doors here lead:\n").and_then(|s| {
            s.lines()
                .map(|line| match line.trim_start_matches('-').trim() {
                    "north" => Some(Direction::North),
                    "south" => Some(Direction::South),
                    "east" => Some(Direction::East),
                    "west" => Some(Direction::West),
                    other => {
                        eprintln!("Unknown direction ! {}", other);
                        None
                    }
                })
                .collect()
        })
    }

    /// The string to pass as input when going in a specific direction
    pub fn input(self) -> &'static str {
        match self {
            Direction::North => "north\n",
            Direction::South => "south\n",
            Direction::East => "east\n",
            Direction::West => "west\n",
        }
    }

    /// The reversed direction from this one
    pub fn reversed(self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
        }
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let message = match self {
            Direction::North => "north",
            Direction::South => "south",
            Direction::East => "east",
            Direction::West => "west",
        };
        message.fmt(f)
    }
}

/// An item to pick up in a room
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Item(String);

impl Item {
    /// The list of items that if picked up will trigger some sort of game over
    /// They are thus excluded from the auto pick up functionality
    const DANGEROUS_ITEMS: [&'static str; 5] = [
        "giant electromagnet",
        "molten lava",
        "escape pod",
        "infinite loop",
        "photons",
    ];

    /// Read a block of output to see what items are available
    pub fn read_block(s: &str) -> Option<Vec<Self>> {
        s.strip_prefix("Items here:\n").map(|s| {
            s.lines()
                .filter_map(|line| {
                    let item = line.trim_start_matches('-').trim();
                    // Prevent dangerous items from being picked up by the script
                    if Self::DANGEROUS_ITEMS.contains(&item) {
                        None
                    } else {
                        Some(Self(item.to_owned()))
                    }
                })
                .collect()
        })
    }

    /// The string to pass as input when wanting to pick up an item
    pub fn take(&self) -> String {
        format!("take {}\n", self.0)
    }

    /// The string to pass as input when wanting to drop an item
    pub fn drop(&self) -> String {
        format!("drop {}\n", self.0)
    }
}

impl Display for Item {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        self.0.fmt(f)
    }
}
