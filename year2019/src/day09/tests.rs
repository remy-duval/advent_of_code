use super::*;

const DATA: &str = include_str!("data.txt");

#[test]
fn solve_test() {
    let memory = Day::parse(DATA).unwrap().data;
    let mut test_process = Processor::with_initial_inputs(&memory, &[1]);
    let mut output_count: usize = 0;
    let mut current: i64 = 0;
    test_process.run_with_callbacks(
        0,
        |_| None,
        |_, out| {
            current = out;
            output_count += 1;
            Ok(())
        },
    );
    assert_eq!(output_count, 1, "The TEST program should output once only");
    assert_eq!(2_752_191_671, current);

    let mut boost_process = Processor::with_initial_inputs(&memory, &[2]);
    let coordinates = boost_process.read_next().expect("BOOST failed !");
    assert_eq!(87_571, coordinates);
}