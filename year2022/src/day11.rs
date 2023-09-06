use std::str::FromStr;

use itertools::Itertools;

use commons::error::Result;
use commons::parse::SepByEmptyLine;
use commons::{err, Report, WrapErr};

pub const TITLE: &str = "Day 11: Monkey in the Middle";

pub fn run(raw: String) -> Result<()> {
    let data = parse(raw.into())?;
    let first = first_part(&data);
    println!("1. Monkey business after 20 rounds is {first}");
    let second = second_part(&data);
    println!("2. Monkey business after 10000 rounds is {second}");

    Ok(())
}

type Item = u64;
type Count = u64;

fn first_part(monkeys: &[Monkey]) -> Count {
    simulate(monkeys, 20, |level| level / 3)
}

fn second_part(monkeys: &[Monkey]) -> Count {
    // This should conserve divisibility (because all the divisor here are primes)
    let modulo_by: Item = monkeys.iter().map(|m| m.divisible).product();
    simulate(monkeys, 10000, |level| level % modulo_by)
}

fn simulate(monkeys: &[Monkey], rounds: usize, update_level: impl Fn(Item) -> Item) -> Count {
    let mut state: Vec<_> = monkeys.iter().map(|m| m.items.clone()).collect();
    let mut inspected: Vec<Count> = vec![0; monkeys.len()];
    let mut next_items = Vec::new();
    for _ in 0..rounds {
        for monkey in monkeys {
            std::mem::swap(&mut next_items, &mut state[monkey.id as usize]);
            inspected[monkey.id as usize] += next_items.len() as Count;
            for item in next_items.drain(..) {
                let item = match monkey.operation {
                    Operation::Add(a, b) => a.value(item) + b.value(item),
                    Operation::Mul(a, b) => a.value(item) * b.value(item),
                };

                let item = update_level(item);
                let destination = if item % monkey.divisible == 0 {
                    monkey.if_true
                } else {
                    monkey.if_false
                };

                if let Some(destination) = state.get_mut(destination as usize) {
                    destination.push(item);
                }
            }
        }
    }

    let (mut first_max, mut second_max) = (0, 0);
    for count in inspected {
        if count > first_max {
            second_max = first_max;
            first_max = count;
        } else if count > second_max {
            second_max = count;
        }
    }

    first_max * second_max
}

#[derive(Clone, Debug)]
struct Monkey {
    id: u8,
    items: Vec<Item>,
    operation: Operation,
    divisible: Item,
    if_true: u8,
    if_false: u8,
}

#[derive(Clone, Debug)]
enum Operation {
    Add(Operand, Operand),
    Mul(Operand, Operand),
}

#[derive(Copy, Clone, Debug)]
enum Operand {
    Old,
    Constant(Item),
}

impl Operand {
    fn value(&self, old: Item) -> Item {
        match self {
            Operand::Old => old,
            Operand::Constant(c) => *c,
        }
    }
}

impl FromStr for Monkey {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self> {
        fn prefixed<'a>(s: &'a str, prefix: &'static str) -> Result<&'a str> {
            match s.strip_prefix(prefix) {
                Some(rest) => Ok(rest.trim()),
                None => Err(err!("expected '{s}' to start with {prefix}")),
            }
        }

        s.lines()
            .map(str::trim)
            .collect_tuple::<(_, _, _, _, _, _)>()
            .wrap_err("expected a block of 6 lines")
            .and_then(|(id, items, operation, divisible, if_true, if_false)| {
                let id = prefixed(id, "Monkey")?;
                let items = prefixed(items, "Starting items:")?;
                let operation = prefixed(operation, "Operation: new =")?;
                let divisible = prefixed(divisible, "Test: divisible by")?;
                let if_true = prefixed(if_true, "If true: throw to monkey")?;
                let if_false = prefixed(if_false, "If false: throw to monkey")?;

                let id: u8 = id
                    .strip_suffix(':')
                    .unwrap_or(id)
                    .parse()
                    .wrap_err("monkey id")?;
                let items = items
                    .split(',')
                    .map(|i| i.trim().parse())
                    .collect::<Result<Vec<Item>, _>>()
                    .wrap_err("items list")?;

                let operation = operation
                    .splitn(3, ' ')
                    .collect_tuple::<(_, _, _)>()
                    .wrap_err("operation does not have 3 parts")
                    .and_then(|(first, operator, second)| {
                        let first = match first {
                            "old" => Operand::Old,
                            c => Operand::Constant(c.parse().wrap_err("first operand")?),
                        };
                        let second = match second {
                            "old" => Operand::Old,
                            c => Operand::Constant(c.parse().wrap_err("second operand")?),
                        };
                        match operator {
                            "+" => Ok(Operation::Add(first, second)),
                            "*" => Ok(Operation::Mul(first, second)),
                            _ => Err(err!("operator {operator}")),
                        }
                    })
                    .wrap_err("operation")?;

                let divisible: Item = divisible.parse().wrap_err("divisible by test")?;
                let if_true: u8 = if_true.parse().wrap_err("if test true")?;
                let if_false: u8 = if_false.parse().wrap_err("if test false")?;
                Ok(Self {
                    id,
                    items,
                    operation,
                    divisible,
                    if_true,
                    if_false,
                })
            })
            .wrap_err_with(|| format!("for '{s}'"))
    }
}

fn parse(s: std::borrow::Cow<'static, str>) -> Result<Vec<Monkey>> {
    let split: SepByEmptyLine<Monkey> = s.parse()?;
    Ok(split.data)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../examples/11.txt");
    const MAIN: &str = include_str!("../inputs/11.txt");

    #[test]
    fn first_part_example() {
        let data = parse(EXAMPLE.into()).unwrap();
        assert_eq!(first_part(&data), 10_605);
    }

    #[test]
    fn first_part_main() {
        let data = parse(MAIN.into()).unwrap();
        assert_eq!(first_part(&data), 111_210);
    }

    #[test]
    fn second_part_example() {
        let data = parse(EXAMPLE.into()).unwrap();
        assert_eq!(second_part(&data), 2_713_310_158);
    }

    #[test]
    fn second_part_main() {
        let data = parse(MAIN.into()).unwrap();
        assert_eq!(second_part(&data), 15_447_387_620);
    }
}
