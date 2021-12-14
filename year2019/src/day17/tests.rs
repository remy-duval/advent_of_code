use super::*;

const TEST_ONE: &str = include_str!("example_1.txt");
const TEST_TWO: &str = include_str!("example_2.txt");
const CODE: &str = include_str!("data_code.txt");

#[test]
fn parse_test() {
    let mut points: HashSet<Point> = HashSet::new();
    points.extend(vec![
        Point::new(2, 0),
        Point::new(2, 1),
        Point::new(2, 2),
        Point::new(0, 2),
        Point::new(1, 2),
        Point::new(2, 2),
        Point::new(3, 2),
        Point::new(4, 2),
        Point::new(5, 2),
        Point::new(6, 2),
        Point::new(10, 2),
        Point::new(11, 2),
        Point::new(12, 2),
        Point::new(0, 3),
        Point::new(2, 3),
        Point::new(6, 3),
        Point::new(10, 3),
        Point::new(12, 3),
        Point::new(0, 4),
        Point::new(1, 4),
        Point::new(2, 4),
        Point::new(3, 4),
        Point::new(4, 4),
        Point::new(5, 4),
        Point::new(6, 4),
        Point::new(7, 4),
        Point::new(8, 4),
        Point::new(9, 4),
        Point::new(10, 4),
        Point::new(11, 4),
        Point::new(12, 4),
        Point::new(2, 5),
        Point::new(6, 5),
        Point::new(10, 5),
        Point::new(2, 6),
        Point::new(3, 6),
        Point::new(4, 6),
        Point::new(5, 6),
        Point::new(6, 6),
        Point::new(10, 6),
    ]);
    let expected = Scaffold {
        path: points,
        robot: (Point::new(10, 6), Direction::North),
    };
    let result: Scaffold = TEST_ONE.parse().expect("The parsing should work !");

    assert_eq!(expected.path, result.path)
}

#[test]
fn calibration_test_a() {
    let scaffold: Scaffold = TEST_ONE.parse().expect("The parsing should work !");
    let calibration = scaffold.intersections_sum();

    assert_eq!(76, calibration);
}

#[test]
fn calibration_test_b() {
    let memory = parse(CODE).unwrap().data;
    let scaffold = Scaffold::from_camera_program(&memory, false).unwrap();
    let calibration = scaffold.intersections_sum();

    assert_eq!(8520, calibration);
}

#[test]
fn straight_path_test_a() {
    let scaffold: Scaffold = TEST_TWO.parse().expect("The parsing should work !");
    let path = scaffold.straight_ahead_path().iter().join(",");

    assert_eq!(
        "R,8,R,8,R,4,R,4,R,8,L,6,L,2,R,4,R,4,R,8,R,8,R,8,L,6,L,2",
        path
    );
}

#[test]
fn straight_path_test_b() {
    let memory = parse(CODE).unwrap().data;
    let scaffold = Scaffold::from_camera_program(&memory, false).unwrap();
    let path = scaffold.straight_ahead_path().iter().join(",");

    assert_eq!(include_str!("data_path.txt"), path);
}

#[test]
fn compression_test_a() {
    let scaffold: Scaffold = TEST_TWO.parse().unwrap();
    let path = scaffold.straight_ahead_path();
    let (main, a, b, c) = compression(&path, (5, 20)).unwrap();
    let rebuilt = main.replace('A', &a).replace('B', &b).replace('C', &c);

    assert!(main.len() < 20);
    assert!(a.len() < 20);
    assert!(b.len() < 20);
    assert!(c.len() < 20);
    assert_eq!(
        "R,8,R,8,R,4,R,4,R,8,L,6,L,2,R,4,R,4,R,8,R,8,R,8,L,6,L,2",
        rebuilt
    );
}

#[test]
fn compression_test_b() {
    let memory = parse(CODE).unwrap().data;
    let scaffold = Scaffold::from_camera_program(&memory, false).unwrap();
    let path = scaffold.straight_ahead_path();
    let (main, a, b, c) = compression(&path, (5, 20)).unwrap();

    assert_eq!("A,A,B,C,B,C,B,C,A,C", main);
    assert_eq!("R,6,L,8,R,8", a);
    assert_eq!("R,4,R,6,R,6,R,4,R,4", b);
    assert_eq!("L,8,R,6,L,10,L,10", c);
    assert_eq!(
        include_str!("data_path.txt"),
        main.replace('A', &a).replace('B', &b).replace('C', &c)
    );
}
