use std::collections::VecDeque;

use color_eyre::eyre::{eyre, Result};
use itertools::Itertools;

use commons::Problem;
use commons::CLEAR_COMMAND;

use super::int_code::{IntCodeInput, Processor};

mod ship;

pub struct Day;

impl Problem for Day {
    type Input = IntCodeInput;
    const TITLE: &'static str = "Day 25: Cryostasis";

    fn solve(data: Self::Input) -> Result<()> {
        let mut line = String::new();
        loop {
            println!("Mode ? (options are manual / script)");
            line.clear();
            std::io::stdin().read_line(&mut line)?;
            line = line.to_ascii_lowercase().trim().into();
            match line.as_str() {
                "manual" => {
                    play_manually(&data.data);
                    return Ok(());
                }
                "script" => {
                    let timer = std::time::Instant::now();
                    return if let Some(code) = auto_play(&data.data) {
                        let elapsed = timer.elapsed();
                        println!("The key code is '{}'", code);
                        println!("Finding the key code took {}s", elapsed.as_secs_f64());
                        Ok(())
                    } else {
                        Err(eyre!("Could not find the key code at the end"))
                    };
                }
                _ => println!("Unrecognized : {}", line),
            }
        }
    }
}

/// The current state of the game to allow deciding the next action to take
struct GameState {
    /// The current path we are following
    path: VecDeque<ship::Direction>,
    /// The last move that was done by the robot
    last_direction: ship::Direction,
    /// The current map of the ship
    ship: ship::Ship,
    /// All the output of the ascii program that has not been processed
    buffer: String,
    /// An inventory of all items we have picked up
    inventory: Vec<ship::Item>,
    /// A flag to indicate the current path ends with a new room
    exploring: bool,
    /// A flag to indicate the current path ends with the checkpoint room
    to_checkpoint: bool,
}

/// You can play the game by typing in the console.
fn play_manually(memory: &[i64]) {
    fn normalize_new_line(line: &mut String) {
        if line.ends_with("\r\n") {
            line.truncate(line.len() - 2);
            line.push('\n');
        }
    }

    println!("{}", CLEAR_COMMAND);
    let mut processor: Processor = memory.into();
    processor.run_with_ascii_callbacks(
        0,
        |_| {
            let mut line: String = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            normalize_new_line(&mut line);
            Some(line)
        },
        |_, line| {
            print!("{}", line);
            Ok(())
        },
    );
}

/// Play the game until we reach the end
///
/// This is overly complex but it should work
fn auto_play(memory: &[i64]) -> Option<u64> {
    let (_, (_, last_state)) = Processor::new(memory).run_with_ascii_callbacks(
        (
            false,
            GameState {
                path: VecDeque::new(),
                last_direction: ship::Direction::North,
                ship: Default::default(),
                buffer: String::new(),
                inventory: Vec::new(),
                exploring: false,
                to_checkpoint: false,
            },
        ),
        main_loop,
        |(_, state), line| {
            state.buffer.push_str(line);
            Ok(())
        },
    );

    if let Some(message) = last_state.buffer.split("\n\n").last() {
        println!("{}", message);
    }

    last_state
        .buffer
        .split_whitespace()
        .filter_map(|word| word.parse::<u64>().ok())
        .last()
}

/// The main loop of the ascii robot
fn main_loop((initialized, state): &mut (bool, GameState)) -> Option<String> {
    if *initialized {
        follow_path(state)
            .or_else(|| explore_room(state))
            .or_else(|| deciding_next_direction(state))
            .or_else(|| bruteforce_lock(state))
    } else {
        *initialized = true;
        state.ship.explore_first_room(&state.buffer);
        state.buffer.clear();
        Some(String::from("inv\n"))
    }
}

/// If there is a path to follow stored in the state, follow it
fn follow_path(state: &mut GameState) -> Option<String> {
    if let Some(next) = state.path.pop_front() {
        println!("Following path {}", next);
        state.buffer.clear(); // We don't really need the output until we reach the end
        state.last_direction = next;
        Some(next.input().to_owned())
    } else {
        None
    }
}

/// If we have arrived in a new room, register it in the map and pick its items
/// In any case try to pick up any remaining item in the current room
fn explore_room(state: &mut GameState) -> Option<String> {
    if state.exploring {
        let from = state.last_direction.reversed();
        println!("Arrived in new room from {}, exploring it", from);
        state.ship.on_explored_room(&state.buffer, from);
        state.exploring = false;
        state.buffer.clear();
    }

    if state.ship.has_items_to_pick() {
        let items = state.ship.take_items();
        state.inventory.extend_from_slice(&items);
        let command = items.iter().map(|item| item.take()).join("");
        println!("Pick up all items in the room:\n{}\n", command);

        Some(command)
    } else {
        None
    }
}

/// If we have no action to do anymore, then decide the next room to explore
fn deciding_next_direction(state: &mut GameState) -> Option<String> {
    if !state.to_checkpoint {
        state.buffer.clear();
        println!("Finding next action (either visit unexplored or go to checkpoint)");
        if let Some((path, room)) = state.ship.find_unexplored() {
            println!("Decision: explore new room {} through {:?}\n", room, path);
            state.ship.current = room; // Set the current room to the one we arrive at
            state.path = VecDeque::from(path);
            state.exploring = true;
        } else {
            let (path, room) = state.ship.go_to_checkpoint()?;
            println!("Decision: go to checkpoint {} through {:?}\n", room, path);
            state.ship.current = room; // Set the current room to the one we arrive at
            state.path = VecDeque::from(path);
            state.to_checkpoint = true;
        }

        follow_path(state)
    } else {
        None
    }
}

/// If we are in the checkpoint room, try all combinations one by one
fn bruteforce_lock(state: &mut GameState) -> Option<String> {
    if state.to_checkpoint {
        println!("Arrived at checkpoint");
        println!("Inventory contains:\n{}", state.inventory.iter().join(", "));
        println!("Now trying all combinations of those items");
        let objective = state.ship.rooms[state.ship.current].objective_direction()?;
        let all_combinations = (1..state.inventory.len())
            .flat_map(|i| state.inventory.iter().combinations(i))
            .map(|items| {
                format!(
                    "{}{}{}",
                    items.iter().map(|item| item.take()).join(""),
                    objective.input(),
                    items.iter().map(|item| item.drop()).join("")
                )
            })
            .join("");

        Some(all_combinations)
    } else {
        None
    }
}

#[cfg(test)]
mod tests;
