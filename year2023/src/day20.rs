use std::collections::{BTreeMap, VecDeque};

use commons::error::Result;
use commons::WrapErr;

pub const TITLE: &str = "Day 20: Pulse Propagation";

pub fn run(raw: String) -> Result<()> {
    let data = parse(raw.into())?;
    let (low, high, first) = first_part(&data);
    println!("1. After 1000 presses: {low} low pulses * {high} high pulses = {first}");
    let steps = second_part(&data)?;
    println!("2. A low pulse will be sent to 'rx' after {steps} steps");

    Ok(())
}

#[derive(Debug, Copy, Clone)]
enum Pulse {
    High,
    Low,
}

#[derive(Debug, Copy, Clone)]
enum Kind {
    Broadcaster,
    FlipFlop,
    Conjunction,
}

#[derive(Debug)]
struct Module {
    kind: Kind,
    inputs: Vec<usize>,
    outputs: Vec<usize>,
}

#[derive(Debug)]
struct Modules {
    start: usize,
    end: Option<usize>,
    modules: Vec<Module>,
}

#[derive(Debug, Copy, Clone)]
struct State(u32);

impl State {
    fn receive(mut self, module: &Module, (pulse, input): (Pulse, usize)) -> (Self, Option<Pulse>) {
        fn bit(module: &Module, input: usize) -> u32 {
            match module.inputs.iter().position(|i| *i == input) {
                Some(pos) => 1 << pos,
                None => 0,
            }
        }
        match module.kind {
            Kind::Broadcaster => (self, Some(pulse)),
            Kind::FlipFlop => match pulse {
                Pulse::High => (self, None),
                Pulse::Low if self.0 == 0 => (Self(1), Some(Pulse::High)),
                Pulse::Low => (Self(0), Some(Pulse::Low)),
            },
            Kind::Conjunction => match pulse {
                Pulse::High => {
                    self.0 |= bit(module, input);
                    let pulse = if self.0.count_ones() == module.inputs.len() as u32 {
                        Pulse::Low
                    } else {
                        Pulse::High
                    };
                    (self, Some(pulse))
                }
                Pulse::Low => {
                    self.0 &= !bit(module, input);
                    (self, Some(Pulse::High))
                }
            },
        }
    }
}

fn first_part(modules: &Modules) -> (u64, u64, u64) {
    let mut low_pulses = 0;
    let mut high_pulses = 0;
    let mut queue: VecDeque<(Pulse, usize, usize)> = VecDeque::new();
    let mut state = vec![State(0); modules.modules.len()];

    (0..1000).for_each(|_| {
        queue.push_back((Pulse::Low, modules.start, modules.start));
        while let Some((pulse, from, to)) = queue.pop_front() {
            match pulse {
                Pulse::High => high_pulses += 1,
                Pulse::Low => low_pulses += 1,
            };

            if let Some((module, state)) = modules.modules.get(to).zip(state.get_mut(to)) {
                let update = state.receive(module, (pulse, from));
                *state = update.0;
                if let Some(next) = update.1 {
                    queue.extend(module.outputs.iter().map(|&out| (next, to, out)));
                }
            }
        }
    });

    (low_pulses, high_pulses, low_pulses * high_pulses)
}

fn second_part(modules: &Modules) -> Result<u64> {
    find_low(modules, modules.end.wrap_err("no receiver module")?)
        .wrap_err("receiver module can never receive a low signal")
}

fn find_low(mods: &Modules, index: usize) -> Option<u64> {
    let m = mods.modules.get(index)?;
    match m.kind {
        Kind::Broadcaster => Some(1),
        Kind::FlipFlop => m.inputs.iter().filter_map(|&i| find_low(mods, i)).min(),
        Kind::Conjunction => find_conjunction_high_period(mods, m, index),
    }
}

fn find_conjunction_high_period(modules: &Modules, dest: &Module, index: usize) -> Option<u64> {
    let mut queue: VecDeque<(Pulse, usize, usize)> = VecDeque::new();
    let mut state = vec![State(0); modules.modules.len()];
    let mut step = 0;
    let mut periods = vec![0; dest.inputs.len()];
    loop {
        step += 1;
        queue.push_back((Pulse::Low, modules.start, modules.start));
        while let Some((pulse, from, to)) = queue.pop_front() {
            if to == index && matches!(pulse, Pulse::High) {
                if let Some(i) = dest.inputs.iter().position(|&e| e == from) {
                    if let Some(period @ 0) = periods.get_mut(i) {
                        *period = step;
                        if periods.iter().all(|s| *s != 0) {
                            return periods.into_iter().reduce(commons::math::lcm);
                        }
                    }
                }
            }
            if let Some((module, state)) = modules.modules.get(to).zip(state.get_mut(to)) {
                let update = state.receive(module, (pulse, from));
                *state = update.0;
                if let Some(next) = update.1 {
                    queue.extend(module.outputs.iter().map(|&out| (next, to, out)));
                }
            }
        }
    }
}

fn parse(s: std::borrow::Cow<'static, str>) -> Result<Modules> {
    let mut names: BTreeMap<&str, usize> = BTreeMap::new();
    let mut modules: Vec<Module> = vec![];
    for line in s.lines() {
        let (module, outputs) = line
            .trim()
            .split_once("->")
            .wrap_err_with(|| format!("missing '->' in {line:?}"))?;
        let module = module.trim();
        let (name, kind) = if let Some(n) = module.strip_prefix('%') {
            (n, Kind::FlipFlop)
        } else if let Some(n) = module.strip_prefix('&') {
            (n, Kind::Conjunction)
        } else {
            (module, Kind::Broadcaster)
        };

        let current = *names.entry(name).or_insert_with(|| {
            modules.push(Module {
                kind,
                inputs: vec![],
                outputs: vec![],
            });
            modules.len() - 1
        });
        modules[current].kind = kind;
        modules[current].outputs = outputs
            .split(',')
            .map(|output| {
                let out = *names.entry(output.trim()).or_insert_with(|| {
                    modules.push(Module {
                        kind: Kind::FlipFlop,
                        inputs: vec![],
                        outputs: vec![],
                    });
                    modules.len() - 1
                });
                modules[out].inputs.push(current);
                out
            })
            .collect();
    }

    let start = names
        .get("broadcaster")
        .copied()
        .wrap_err("missing broadcaster module")?;
    let end = names.get("rx").copied();
    Ok(Modules {
        start,
        end,
        modules,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = include_str!("../examples/20_1.txt");
    const EXAMPLE_2: &str = include_str!("../examples/20_2.txt");
    const MAIN: &str = include_str!("../inputs/20.txt");

    #[test]
    fn first_part_example_1() {
        let data = parse(EXAMPLE_1.into()).unwrap();
        assert_eq!(first_part(&data), (8_000, 4_000, 32_000_000));
    }

    #[test]
    fn first_part_example_2() {
        let data = parse(EXAMPLE_2.into()).unwrap();
        assert_eq!(first_part(&data), (4_250, 2_750, 11_687_500));
    }

    #[test]
    fn first_part_main() {
        let data = parse(MAIN.into()).unwrap();
        assert_eq!(first_part(&data), (18_266, 44_777, 817_896_682));
    }

    #[test]
    fn second_part_main() {
        let data = parse(MAIN.into()).unwrap();
        assert_eq!(second_part(&data).unwrap(), 250_924_073_918_341);
    }
}
