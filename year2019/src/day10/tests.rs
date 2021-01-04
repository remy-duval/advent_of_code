use super::*;

const TEST_ONE: &str = include_str!("example_1.txt");
const TEST_TWO: &str = include_str!("example_2.txt");
const TEST_THREE: &str = include_str!("example_3.txt");
const TEST_FOUR: &str = include_str!("example_4.txt");
const DATA: &str = include_str!("data.txt");

#[test]
fn check_surveillance_point() {
    fn assertion(data: &str, expected_station: Point, expected_count: usize) {
        let (station, view) = data
            .parse::<AsteroidField>()
            .expect("Parse error !")
            .find_surveillance_point()
            .expect("Not found any surveillance point");

        assert_eq!(
            expected_station, station,
            "\n{} != {} for \n{}",
            expected_station, station, data
        );
        assert_eq!(
            expected_count,
            view.len(),
            "\n{} != {} for \n{}",
            expected_count,
            view.len(),
            data
        );
    }

    assertion(TEST_ONE, Point::new(5, 8), 33);
    assertion(TEST_TWO, Point::new(1, 2), 35);
    assertion(TEST_THREE, Point::new(6, 3), 41);
    assertion(TEST_FOUR, Point::new(11, 13), 210);
}

#[test]
fn destroy_order() {
    let asteroids: AsteroidField = TEST_FOUR.parse().expect("Parse error !");
    let (station, view) = asteroids
        .find_surveillance_point()
        .expect("Not found any surveillance point");
    let ordered = field_ordering(&station, view);

    // The 1st asteroid to be vaporized is at 11,12.
    // The 2nd asteroid to be vaporized is at 12,1.
    // The 3rd asteroid to be vaporized is at 12,2.
    // The 10th asteroid to be vaporized is at 12,8.
    // The 20th asteroid to be vaporized is at 16,0.
    // The 50th asteroid to be vaporized is at 16,9.
    // The 100th asteroid to be vaporized is at 10,16.
    // The 199th asteroid to be vaporized is at 9,6.
    // The 200th asteroid to be vaporized is at 8,2.
    // The 201st asteroid to be vaporized is at 10,9.
    assert_eq!(ordered[0], Point::new(11, 12));
    assert_eq!(ordered[1], Point::new(12, 1));
    assert_eq!(ordered[2], Point::new(12, 2));
    assert_eq!(ordered[9], Point::new(12, 8));
    assert_eq!(ordered[19], Point::new(16, 0));
    assert_eq!(ordered[49], Point::new(16, 9));
    assert_eq!(ordered[99], Point::new(10, 16));
    assert_eq!(ordered[198], Point::new(9, 6));
    assert_eq!(ordered[199], Point::new(8, 2));
    assert_eq!(ordered[200], Point::new(10, 9));
}

#[test]
fn solve_test() {
    let mut asteroids: AsteroidField = DATA.parse().unwrap();
    let (station, station_view) = asteroids
        .find_surveillance_point()
        .expect("Not found any surveillance point");
    assert_eq!(340, station_view.len());

    asteroids.set_station(station);
    let ordered = field_ordering(&station, station_view);
    let two_hundredth = ordered[199];

    assert_eq!(2_628, two_hundredth.x * 100 + two_hundredth.y);
}
