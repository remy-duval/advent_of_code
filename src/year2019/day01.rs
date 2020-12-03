pub struct Day;

impl crate::Problem for Day {
    type Input = crate::parse::LineSep<i64>;
    type Err = std::convert::Infallible;
    const TITLE: &'static str = "Day 1 : The Tyranny of the Rocket Equation";

    fn solve(data: Self::Input) -> Result<(), Self::Err> {
        let (first, second) = solve(&data.data);
        println!("Fuel for single stage : {}", first);
        println!("Fuel complete : {}", second);
        Ok(())
    }
}

fn solve(masses: &[i64]) -> (i64, i64) {
    let first: i64 = masses.iter().map(|x| fuel_for_mass_one_stage(*x)).sum();
    let second: i64 = masses.iter().map(|x| fuel_for_mass_all(*x)).sum();

    (first, second)
}

fn fuel_for_mass_one_stage(mass: i64) -> i64 {
    mass / 3 - 2
}

fn fuel_for_mass_all(mass: i64) -> i64 {
    let mut acc: i64 = 0;
    let mut current: i64 = mass;
    loop {
        current = fuel_for_mass_one_stage(current);
        if current > 0 {
            acc += current;
        } else {
            return acc;
        }
    }
}

#[cfg(test)]
mod test {
    use crate::Problem;

    use super::*;

    const DATA: &str = include_str!("test_resources/day01.txt");

    #[test]
    fn all_parts_test() {
        let masses = Day::parse(DATA).unwrap();
        let (first, second) = solve(&masses.data);
        assert_eq!(3_256_794, first);
        assert_eq!(4_882_337, second);
    }
}
