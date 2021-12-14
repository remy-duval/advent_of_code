use super::*;

const EXAMPLE: &str = include_str!("example.txt");
const MAIN: &str = include_str!("data.txt");

#[test]
fn first_part_example() {
    let input = parse(EXAMPLE).unwrap();
    assert_eq!(71, input.error_rate);
}

#[test]
fn first_part_main() {
    let input = parse(MAIN).unwrap();
    assert_eq!(27870, input.error_rate);
}

#[test]
fn second_part_example() {
    let input = parse(EXAMPLE).unwrap();
    let headers = input.find_headers().unwrap();
    assert_eq!(&headers, &["row", "class", "seat"]);
}

#[test]
fn second_part_main() {
    let input = parse(MAIN).unwrap();
    let headers = input.find_headers().unwrap();

    assert_eq!(
        &headers,
        &[
            "price",
            "train",
            "duration",
            "seat",
            "arrival location",
            "departure location",
            "arrival track",
            "zone",
            "arrival station",
            "route",
            "departure date",
            "arrival platform",
            "row",
            "departure track",
            "wagon",
            "type",
            "class",
            "departure platform",
            "departure station",
            "departure time",
        ]
    );

    assert_eq!(input.departure_product(headers), 3_173_135_507_987);
}
