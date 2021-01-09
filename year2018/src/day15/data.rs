use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::fmt::{Display, Formatter, Result as FmtResult, Write};
use std::str::FromStr;

use hashbrown::{HashMap, HashSet};

use commons::grid::Direction;

use crate::points::Point;

/// The current state of the fight
#[derive(Debug, Clone)]
pub struct Fight {
    tiles: HashMap<Point, Tile>,
    units: BTreeMap<Point, u8>,
    dead_elf: bool,
}

impl Fight {
    /// Compute the next rounds until the end of the fight
    pub fn first_part(&mut self) -> (usize, usize) {
        let mut rounds = 0;
        while !self.next_round(3, 3) {
            rounds += 1;
        }

        self.outcome(rounds)
    }

    /// Find the first outcome where the elves win
    pub fn second_part(&self) -> Option<(usize, usize)> {
        (4..u8::MAX).find_map(|elves_attack| {
            let mut fight = self.clone();
            let mut rounds = 0;
            // Short-circuit as soon as an elf die (we don't care about the outcome then)
            while !fight.next_round(elves_attack, 3) && !fight.dead_elf {
                rounds += 1;
            }

            if !fight.dead_elf {
                Some(fight.outcome(rounds))
            } else {
                None
            }
        })
    }

    /// Compute the outcome of the fight (completed rounds, remaining health)
    pub fn outcome(&self, rounds: usize) -> (usize, usize) {
        let remaining_health: usize = self
            .tiles
            .values()
            .map(|tile| match tile {
                Tile::Unit { hp, .. } => *hp as usize,
                Tile::Wall => 0,
            })
            .sum();

        (rounds, remaining_health)
    }

    /// Compute the next round of the fight
    /// False if the fight is not finished, True if it is
    pub fn next_round(&mut self, elves_attack: u8, goblins_attack: u8) -> bool {
        self.units.clone().into_iter().any(|(p, expected_id)| {
            if let Some(tile) = self.tiles.get(&p).copied() {
                match tile {
                    Tile::Unit { elf, id, .. } if id == expected_id => {
                        if elf {
                            self.turn(p, tile, elves_attack, Tile::is_goblin, Tile::goblin_health)
                        } else {
                            self.turn(p, tile, goblins_attack, Tile::is_elf, Tile::elf_health)
                        }
                    }
                    _ => false,
                }
            } else {
                false
            }
        })
    }

    /// Play a unit turn, returning true if the unit decided the fight was finished
    fn turn<G, T>(&mut self, point: Point, tile: Tile, dmg: u8, goal: G, target: T) -> bool
    where
        G: Fn(Tile) -> bool,
        T: Fn(Tile) -> Option<u8>,
    {
        if let Some(next) = next_move(point, &self.tiles, goal) {
            self.move_unit(point, next, tile);
            self.fight(next, dmg, target);
            false
        } else if self.is_fight_done() {
            true
        } else {
            self.fight(point, dmg, target);
            false
        }
    }

    /// Move a `tile` from `from` to `to`
    fn move_unit(&mut self, from: Point, to: Point, tile: Tile) {
        self.tiles.remove(&from);
        self.units.remove(&from);
        self.tiles.insert(to, tile);
        self.units.insert(to, tile.id());
    }

    /// True if one of the army has been decimated
    fn is_fight_done(&self) -> bool {
        let mut seen_elf = false;
        let mut seen_goblin = false;
        for tile in self.units.iter().filter_map(|(p, _)| self.tiles.get(p)) {
            match tile {
                Tile::Unit { elf, .. } => {
                    if *elf {
                        seen_elf = true;
                    } else {
                        seen_goblin = true;
                    }
                    if seen_elf && seen_goblin {
                        return false;
                    }
                }
                Tile::Wall => {}
            }
        }

        true
    }

    /// Attack one of the unit around `from`
    /// Non target should be None with `target_health`, else Some(hp)
    /// The attacked unit is chosen based on the minimum returned health by `target_health`
    /// If the attacked unit dies, it is removed from the field
    ///
    /// ### Returns
    /// True if the unit attacked at all
    fn fight(&mut self, from: Point, dmg: u8, target_health: impl Fn(Tile) -> Option<u8>) {
        let target = around(from)
            .iter()
            .filter_map(|point| {
                let tile = *self.tiles.get(point)?;
                let health = target_health(tile)?; // If not target this is None
                Some(((health, *point), tile)) // The minimum implementation will use reading order
            })
            .min_by_key(|(cmp, _)| *cmp);

        if let Some(((_, point), tile)) = target {
            if let Some(updated) = tile.damage(dmg) {
                // Unit is alive, update its HP
                self.tiles.insert(point, updated);
            } else {
                // Unit is dead
                self.tiles.remove(&point);
                self.units.remove(&point);
                if tile.is_elf() {
                    self.dead_elf = true;
                }
            }
        }
    }
}

impl Display for Fight {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let max = self.tiles.keys().fold((0, 0), |(max_x, max_y), point| {
            (max_x.max(point.x), max_y.max(point.y))
        });

