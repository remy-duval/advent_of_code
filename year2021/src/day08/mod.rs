use commons::parse::LineSep;
use commons::{err, Report, Result, WrapErr};

pub const TITLE: &str = "Day 8: Seven Segment Search";

pub fn run(raw: String) -> Result<()> {
    let data = parse(&raw)?;
    println!("1. {} simple digits", first_part(&data.data));
    println!("2. {} total output", second_part(&data.data)?);
    Ok(())
}

fn parse(s: &str) -> Result<LineSep<Outputs>> {
    s.parse()
}

const DIGITS: usize = 10;
const WIRES: usize = 7;
const OUTPUT: usize = 4;
static NUMBERS: [WireBitSet; DIGITS] = [
    WireBitSet::from_array([true, true, true, false, true, true, true]),
    WireBitSet::from_array([false, false, true, false, false, true, false]),
    WireBitSet::from_array([true, false, true, true, true, false, true]),
    WireBitSet::from_array([true, false, true, true, false, true, true]),
    WireBitSet::from_array([false, true, true, true, false, true, false]),
    WireBitSet::from_array([true, true, false, true, false, true, true]),
    WireBitSet::from_array([true, true, false, true, true, true, true]),
    WireBitSet::from_array([true, false, true, false, false, true, false]),
    WireBitSet::from_array([true, true, true, true, true, true, true]),
    WireBitSet::from_array([true, true, true, true, false, true, true]),
];

/// Resolve the unambiguous digits and count their occurrences in the output values
fn first_part(outputs: &[Outputs]) -> usize {
    outputs.iter().fold(0, |global, o| {
        // Find the unknown elements that are the only ones with a given length
        // Then count the number of times they appear in the output list
        o.simple_digits().fold(global, |acc, simple| {
            acc + o.outputs.iter().filter(|&o| o == simple).count()
        })
    })
}

/// Resolve all digits and compute the total of output values
fn second_part(outputs: &[Outputs]) -> Result<usize> {
    outputs.iter().try_fold(0, |total, o| {
        let digits = resolve(o)?;
        let local = o.outputs.iter().try_fold(0, |acc, next| -> Result<usize> {
            let number = digits
                .iter()
                .position(|x| x == next)
                .ok_or_else(|| err!("Missing {:?} in {:?}", next, digits))?;

            Ok(acc * 10 + number)
        })?;

        Ok(total + local)
    })
}

/// Try to resolve all the digits of the output:
/// - Get the unambiguous digits to reduce the possibility set
/// - Try to build the whole set of digits from that
/// - Try all branches in a depth-first-search when there all multiple choices
fn resolve(output: &Outputs) -> Result<[WireBitSet; DIGITS]> {
    let mut stack = Vec::with_capacity(DIGITS);
    stack.push(Possibilities::new(output));
    while let Some(mut next) = stack.pop() {
        // This will build the full digit set if there are no ambiguities
        // We still need to check that this is the wanted digit set (but scrambled) at the end
        if let Some(built) = next.try_build() {
            if output.outputs.iter().all(|o| built.contains(o)) {
                return Ok(built);
            }
        }

        // The current possibilities are ambiguous, add both branches to the depth-first stack
        if let Some((first, rest)) = next.split_possibilities() {
            stack.push(rest);
            stack.push(first);
        }
    }

    Err(err!("Exhausted all possibilities for {:?}", output))
}

/// The puzzle input
#[derive(Debug)]
struct Outputs {
    /// The unknown digits, represented by their wires set
    unknown: [WireBitSet; DIGITS],
    /// The output we need to read, containing some of the elements in `unknown`
    outputs: [WireBitSet; OUTPUT],
}

/// The combination of wires that form a number on a display
#[derive(Copy, Clone, Eq, PartialEq, Default)]
struct WireBitSet {
    value: u8,
}

/// Link between the scrambled wires and the correct wires
/// Either Ok(known) or Err(set of possibilities) for each wire
#[derive(Clone)]
struct Possibilities([Result<u8, WireBitSet>; WIRES]);

impl Outputs {
    /// Find the simple digits (1, 4, 7, and 8 normally) in the the unknown set
    /// We can do this because they have a unique number of wires among the set each
    fn simple_digits(&self) -> impl Iterator<Item = &WireBitSet> {
        self.unknown
            .iter()
            .filter(|u| self.unknown.iter().filter(|v| u.len() == v.len()).count() == 1)
    }
}

impl Possibilities {
    /// The possibility set from only the unambiguous digits
    fn new(out: &Outputs) -> Self {
        let mut result = [NUMBERS[8]; WIRES]; // The 8th number has all the wires
        out.simple_digits().for_each(|actual| {
            NUMBERS.iter().for_each(|original| {
                if original.len() == actual.len() {
                    for (i, possible) in (0..WIRES).zip(result.iter_mut()) {
                        if original.contains(i as u8) {
                            possible.value &= actual.value;
                        } else {
                            possible.value &= u8::MAX - actual.value;
                        };
                    }
                }
            })
        });

        Self(result.map(Err))
    }

