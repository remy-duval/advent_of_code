use std::cmp::Reverse;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str::FromStr;

use color_eyre::eyre::{eyre, Report, Result, WrapErr};
use itertools::Itertools;

use commons::parse::sep_by_empty_lines;

pub type Int = u64;

/// The state of the battle
#[derive(Debug, Clone)]
pub struct Battle {
    immune_system: Vec<Units>,
    infection: Vec<Units>,
    elements: Vec<String>,
}

impl Battle {
    /// The maximum amount of hit points the infection unit have
    pub fn infection_max_hp(&self) -> Int {
        self.infection
            .iter()
            .map(|u| u.hit_points)
            .max()
            .unwrap_or_default()
    }

    /// Return a copy of this battle with the immune system unit damage increased
    pub fn with_boosted_immune_system(&self, amount: Int) -> Self {
        let mut cloned = self.clone();
        cloned
            .immune_system
            .iter_mut()
            .for_each(|unit| unit.damage += amount);

        cloned
    }

    /// True if the immune system has won the battle
    pub fn has_immune_system_won(&self) -> bool {
        self.infection.is_empty()
    }

    /// True if the infection has won the battle
    pub fn has_infection_won(&self) -> bool {
        self.immune_system.is_empty()
    }

    /// Execute a full fight, returning how much units the winning side has left after that
    pub fn fight(&mut self) -> Int {
        loop {
            // Phase 1: assign the targets for each group
            Self::assign_targets(&mut self.immune_system, &self.infection);
            Self::assign_targets(&mut self.infection, &self.immune_system);

            // Phase 2: units attack in order of initiative their designated targets
            let mut is_not_tie = false;
            self.foreach_group(|attacker, defenders| {
                if let Some(target) = attacker.target.and_then(|i| defenders.get_mut(i)) {
                    let dealt_losses = target.take_damage_from(attacker);
                    is_not_tie = is_not_tie || dealt_losses;
                }
            });

            // Phase 3: remove empty units
            self.remove_dead_units();

            // Phase 4: check if any side has won, if not continue the battle
            if self.has_infection_won() {
                return self.infection.iter().map(|u| u.count).sum();
            } else if self.has_immune_system_won() {
                return self.immune_system.iter().map(|u| u.count).sum();
            } else if !is_not_tie {
                // No damage happened this turn, this means the fight is an eternal tie
                // 0 is a good signal value (to avoid introducing an Option) for that
                return 0;
            }
        }
    }

    /// Do an action for each group in order of initiative
    ///
    /// ### Arguments
    /// * `action` - A closure that takes two arguments
    ///   * A reference to the unit taking its turn
    ///   * A mutable reference to the units in the opposing army
    fn foreach_group(&mut self, mut action: impl FnMut(&Units, &mut [Units])) {
        fn initiative(units: &[Units], idx: usize) -> Option<Int> {
            units.get(idx).map(|unit| unit.initiative)
        }

        let mut imm: usize = 0;
        let mut inf: usize = 0;
        loop {
            match (
                initiative(&self.immune_system, imm),
                initiative(&self.infection, inf),
            ) {
                (Some(a), Some(b)) if a < b => {
                    action(&self.infection[inf], &mut self.immune_system);
                    inf += 1;
                }
                (Some(_), _) => {
                    action(&self.immune_system[imm], &mut self.infection);
                    imm += 1;
                }
                (_, Some(_)) => {
                    action(&self.infection[inf], &mut self.immune_system);
                    inf += 1;
                }
                (None, None) => break,
            }
        }
    }

    /// Remove all units that no longer have any combatants from the battle
    fn remove_dead_units(&mut self) {
        self.immune_system.retain(|unit| unit.count > 0);
        self.infection.retain(|unit| unit.count > 0);
    }