        (0..(max.1 + 1)).try_for_each(|y| {
            (0..(max.0 + 1)).try_for_each(|x| {
                f.write_char(match self.tiles.get(&Point::new(x, y)) {
                    Some(Tile::Unit { elf: false, .. }) => 'G',
                    Some(Tile::Unit { elf: true, .. }) => 'E',
                    Some(Tile::Wall) => '#',
                    None => ' ',
                })
            })?;

            f.write_char('\n')
        })
    }
}

impl FromStr for Fight {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut next_id = 1;
        let mut tiles = HashMap::new();
        let mut units = BTreeMap::new();

        s.lines().enumerate().for_each(|(y, line)| {
            line.chars().enumerate().for_each(|(x, c)| {
                let point = Point::new(x as i64, y as i64);
                match c {
                    'G' | 'E' => {
                        let tile = Tile::Unit {
                            elf: c == 'E',
                            id: next_id,
                            hp: 200,
                        };
                        tiles.insert(point, tile);
                        units.insert(point, next_id);
                        next_id += 1;
                    }
                    '#' => {
                        tiles.insert(point, Tile::Wall);
                    }
                    _ => {}
                };
            });
        });

        Ok(Self {
            tiles,
            units,
            dead_elf: false,
        })
    }
}

/// A tile in the fight
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Tile {
    Unit { elf: bool, id: u8, hp: u8 },
    Wall,
}

impl Tile {
    /// The ID of this tile. It is 0 for walls, unique for unit
    fn id(self) -> u8 {
        match self {
            Tile::Unit { id, .. } => id,
            _ => 0,
        }
    }

    /// True if this tile is a goblin
    fn is_goblin(self) -> bool {
        matches!(self, Tile::Unit { elf: false, .. })
    }

    /// Get the health of the tile if it is a goblin
    fn goblin_health(self) -> Option<u8> {
        match self {
            Tile::Unit { elf: false, hp, .. } => Some(hp),
            _ => None,
        }
    }

    /// True if this tile is an elf
    fn is_elf(self) -> bool {
        matches!(self, Tile::Unit { elf: true, .. })
    }

    /// Get the health of the tile if it is an elf
    fn elf_health(self) -> Option<u8> {
        match self {
            Tile::Unit { elf: true, hp, .. } => Some(hp),
            _ => None,
        }
    }

    /// Damage a unit (if it can be damaged). If the unit dies from it, returns None.
    fn damage(self, dmg: u8) -> Option<Self> {
        match self {
            Tile::Unit { elf, id, hp } => {
                if hp <= dmg {
                    None
                } else {
                    Some(Self::Unit {
                        elf,
                        id,
                        hp: hp - dmg,
                    })
                }
            }
            Tile::Wall => Some(Tile::Wall),
        }
    }
}

/// All the points around a center (in reading order)
fn around(point: Point) -> [Point; 4] {
    [
        Point::new(point.x, point.y - 1),
        Point::new(point.x - 1, point.y),
        Point::new(point.x + 1, point.y),
        Point::new(point.x, point.y + 1),
    ]
}

/// Find the next move for a unit
///
/// ### Arguments
/// * `start` - The starting tile
/// * `map` - The map of the occupied tiles
/// * `is_goal` - A check to see if an occupied tile is one we want to go to
///
/// ### Returns
/// Some(point) the next move to commit to
/// None if there is no available move for the unit
fn next_move(
    from: Point,
    map: &HashMap<Point, Tile>,
    is_goal: impl Fn(Tile) -> bool,
) -> Option<Point> {
    if around(from)
        .iter()
        .any(|point| map.get(point).map_or(false, |t| is_goal(*t)))
    {
        // A destination is directly around the start tile, no move to do
        return None;
    }

    let mut path: Option<Point> = None;
    let mut shortest: Option<usize> = None;
    let mut destination: Option<Point> = None;

    // queue of the paths as (length of the path, destination tile, starting move)
    let mut queue = VecDeque::with_capacity(10);
    let mut visited = HashSet::with_capacity(100);
    queue.push_back((0, from, None));
    visited.insert(from);

    while let Some((length, current, initial)) = queue.pop_front() {
        // If the current path is longer than the best path found, discard it
        if shortest.map_or(false, |len| length > len) {
            continue;
        }

        around(current).iter().for_each(|&point| {
            // Avoid backtracking
            if !visited.insert(point) {
                return;
            }

            let initial = initial.unwrap_or(point);
            if let Some(tile) = map.get(&point) {
                // Check if we arrive near a target
                if is_goal(*tile) {
                    // Set the best path to the minimum of current and new
                    let shortest = shortest.get_or_insert(length);
                    let destination = destination.get_or_insert(point);
                    let path = path.get_or_insert(initial);
                    if length < *shortest || (length == *shortest && point < *destination) {
                        *shortest = length;
                        *destination = point;
                        *path = initial;
                    }
                }
            } else {
                // Append the new path to the queue
                queue.push_back((length + 1, point, Some(initial)));
            }
        });
    }

    path
}
