use commons::error::Result;

pub const TITLE: &str = "Day 1 : The Tyranny of the Rocket Equation";

pub fn run(raw: String) -> Result<()> {
    let data = parse(&raw)?;
    let (first, second) = solve(&data.data);
    println!("Fuel for single stage : {first}");
    println!("Fuel complete : {second}");
    Ok(())
}

fn parse(s: &str) -> Result<commons::parse::LineSep<i64>> {
    Ok(s.parse()?)
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
mod tests;