    /// Assign a target for each of the attacking units
    fn assign_targets(attacking: &mut [Units], defending: &[Units]) {
        let mut targets = defending.iter().enumerate().collect_vec();
        let mut attacking = attacking.iter_mut().collect_vec();
        attacking.sort_unstable_by_key(|u| Reverse((u.effective_power(), u.initiative)));

        for attacker in attacking {
            let target = targets
                .iter()
                .max_by(|(_, a), (_, b)| {
                    attacker
                        .actual_damage_to(a)
                        .cmp(&attacker.actual_damage_to(b))
                        .then_with(|| {
                            a.effective_power()
                                .cmp(&b.effective_power())
                                .then_with(|| a.initiative.cmp(&b.initiative))
                        })
                })
                .and_then(|(i, unit)| {
                    if attacker.actual_damage_to(*unit) > 0 {
                        Some(*i)
                    } else {
                        None
                    }
                });

            attacker.target = target;
            if let Some(found) = target {
                targets.retain(|(i, _)| *i != found);
            }
        }
    }
}

/// A pack of units in either the infection or immune system
#[derive(Debug, Clone)]
struct Units {
    count: Int,
    /// The number of hit points of each unit
    hit_points: Int,
    /// The initiative of each unit attack
    initiative: Int,
    /// The damage of each unit attack
    damage: Int,
    /// The element of each unit attack
    element: usize,
    /// The units weaknesses (as indexes to the elements vector)
    weaknesses: Vec<usize>,
    /// The units immunities (as indexes to the elements vector)
    immunities: Vec<usize>,
    /// The units next target in the battle
    target: Option<usize>,
}

impl Units {
    /// Check if this group is weak to an element
    fn is_weak_to(&self, element: usize) -> bool {
        self.weaknesses.iter().any(|&elt| elt == element)
    }

    /// Check if this group is immune from an element
    fn is_immune_from(&self, element: usize) -> bool {
        self.immunities.iter().any(|&elt| elt == element)
    }

    /// The damage that this unit group will do when attacking another group
    fn effective_power(&self) -> Int {
        self.count * self.damage
    }

    /// Compute the actual amount of damage this group could deal to a target group
    fn actual_damage_to(&self, target: &Self) -> Int {
        if target.is_immune_from(self.element) {
            0
        } else if target.is_weak_to(self.element) {
            self.effective_power() * 2
        } else {
            self.effective_power()
        }
    }

    /// Take damage from an attacker, killing the most unit possible, leaving the rest at full HP
    ///
    /// ### Arguments
    /// * `attacker` - The unit that attacks this ones
    ///
    /// ### Returns
    /// * `true` if the attacker killed anything
    /// * `false` if it didn't
    fn take_damage_from(&mut self, attacker: &Self) -> bool {
        let damage = attacker.actual_damage_to(self);
        let losses = damage / self.hit_points;
        self.count = self.count.saturating_sub(losses);
        losses > 0
    }
}

impl Display for Battle {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        fn elem(index: usize, elements: &[String]) -> &str {
            elements
                .get(index)
                .map_or_else(|| "UNKNOWN", |str| str.as_str())
        }

        let write_units = |f: &mut Formatter<'_>, unit: &Units| {
            writeln!(
                f,
                "{} x (HP: {}, INI: {}, ATK: ({} {}), WEAK: [{}], IMM: [{}])",
                unit.count,
                unit.hit_points,
                unit.initiative,
                unit.damage,
                elem(unit.element, &self.elements),
                unit.weaknesses
                    .iter()
                    .map(|&e| elem(e, &self.elements))
                    .join(", "),
                unit.immunities
                    .iter()
                    .map(|&e| elem(e, &self.elements))
                    .join(", "),
            )
        };

        writeln!(f, "Immune System:")?;
        self.immune_system
            .iter()
            .try_for_each(|unit| write_units(f, unit))?;
        writeln!(f, "Infection:")?;
        self.infection
            .iter()
            .try_for_each(|unit| write_units(f, unit))
    }
}

impl FromStr for Battle {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self> {
        let mut immune_system = Vec::with_capacity(10);
        let mut infection = Vec::with_capacity(10);
        let mut elements = Vec::with_capacity(10);

        for section in sep_by_empty_lines(s) {
            if let Some(imm) = section.strip_prefix("Immune System:") {
                for line in imm.trim().lines().filter(|l| !l.is_empty()) {
                    immune_system.push(parse_unit(line, &mut elements)?);
                }
            } else if let Some(inf) = section.strip_prefix("Infection:") {
                for line in inf.trim().lines().filter(|l| !l.is_empty()) {
                    infection.push(parse_unit(line, &mut elements)?);
                }
            } else {
                return Err(eyre!("Unknown section for the battle data: {}", section));
            }
        }

