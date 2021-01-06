use super::*;

const EXAMPLE: &str = include_str!("example.txt");
const MAIN: &str = include_str!("data.txt");

#[test]
fn first_part_example() {
    let steps = Day::parse(EXAMPLE).unwrap().data;
    let requirements = build_requirements(&steps);
    let steps = process_steps(requirements);
    assert_eq!(steps, "CABDFE");
}

#[test]
fn first_part_main() {
    let steps = Day::parse(MAIN).unwrap().data;
    let requirements = build_requirements(&steps);
    let steps = process_steps(requirements);
    assert_eq!(steps, "HPDTNXYLOCGEQSIMABZKRUWVFJ");
}

#[test]
fn second_part_example() {
    let steps = Day::parse(EXAMPLE).unwrap().data;
    let requirements = build_requirements(&steps);
    let time = count_time(requirements, 2, 0);
    assert_eq!(time, 15);
}

#[test]
fn second_part_main() {
    let steps = Day::parse(MAIN).unwrap().data;
    let requirements = build_requirements(&steps);
    let time = count_time(requirements, 5, 60);
    assert_eq!(time, 908);
}
