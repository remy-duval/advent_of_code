use std::str::FromStr;

use commons::eyre::{eyre, Report, Result, WrapErr};
use itertools::Itertools;

/// (Month, Day)
pub type Day = (u8, u8);

/// The events, sorted
#[non_exhaustive] // To prevent Events from being constructed outside `new`
#[derive(Debug, Clone)]
pub struct Events {
    pub events: Vec<TimedEvent>,
}

impl Events {
    /// Build the events, sorting them in their timestamp order
    pub fn new(mut events: Vec<TimedEvent>) -> Self {
        events.sort_unstable_by(|a, b| a.timestamp.cmp(&b.timestamp));
        Self { events }
    }
}

/// An event and its timestamp
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TimedEvent {
    pub timestamp: Timestamp,
    pub event: Event,
}

/// A timestamp in an event
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Timestamp {
    /// (Month, Day) of the timestamp
    pub day: Day,
    /// Hours of the timestamp
    pub hour: u8,
    /// Minutes of the timestamp
    pub minutes: u8,
}

/// An event in a guard shift
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Event {
    Sleep,
    WakeUp,
    Change(u16),
}

impl FromStr for Events {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self> {
        Ok(Self::new(s.lines().map(|line| line.parse()).try_collect()?))
    }
}

impl FromStr for TimedEvent {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self> {
        let (timestamp, event) = s
            .splitn(2, ']')
            .collect_tuple::<(_, _)>()
            .ok_or_else(|| eyre!("Bad format for: [<TIMESTAMP>] <EVENT>, got {}", s))?;

        Ok(Self {
            timestamp: timestamp
                .parse()
                .wrap_err("Could not parse the timestamp")?,
            event: event.parse().wrap_err("Could not parse the event")?,
        })
    }
}

impl FromStr for Timestamp {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self> {
        fn parse_u8(s: &str) -> Result<u8> {
            s.parse()
                .wrap_err_with(|| format!("Could not parse number {}", s))
        }

        fn split_two(s: &str, sep: char) -> Result<(&str, &str)> {
            s.trim()
                .splitn(2, sep)
                .collect_tuple::<(_, _)>()
                .ok_or_else(|| {
                    eyre!(
                        "Bad format for a timestamp <YEAR>-<MONTH>-<DAY> <HOUR>:<MINUTES>, got {}",
                        s
                    )
                })
        }

        let s = s.trim().trim_start_matches("[1518-").trim_end_matches(']');
        let (day, time) = split_two(s, ' ')?;
        let (hour, minutes) = split_two(time, ':')?;
        let (month, day) = split_two(day, '-')?;

        Ok(Self {
            day: (parse_u8(month)?, parse_u8(day)?),
            hour: parse_u8(hour)?,
            minutes: parse_u8(minutes)?,
        })
    }
}

impl FromStr for Event {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self> {
        fn parse_guard(guard: &str) -> Option<Result<u16>> {
            let id = guard
                .strip_prefix("Guard #")?
                .strip_suffix(" begins shift")?
                .parse()
                .wrap_err_with(|| format!("Could not parse guard ID {}", guard));

            Some(id)
        }

        match s.trim() {
            "falls asleep" => Ok(Event::Sleep),
            "wakes up" => Ok(Event::WakeUp),
            other => parse_guard(other).map_or_else(
                || Err(eyre!("Unknown event {}", other)),
                |result| result.map(Event::Change),
            ),
        }
    }
}
