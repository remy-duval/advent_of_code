use commons::error::Result;
use commons::WrapErr;

pub const TITLE: &str = "Day 6: Wait For It";

pub fn run(raw: String) -> Result<()> {
    let data = parse(raw.into())?;
    let first = first_part(&data);
    println!("1. The product of ways to beat each boat race is {first}");
    let second = second_part(&data);
    println!("2. The number of ways to beat the complete boat race is {second}");

    Ok(())
}

#[derive(Debug)]
struct Race {
    time: i64,
    distance: i64,
}

fn first_part(races: &[Race]) -> u64 {
    races.iter().map(count_solutions).product()
}

fn second_part(races: &[Race]) -> u64 {
    fn concat_numbers(num: impl Iterator<Item = i64>) -> i64 {
        num.fold(0, |acc, n| {
            let mut result = acc;
            let mut rest = n;
            while rest > 0 {
                result *= 10;
                rest /= 10;
            }
            result + n
        })
    }
    let time = concat_numbers(races.iter().map(|r| r.time));
    let distance = concat_numbers(races.iter().map(|r| r.distance));
    count_solutions(&Race { time, distance })
}

fn count_solutions(race: &Race) -> u64 {
    // The formula is WantedDistance = ButtonPress * (Time - ButtonPress)
    // Which becomes ButtonPress² - Time * ButtonPress + WantedDistance = 0
    // Of the form: AX² + BX + C = 0 with A = 1, B = -Time, C = WantedDistance
    // => Delta = Time² - 4 * WantedDistance
    // => If Delta is > 0, 2 Solutions at: (Time +- root(Delta)) / 2
    // => The Distance will be higher than WantedDistance between the 2 solutions (strictly)
    let delta = race.time * race.time - 4 * race.distance;
    if delta > 0 {
        let diff = (delta as f64).sqrt();
        let x1 = (race.time as f64 - diff) / 2.0;
        let x2 = (race.time as f64 + diff) / 2.0;
        (x2.ceil() - x1.floor() - 1.0) as u64
    } else {
        0
    }
}

fn parse(s: std::borrow::Cow<'static, str>) -> Result<Vec<Race>> {
    let times = s
        .lines()
        .find_map(|l| l.trim().strip_prefix("Time:"))
        .wrap_err("missing Time line")?
        .split_ascii_whitespace();
    let distances = s
        .lines()
        .find_map(|l| l.trim().strip_prefix("Distance:"))
        .wrap_err("missing Distance line")?
        .split_ascii_whitespace();

    times
        .zip(distances)
        .map(|(time, distance)| -> Result<Race> {
            Ok(Race {
                time: time.parse()?,
                distance: distance.parse()?,
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../examples/06.txt");
    const MAIN: &str = include_str!("../inputs/06.txt");

    #[test]
    fn first_part_example() {
        let data = parse(EXAMPLE.into()).unwrap();
        assert_eq!(first_part(&data), 288);
    }

    #[test]
    fn first_part_main() {
        let data = parse(MAIN.into()).unwrap();
        assert_eq!(first_part(&data), 6_209_190);
    }

    #[test]
    fn second_part_example() {
        let data = parse(EXAMPLE.into()).unwrap();
        assert_eq!(second_part(&data), 71_503);
    }

    #[test]
    fn second_part_main() {
        let data = parse(MAIN.into()).unwrap();
        assert_eq!(second_part(&data), 28_545_089);
    }
}
