use super::*;

const DATA: &str = include_str!("data.txt");

const A: &str = "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83";
const B: &str = "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7";

#[test]
fn first_part() {
    let first = parse(A);
    let second = parse(B);

    let result = closest(&first[..])
        .expect("Could not find closest !")
        .manhattan_distance();
    assert_eq!(result, 159);
    let result = closest(&second[..])
        .expect("Could not find closest !")
        .manhattan_distance();
    assert_eq!(result, 135);
}

#[test]
fn second_part() {
    let first = parse(A);
    let second = parse(B);

    let result = shortest(&first[..]).expect("Could not find shortest !").1;
    assert_eq!(result, 610);
    let result = shortest(&second[..]).expect("Could not find shortest !").1;
    assert_eq!(result, 410);
}

#[test]
fn solve_test() {
    let crossed = parse(DATA);
    let closest = closest(&crossed[..]).expect("Could not find closest !");
    let length = shortest(&crossed[..]).expect("Could not find shortest !").1;

    assert_eq!(529, closest.manhattan_distance());
    assert_eq!(20_386, length);
}
