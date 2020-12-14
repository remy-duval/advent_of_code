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

    const A: &str = include_str!("test_resources/15-A.txt");
    const B: &str = include_str!("test_resources/15-B.txt");

    #[test]
    fn first_part_test_a() {
        let input = Day::parse(A).unwrap();
    }

    #[test]
    fn first_part_test_b() {
        let input = Day::parse(B).unwrap();
    }

    #[test]
    fn second_part_test_a() {
        let input = Day::parse(A).unwrap();
    }

    #[test]
    fn second_part_test_b() {
        let input = Day::parse(B).unwrap();
    }
}
