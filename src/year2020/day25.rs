use crate::Problem;

pub struct Day;

impl Problem for Day {
    type Input = String;
    type Err = std::convert::Infallible;
    const TITLE: &'static str = "";

    fn solve(data: Self::Input) -> Result<(), Self::Err> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("test_resources/25-example.txt");
    const MAIN: &str = include_str!("test_resources/25-main.txt");

    #[test]
    fn first_part_example() {
        let input = Day::parse(EXAMPLE).unwrap();
    }

    #[test]
    fn first_part_main() {
        let input = Day::parse(MAIN).unwrap();
    }
}
