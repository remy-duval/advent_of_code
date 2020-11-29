use std::collections::HashSet;
use std::io;

use itertools::Itertools;

use aoc::generator::data_from_cli;
use aoc::grid::Direction;
use aoc::int_code::{parse_int_code, Processor};

const TITLE: &str = "Day 25: Cryostasis";
const DATA: &str = include_str!("../resources/day25.txt");
const DANGEROUS_ITEMS: [&str; 5] = [
    "giant electromagnet",
    "molten lava",
    "escape pod",
    "infinite loop",
    "photons",
];
const AUTO_PATH: [&str; 19] = [
    "south\n", "south\n", "south\n", "south\n", "north\n", "north\n", "west\n", "north\n",
    "north\n", "south\n", "south\n", "east\n", "north\n", "west\n", "north\n", "south\n", "west\n",
    "west\n", "west\n",
];

fn main() {
    let data = data_from_cli(TITLE, DATA);
    println!("{}", TITLE);
    let memory = parse_int_code(&data).expect("Parse Int code error !");

    loop {
        println!("Mode ? (options are manual / script)");
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        line = line.to_ascii_lowercase().trim_end_matches('\n').into();
        match line.as_str() {
            "manual" => return play_manually(&memory),
            "script" => return auto_play(&memory),
            _ => println!("Unrecognized : {}", line),
        }
    }
}

/// You can play the game by typing in the console.
/// This is pretty fun for something in IntCode, really nice job from the AOC designer
fn play_manually(memory: &[i64]) {
    println!("{}", aoc::CLEAR_COMMAND);
    let mut processor: Processor = memory.into();
    processor.run_with_ascii_callbacks(
        0,
        |_| {
            let mut line: String = String::new();
            io::stdin().read_line(&mut line).ok()?;
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
                    "west\n",
                    items.iter().map(|item| format!("take {}\n", item)).join("")
                )
            })
            .join("")
    }

    println!("{}", aoc::CLEAR_COMMAND);
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