        // Sort the armies in decreasing order of initiative (their turn order)
        immune_system.sort_unstable_by(|a, b| b.initiative.cmp(&a.initiative));
        infection.sort_unstable_by(|a, b| b.initiative.cmp(&a.initiative));

        Ok(Self {
            immune_system,
            infection,
            elements,
        })
    }
}

/// Parse a unit, filling the elements array with newly discovered elements
fn parse_unit(line: &str, elements: &mut Vec<String>) -> Result<Units> {
    fn inner(s: &str, elements: &mut Vec<String>) -> Option<Result<Units>> {
        let (count, s) = s.splitn(2, ' ').collect_tuple::<(_, _)>()?;
        let s = s.strip_prefix("units each with")?.trim_start();
        let (hp, s) = s.splitn(2, ' ').collect_tuple::<(_, _)>()?;
        let s = s.trim_start().strip_prefix("hit points")?.trim_start();
        let (resistances, s) = match s.strip_prefix('(') {
            Some(resistances) => {
                let (raw, s) = resistances.splitn(2, ')').collect_tuple::<(_, _)>()?;
                (Some(raw), s)
            }
            None => (None, s),
        };

        let s = s
            .trim_start()
            .strip_prefix("with an attack that does")?
            .trim_start();
        let (damage, element, s) = s.splitn(3, ' ').collect_tuple::<(_, _, _)>()?;
        let initiative = s.trim_start().strip_prefix("damage at initiative")?.trim();

        let count = match parse_number(count) {
            Ok(value) => value,
            Err(err) => return Some(Err(err)),
        };

        let hit_points = match parse_number(hp) {
            Ok(value) => value,
            Err(err) => return Some(Err(err)),
        };

        let damage = match parse_number(damage) {
            Ok(value) => value,
            Err(err) => return Some(Err(err)),
        };

        let element = get_or_insert_element(element, elements);

        let initiative = match parse_number(initiative) {
            Ok(value) => value,
            Err(err) => return Some(Err(err)),
        };

        let (weaknesses, immunities) = match resistances {
            Some(res) => parse_resistances(res, elements),
            None => (Vec::new(), Vec::new()),
        };

        Some(Ok(Units {
            count,
            hit_points,
            initiative,
            damage,
            element,
            weaknesses,
            immunities,
            target: None,
        }))
    }

    inner(line, elements).unwrap_or_else(|| Err(eyre!("Bad format for a units line: {}", line)))
}

/// Parse a number for a unit, wrapping any error in a ParseError
fn parse_number(number: &str) -> Result<Int> {
    number
        .trim()
        .parse()
        .wrap_err_with(|| format!("Could not parse an integer '{0}'", number))
}

/// Find the index of the given element in the element vector
/// Insert it in the element vector if it does not exist yet
fn get_or_insert_element(new: &str, elements: &mut Vec<String>) -> usize {
    elements
        .iter()
        .position(|s| s.as_str() == new)
        .unwrap_or_else(|| {
            elements.push(new.to_owned());
            elements.len() - 1
        })
}

/// Parse the resistances part of a unit
fn parse_resistances(raw: &str, elements: &mut Vec<String>) -> (Vec<usize>, Vec<usize>) {
    fn extend_index(element: &str, indexes: &mut Vec<usize>, elements: &mut Vec<String>) {
        let element = element.trim();
        let index = get_or_insert_element(element, elements);
        if !indexes.iter().any(|&i| i == index) {
            indexes.push(index);
        }
    }

    let mut weaknesses = Vec::new();
    let mut immunities = Vec::new();
    for part in raw.split(';').map(|s| s.trim()) {
        if let Some(weak) = part.strip_prefix("weak to") {
            weak.split(',')
                .for_each(|element| extend_index(element, &mut weaknesses, elements));
        } else if let Some(imm) = part.strip_prefix("immune to") {
            imm.split(',')
                .for_each(|element| extend_index(element, &mut immunities, elements));
        }
    }

    (weaknesses, immunities)
}
