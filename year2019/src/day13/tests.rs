use super::*;

const DATA: &str = include_str!("data.txt");

#[test]
fn solve_test() {
    let mut memory = Day::parse(DATA).unwrap().data;
    memory[0] = 2;
    let mut engine = Processor::new(&memory);
    let mut state = GameState::default();
    let (score, (remaining, total_blocks)) =
        state.run_with_decider(&mut engine, false, simple_decider);

    assert_eq!(452, total_blocks);
    assert_eq!(
        0, remaining,
        "No blocks should remain after the game ends !"
    );
    assert_eq!(21415, score);
}
