use commons::bail;
use commons::error::Result;

pub const TITLE: &str = "Day 2: Cube Conundrum";

pub fn run(raw: String) -> Result<()> {
    let data = parse(raw.into())?;
    let first = first_part(&data);
    println!("1. The sum of IDs of possible games is {first}");
    let second = second_part(&data);
    println!("2. The sum of minimum sets powers is {second}");

    Ok(())
}

#[derive(Debug)]
struct Game {
    id: usize,
    sets: Vec<CubeSet>,
}

#[derive(Debug, Default)]
struct CubeSet {
    blue: u8,
    red: u8,
    green: u8,
}

fn first_part(games: &[Game]) -> usize {
    games
        .iter()
        .filter(|game| {
            game.sets
                .iter()
                .all(|set| set.blue <= 14 && set.red <= 12 && set.green <= 13)
        })
        .map(|game| game.id)
        .sum()
}

fn second_part(games: &[Game]) -> usize {
    games
        .iter()
        .map(|game| {
            let mut min_set = CubeSet::default();
            game.sets.iter().for_each(|set| {
                min_set.blue = min_set.blue.max(set.blue);
                min_set.red = min_set.red.max(set.red);
                min_set.green = min_set.green.max(set.green);
            });
            min_set.blue as usize * min_set.red as usize * min_set.green as usize
        })
        .sum()
}

fn parse(s: std::borrow::Cow<'static, str>) -> Result<Vec<Game>> {
    s.lines()
        .filter_map(|line| {
            line.strip_prefix("Game ")
                .and_then(|line| line.split_once(':'))
                .map(|(id, line)| {
                    line.split(';')
                        .map(|cube_set| -> Result<CubeSet> {
                            let mut set = CubeSet::default();
                            for cubes in cube_set.split(',') {
                                match cubes.trim().split_once(' ') {
                                    Some((red, "red")) => set.red += red.parse::<u8>()?,
                                    Some((green, "green")) => set.green += green.parse::<u8>()?,
                                    Some((blue, "blue")) => set.blue += blue.parse::<u8>()?,
                                    _ => bail!("bad line {id}: {line:?} at {cubes:?}"),
                                }
                            }

                            Ok(set)
                        })
                        .collect::<Result<Vec<CubeSet>>>()
                        .and_then(|sets| {
                            Ok(Game {
                                id: id.parse()?,
                                sets,
                            })
                        })
                })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../examples/02.txt");
    const MAIN: &str = include_str!("../inputs/02.txt");

    #[test]
    fn first_part_example() {
        let data = parse(EXAMPLE.into()).unwrap();
        assert_eq!(first_part(&data), 8);
    }

    #[test]
    fn first_part_main() {
        let data = parse(MAIN.into()).unwrap();
        assert_eq!(first_part(&data), 1_867);
    }

    #[test]
    fn second_part_example() {
        let data = parse(EXAMPLE.into()).unwrap();
        assert_eq!(second_part(&data), 2_286);
    }

    #[test]
    fn second_part_main() {
        let data = parse(MAIN.into()).unwrap();
        assert_eq!(second_part(&data), 84_538);
    }
}