    /// Split this possibility set into two on the first branching path
    /// - The first set will have chosen the first choice on this branch (other branches untouched)
    /// - The second set will have all the rest of the choices on this branch
    ///
    /// ### Returns
    /// None if there is no longer any remaining branching path.
    /// In that case we either have a successful set or a dead end
    fn split_possibilities(&self) -> Option<(Self, Self)> {
        // Find the first branch if there is one
        // Also short-circuits if there is a branch with 0 choices (dead-end)
        let (i, next) = self.0.iter().enumerate().find_map(|(i, x)| match x {
            Err(wires) if wires.len() != 1 => Some((i, wires)),
            _ => None,
        })?;
        let (first, rest) = next.first_and_rest()?;
        let mut one = self.clone();
        let mut two = self.clone();
        one.0[i] = Err(first);
        two.0[i] = Err(rest);
        Some((one, two))
    }

    /// Reconstruct the digits from this possibility set
    /// Will only return Some if there is no ambiguity in the decisions.
    /// If it returns, None, We should split the possibility set and try again
    fn try_build(&mut self) -> Option<[WireBitSet; DIGITS]> {
        let mut result = [WireBitSet::default(); DIGITS];
        let ok = (0..DIGITS).all(|i| {
            // Start from the good number, and scramble it with the known bindings
            let original = &NUMBERS[i];
            let scrambled = &mut result[i];
            (0..WIRES).filter(|j| original.contains(*j as u8)).all(|j| {
                // For each wire binding, we either have:
                // - A single choice that was already fixed before: just use it
                // - A set of one choice: choose it and remove it from the other choice sets
                // - A set of multiple choices: short-circuit
                match &self.0[j] {
                    Ok(chosen) => {
                        let add = 1 << *chosen;
                        scrambled.value |= add;
                        true
                    }
                    Err(possible) => {
                        if let Some(f) = possible.first().filter(|_| possible.len() == 1) {
                            // Fix this wire binding for later
                            // To do that, set the wire binding to Ok
                            // And remove this binding from the other wires choices
                            let add = 1 << f;
                            let remove = u8::MAX - add;
                            scrambled.value |= add;
                            self.0[j] = Ok(f);
                            self.0.iter_mut().for_each(|x| match x {
                                Ok(_) => (),
                                Err(w) => w.value &= remove,
                            });
                            true
                        } else {
                            false
                        }
                    }
                }
            })
        });

        if ok {
            Some(result)
        } else {
            None
        }
    }
}

impl WireBitSet {
    /// Build this bit set from an array of booleans for each wire
    const fn from_array(wires: [bool; WIRES]) -> Self {
        let mut value = 0;
        let mut i = 0;
        while i < wires.len() {
            if wires[i] {
                let mask = 1 << i;
                value |= mask;
            }
            i += 1;
        }

        Self { value }
    }

    /// Check if this wire is in this set
    const fn contains(&self, num: u8) -> bool {
        (self.value & 1 << num) != 0
    }

    /// Get the first wire in this set (the minimum one)
    const fn first(&self) -> Option<u8> {
        match self.value.trailing_zeros() {
            n if n as usize >= WIRES => None,
            n => Some(n as u8),
        }
    }

    /// Get the number of wires in this set
    const fn len(&self) -> u32 {
        self.value.count_ones()
    }

    /// Split this into the first wire + the rest of the wires (None if no wires)
    const fn first_and_rest(&self) -> Option<(Self, Self)> {
        if let Some(n) = self.first() {
            let mask = 1 << n;
            let first = Self {
                value: self.value & mask,
            };
            let rest = Self {
                value: self.value ^ mask,
            };
            Some((first, rest))
        } else {
            None
        }
    }
}

impl std::fmt::Debug for WireBitSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use std::fmt::Write;
        (0..(WIRES as u8)).try_for_each(|i| {
            if self.contains(i) {
                f.write_char(char::from(b'a' + i))
            } else {
                Ok(())
            }
        })
    }
}

impl std::str::FromStr for WireBitSet {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self> {
        let mut value = 0;
        for c in s.chars() {
            value |= match c {
                'a' => 1,
                'b' => 1 << 1,
                'c' => 1 << 2,
                'd' => 1 << 3,
                'e' => 1 << 4,
                'f' => 1 << 5,
                'g' => 1 << 6,
                _ => return Err(err!("Bad number wire {c}")),
            };
        }

        Ok(Self { value })
    }
}

impl std::str::FromStr for Outputs {
    type Err = Report;

    #[inline]
    fn from_str(s: &str) -> Result<Self> {
        fn parse_into(s: &str, into: &mut [WireBitSet]) -> Result<()> {
            for (wires, dest) in s.split_whitespace().zip(into.iter_mut()) {
                *dest = wires.parse().wrap_err_with(|| format!("In {s}"))?;
            }

            Ok(())
        }

        let (s, o) = s.split_once('|').ok_or_else(|| err!("Missing '|' sep"))?;
        let mut unknown: [WireBitSet; DIGITS] = Default::default();
        parse_into(s, &mut unknown)?;
        let mut outputs: [WireBitSet; OUTPUT] = Default::default();
        parse_into(o, &mut outputs)?;

        Ok(Self { unknown, outputs })
    }
}

#[cfg(test)]
mod tests;
