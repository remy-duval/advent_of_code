use commons::Problem;

mod spring;

pub struct Day;

impl Problem for Day {
    type Input = spring::Scan;
    type Err = std::convert::Infallible;
    const TITLE: &'static str = "Day 17: Reservoir Research";

    fn solve(mut scan: Self::Input) -> Result<(), Self::Err> {
        scan.fill();
        println!("The scan contains {} wet tiles", scan.wet_tiles());
        println!("The scan contains {} water tiles", scan.water());

        Ok(())
    }
}

#[cfg(test)]
mod tests;
