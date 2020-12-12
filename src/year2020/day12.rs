use crate::Problem;

pub struct Day;

impl Problem for Day {
    type Input = ();
    type Err = ();
    const TITLE: &'static str = "";

    fn solve(data: Self::Input) -> Result<(), Self::Err> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const A: &str = include_str!("test_resources/12-A.txt");
    const B: &str = include_str!("test_resources/12-B.txt");

    #[test]
    fn first_part_test_a() {}

    #[test]
    fn first_part_test_b() {}

    #[test]
    fn second_part_test_a() {}

    #[test]
    fn second_part_test_b() {}
}
