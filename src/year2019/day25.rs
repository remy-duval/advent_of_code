use std::io;

use hashbrown::HashSet;
use itertools::Itertools;

use crate::commons::CLEAR_COMMAND;
use crate::commons::grid::Direction;
use crate::Problem;

use super::int_code::{IntCodeInput, Processor};

/// The path to take to gather all items
/// Note that the take command are not present here, since they are done automatically
/// Just be sure to add dangerous items to the [DANGEROUS_ITEMS](DANGEROUS_ITEMS) constant
/// Else you will automatically pick them up and game over
const AUTO_PATH: [&str; 23] = [
    // Visit the south branch, taking its items
    "south\n", "east\n", // Then return to Start
    "west\n", "north\n", // Visit the west branch taking its items
    "west\n", "west\n", "west\n", "north\n", "west\n", "south\n", // Return to Start
    "north\n", "east\n", "south\n", "east\n", "east\n", "east\n",
    // Visit the east branch, taking its items
    "east\n", "north\n", "west\n", "north\n", "west\n", "west\n",
    "south\n", // We are now right before the Weight test room (go south for that)
];

/// The list of items that if picked up will trigger some sort of game over
/// They are thus excluded from the auto pick up functionality
const DANGEROUS_ITEMS: [&str; 5] = [
    "giant electromagnet",
    "molten lava",
    "escape pod",
    "infinite loop",
    "photons",
];

/// The direction that the pressure door is from the last room
/// The 'script' mode will try to take any possible item combination before going this way
/// Over and over until it succeeds
const TRY_COMBINATION: &str = "south\n";

pub struct Day;

impl Problem for Day {
    type Input = IntCodeInput;
    type Err = io::Error;
    const TITLE: &'static str = "Day 25: Cryostasis";

    fn solve(data: Self::Input) -> Result<(), Self::Err> {
        let mut line = String::new();
        loop {
            println!("Mode ? (options are manual / script)");
            println!("Note that 'script' mode is only valid for my original input");
            line.clear();
            io::stdin().read_line(&mut line)?;
            line = line.to_ascii_lowercase().trim().into();
            match line.as_str() {
                "manual" => return Ok(play_manually(&data.data)),
                "script" => return Ok(auto_play(&data.data)),
                _ => println!("Unrecognized : {}", line),
            }
        }
    }
}

fn normalize_new_line(line: &mut String) {
    if line.ends_with("\r\n") {
        line.truncate(line.len() - 2);
        line.push('\n');
    }
}

/// You can play the game by typing in the console.
fn play_manually(memory: &[i64]) {
    println!("{}", CLEAR_COMMAND);
    let mut processor: Processor = memory.into();
    processor.run_with_ascii_callbacks(
        0,
        |_| {
            let mut line: String = String::new();
            io::stdin().read_line(&mut line).ok()?;
            normalize_new_line(&mut line);
            Some(line)
        },
        |_, line| {
            print!("{}", line);
            Ok(())
        },
    );
}

/// This will execute the script to collect most items and try to solve the puzzle
/// /!\ This is an implementation specific to my input, it will not work with other.
/// TODO ? Add some maze solving to generalize the initial path part to any input ?
fn auto_play(memory: &[i64]) {
    // To gain access, we simply try all combinations of items
    // The command this returns will chain all tries into one
    // It will try to drop some items, try the door and then grab them again.
    fn brute_force_command(inventory: &[String]) -> String {
        (1..inventory.len())
            .flat_map(|i| inventory.iter().combinations(i))
            .map(|items| {
                format!(
                    "{}{}{}",
                    items.iter().map(|item| format!("drop {}\n", item)).join(""),
                    TRY_COMBINATION,
                    items.iter().map(|item| format!("take {}\n", item)).join("")
                )
            })
            .join("")
    }

    println!("{}", CLEAR_COMMAND);
    let mut processor: Processor = memory.into();
    let mut inventory: Vec<String> = Vec::new();
    let mut initial_path = AUTO_PATH.iter().map(|&x| x.to_string());
    processor.run_with_ascii_callbacks(
        Room::default(),
        |room| {
            // This will try to collect any non collected item that are not marked as dangerous
            if let Some(take) = room
                .available_items
                .iter()
                .find(|item| !DANGEROUS_ITEMS.contains(&item.as_str()) && !inventory.contains(item))
            {
                inventory.push(take.clone());
                let command = format!("take {}\n", take);
                print!("{}", command);
                Some(command)
                // If there are no items to collect in the room, we follow the rest of the path
            } else if let Some(next) = initial_path.next() {
                Some(next)
                // Then when we arrive in the last room, time to brute-force the item weight puzzle !
            } else {
                Some(brute_force_command(&inventory))
            }
        },
        |room, line| {
            room.parse_information(line);
            print!("{}", line);
            Ok(())
        },
    );
}

#[derive(Debug)]
struct Room {
    name: String,
    available_directions: HashSet<Direction>,
    available_items: Vec<String>,
}

impl Room {
    /// Parse new data for the Room from the given String.
    pub fn parse_information(&mut self, data: &str) {
        data.lines()
            .filter(|line| !line.is_empty())
            .for_each(|line| {
                if line.starts_with('=') {
                    // This is the start of a room description, we should empty this room
                    self.name.clear();
                    self.name.push_str(line.replace("=", "").trim());
                    self.available_directions.clear();
                    self.available_items.clear();
                } else if line.starts_with('-') {
                    if let Some(direction) = Self::read_direction(line) {
                        self.available_directions.insert(direction);
                    } else {
                        self.available_items
                            .push(line.trim_start_matches("- ").into());
                    }
                }
            });
    }

    /// Read direction description into a Direction
    fn read_direction(string: &str) -> Option<Direction> {
        match string.trim_start_matches("- ") {
            "north" => Some(Direction::North),
            "south" => Some(Direction::South),
            "east" => Some(Direction::East),
            "west" => Some(Direction::West),
            _ => None,
        }
    }
}

impl Default for Room {
    fn default() -> Self {
        Self {
            name: "".into(),
            available_directions: HashSet::new(),
            available_items: Vec::new(),
        }
    }
}
