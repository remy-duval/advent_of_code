use std::str::FromStr;

use itertools::Itertools;

/// (Month, Day)
pub type Day = (u8, u8);

/// The events, sorted
#[derive(Debug, Clone)]
pub struct Events {
    pub events: Vec<TimedEvent>,
    /// To prevent Events from being constructed outside `new`
    private: (),
}

impl Events {
    /// Build the events, sorting them in their timestamp order
    pub fn new(mut events: Vec<TimedEvent>) -> Self {
        events.sort_unstable_by(|a, b| a.timestamp.cmp(&b.timestamp));
        Self {
            events,
            private: (),
        }
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

/// An error that happened while parsing a timed event
#[derive(Debug, thiserror::Error)]
pub enum TimedEventParseError {
    #[error("Could not parse the event: {0}")]
    Event(#[from] EventParseError),
    #[error("Could not parse the timestamp: {0}")]
    Timestamp(#[from] TimestampParseError),
    #[error("Bad format for: [<TIMESTAMP>] <EVENT>, got {0}")]
    BadFormat(Box<str>),
}

/// An error that happened while parsing a timestamp
#[derive(Debug, thiserror::Error)]
pub enum TimestampParseError {
    #[error("Could not parse number {0} ({1})")]
    NumberParseError(Box<str>, #[source] std::num::ParseIntError),
    #[error("Bad format for a timestamp <YEAR>-<MONTH>-<DAY> <HOUR>:<MINUTES>, got {0}")]
    BadFormat(Box<str>),
}

/// An error that happened while parsing an event
#[derive(Debug, thiserror::Error)]
pub enum EventParseError {
    #[error("Unknown event {0}")]
    UnknownEvent(Box<str>),
    #[error("Could not guard ID {0} ({1})")]
    IdParse(Box<str>, #[source] std::num::ParseIntError),
}

impl FromStr for Events {
    type Err = TimedEventParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(s.lines().map(|line| line.parse()).try_collect()?))
    }
}

impl FromStr for TimedEvent {
    type Err = TimedEventParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (timestamp, event) = s
            .splitn(2, ']')
            .collect_tuple::<(_, _)>()
            .ok_or_else(|| TimedEventParseError::BadFormat(s.into()))?;

        Ok(Self {
            timestamp: timestamp.parse()?,
            event: event.parse()?,
        })
    }
}

impl FromStr for Timestamp {
    type Err = TimestampParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_u8(s: &str) -> Result<u8, TimestampParseError> {
            s.parse()
                .map_err(|err| TimestampParseError::NumberParseError(s.into(), err))
        }

        fn split_two(s: &str, sep: char) -> Result<(&str, &str), TimestampParseError> {
            s.trim()
                .splitn(2, sep)
                .collect_tuple::<(_, _)>()
                .ok_or_else(|| TimestampParseError::BadFormat(s.into()))
        }

        let s = s.trim_start_matches('[').trim_end_matches(']');
        let (day, time) = split_two(s, ' ')?;
        let (hour, minutes) = split_two(time, ':')?;
        let (_, month, day) = day
            .trim()
            .splitn(3, '-')
            .collect_tuple::<(_, _, _)>()
            .ok_or_else(|| TimestampParseError::BadFormat(s.into()))?;

        Ok(Self {
            day: (parse_u8(month)?, parse_u8(day)?),
            hour: parse_u8(hour)?,
            minutes: parse_u8(minutes)?,
        })
    }
}

impl FromStr for Event {
    type Err = EventParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_guard(guard: &str) -> Option<Result<u16, EventParseError>> {
            let id = guard
                .strip_prefix("Guard #")?
                .strip_suffix(" begins shift")?
                .parse()
                .map_err(|err| EventParseError::IdParse(guard.into(), err));

            Some(id)
        }

        match s.trim() {
            "falls asleep" => Ok(Event::Sleep),
            "wakes up" => Ok(Event::WakeUp),
            other => parse_guard(other).map_or_else(
                || Err(EventParseError::UnknownEvent(other.into())),
                |result| result.map(Event::Change),
            ),
        }
    }
}
