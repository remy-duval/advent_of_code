use super::*;

const TEST_ONE: &str = include_str!("example_1.txt");
const TEST_TWO: &str = include_str!("example_2.txt");
const TEST_THREE: &str = include_str!("example_3.txt");
const TEST_FOUR: &str = include_str!("example_4.txt");
const DATA: &str = include_str!("data.txt");

#[test]
fn small_deck_test() {
    // Test that the shuffling with LCF produces the expected order (as per the examples)
    fn assertion(data: &str, expected: [i128; 10]) {
        let shuffles = Day::parse(data).unwrap().data;
        let lcf = LinearFunction::fold(shuffles, 10);
        let mut result = [0; 10];
        for card in 0..10 {
            let pos = lcf.apply(card, 10);
            result[pos as usize] = card;
        }

        assert_eq!(
            &expected, &result,
            "Last position was expected to be {:?} instead of {:?}",
            expected, result
        );
    }

    assertion(TEST_ONE, [0, 3, 6, 9, 2, 5, 8, 1, 4, 7]);
    assertion(TEST_TWO, [3, 0, 7, 4, 1, 8, 5, 2, 9, 6]);
    assertion(TEST_THREE, [6, 3, 0, 7, 4, 1, 8, 5, 2, 9]);
    assertion(TEST_FOUR, [9, 2, 5, 8, 1, 4, 7, 0, 3, 6]);
}

#[test]
fn inverse_test() {
    // Test that the inverse function produced gives back [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
    // When applied to the last positions of the shuffling (as per the examples)
    fn assertion(data: &str, last_position: [i128; 10]) {
        let shuffles = Day::parse(data).unwrap().data;
        let lcf = LinearFunction::fold(shuffles, 10).inverse(10);
        let mut result = [0; 10];
        for (card, &last) in last_position.iter().enumerate() {
            let pos = lcf.apply(card as i128, 10);
            result[pos as usize] = last;
        }

        assert_eq!(
            &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
            &result,
            "First position was expected to be {:?} instead of {:?}",
            [0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
            result
        );
    }

    assertion(TEST_ONE, [0, 3, 6, 9, 2, 5, 8, 1, 4, 7]);
    assertion(TEST_TWO, [3, 0, 7, 4, 1, 8, 5, 2, 9, 6]);
    assertion(TEST_THREE, [6, 3, 0, 7, 4, 1, 8, 5, 2, 9]);
    assertion(TEST_FOUR, [9, 2, 5, 8, 1, 4, 7, 0, 3, 6]);
}

#[test]
fn first_part_test() {
    let shuffles = Day::parse(DATA).unwrap().data;
    assert_eq!(1_879, first_part(shuffles))
}

#[test]
fn second_part_test() {
    let shuffles = Day::parse(DATA).unwrap().data;
    assert_eq!(73_729_306_030_290, second_part(shuffles))
}
