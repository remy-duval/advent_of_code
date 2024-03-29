use super::*;

const TEST_ONE: &str = "12345678";
const TEST_TWO: &str = "80871224585914546619083218645595";
const TEST_THREE: &str = "19617804207202209144916044189917";
const TEST_FOUR: &str = "69317163492948606335995924319873";
const TEST_FIVE: &str = "03036732577212944063491565474664";
const TEST_SIX: &str = "02935109699940807407585447034323";
const TEST_SEVEN: &str = "03081770884921959731165446850517";
const DATA: &str = include_str!("data.txt");

#[test]
fn patterns_test() {
    assert_eq!(1, pattern_element(0, 0));
    assert_eq!(0, pattern_element(0, 1));
    assert_eq!(0, pattern_element(0, 2));

    assert_eq!(0, pattern_element(1, 0));
    assert_eq!(1, pattern_element(1, 1));
    assert_eq!(0, pattern_element(1, 2));

    let first: Vec<i32> = (0..8).map(|idx| pattern_element(idx, 0)).collect();
    assert_eq!(&[1, 0, -1, 0, 1, 0, -1, 0], &first[..]);

    let second: Vec<i32> = (0..8).map(|idx| pattern_element(idx, 1)).collect();
    assert_eq!(&[0, 1, 1, 0, 0, -1, -1, 0], &second[..]);

    let third: Vec<i32> = (0..8).map(|idx| pattern_element(idx, 2)).collect();
    assert_eq!(&[0, 0, 1, 1, 1, 0, 0, 0], &third[..]);

    let fourth: Vec<i32> = (0..8).map(|idx| pattern_element(idx, 3)).collect();
    assert_eq!(&[0, 0, 0, 1, 1, 1, 1, 0], &fourth[..]);
}

#[test]
fn naive_fft_test() {
    fn assertion(data: &str, steps: usize, expected: [i32; 8]) {
        let input: Vec<i32> = parse(data).unwrap();
        assert_eq!(&expected, &naive_fft(&input, steps)[..8])
    }

    assertion(TEST_ONE, 1, [4, 8, 2, 2, 6, 1, 5, 8]);
    assertion(TEST_ONE, 2, [3, 4, 0, 4, 0, 4, 3, 8]);
    assertion(TEST_ONE, 3, [0, 3, 4, 1, 5, 5, 1, 8]);
    assertion(TEST_ONE, 4, [0, 1, 0, 2, 9, 4, 9, 8]);
    assertion(TEST_TWO, 100, [2, 4, 1, 7, 6, 1, 7, 6]);
    assertion(TEST_THREE, 100, [7, 3, 7, 4, 5, 4, 1, 8]);
    assertion(TEST_FOUR, 100, [5, 2, 4, 3, 2, 1, 3, 3]);
    assertion(DATA, 100, [2, 9, 7, 9, 5, 5, 0, 7]);
}

#[test]
fn fast_second_half_fft_test() {
    fn assertion(data: &str, steps: usize, expected: [i32; 8]) {
        let input: Vec<i32> = parse(data).unwrap();
        assert_eq!(&expected, &fast_second_half_fft(&input, steps)[..])
    }

    assertion(TEST_FIVE, 100, [8, 4, 4, 6, 2, 0, 2, 6]);
    assertion(TEST_SIX, 100, [7, 8, 7, 2, 5, 2, 7, 0]);
    assertion(TEST_SEVEN, 100, [5, 3, 5, 5, 3, 7, 3, 1]);
    assertion(DATA, 100, [8, 9, 5, 6, 8, 5, 2, 9]);
}
