use itertools::Itertools;
use std::collections::HashMap;

use commons::Result;

mod events;

pub const TITLE: &str = "Day 4: Repose Record";

pub fn run(raw: String) -> Result<()> {
    let data = parse(&raw)?;
    let schedule = Schedule::new(&data.events);
    let (guard, sleepiest) = schedule.first_strategy().unwrap();
    println!(
        "Strategy 1: The sleepiest guard is {} at 00:{:02} for a result of {}",
        guard,
        sleepiest,
        guard * sleepiest
    );

    let (guard, sleepiest) = schedule.second_strategy().unwrap();
    println!(
        "Strategy 2: The sleepiest guard is {} at 00:{:02} for a result of {}",
        guard,
        sleepiest,
        guard * sleepiest
    );

    Ok(())
}

fn parse(s: &str) -> Result<events::Events> {
    s.parse()
}

#[derive(Debug)]
struct Schedule {
    guards: HashMap<u16, [u16; 60]>,
}

impl Schedule {
    /// Create the schedule of the guards
    fn new(events: &[events::TimedEvent]) -> Self {
        assert!(!events.is_empty());
        let mut guards: HashMap<u16, [u16; 60]> = HashMap::new();
        let mut guard = 0;
        let mut is_asleep = false;
        let mut sleep = |guard: u16, is_asleep: bool, duration: std::ops::Range<usize>| {
            let time = guards.entry(guard).or_insert_with(|| [0; 60]);
            if is_asleep {
                time[duration].iter_mut().for_each(|m| *m += 1);
            }
        };

        for (_, events) in &events.iter().group_by(|e| e.timestamp.day) {
            let mut now = 0;
            let mut next_guard = 0;
            events.into_iter().for_each(|event| {
                if event.timestamp.hour == 0 && event.timestamp.minutes > 0 {
                    let next = event.timestamp.minutes as usize;
                    sleep(guard, is_asleep, now..next);
                    now = next;
                }

                is_asleep = match event.event {
                    events::Event::Sleep => true,
                    events::Event::WakeUp => false,
                    events::Event::Change(id) => {
                        next_guard = id;
                        if event.timestamp.hour == 0 {
                            guard = id;
                        }
                        false
                    }
                };
            });

            // Complete the remaining schedule of the guard
            if now < 60 {
                sleep(guard, is_asleep, now..60);
            }
            guard = next_guard;
        }

        Self { guards }
    }

    /// Find (sleepiest guard, sleepiest minute)
    /// With strategy 1: the guard that is asleep the most minutes in total
    fn first_strategy(&self) -> Option<(usize, usize)> {
        let (&guard, minutes) = self
            .guards
            .iter()
            .max_by_key(|(_, m)| m.iter().sum::<u16>())?;
        let sleepiest = minutes.iter().position_max()?;
        Some((guard as usize, sleepiest))
    }

    /// Find (sleepiest guard, sleepiest minute)
    /// With strategy 2: the guard that is asleep at the same minute the most
    fn second_strategy(&self) -> Option<(usize, usize)> {
        let (&guard, minutes) = self
            .guards
            .iter()
            .max_by_key(|(_, m)| m.iter().max().unwrap_or(&0))?;
        let sleepiest = minutes.iter().position_max()?;
        Some((guard as usize, sleepiest))
    }
}

#[cfg(test)]
mod tests;
