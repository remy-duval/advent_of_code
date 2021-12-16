use super::*;

const MAIN: &str = include_str!("data.txt");

#[test]
fn first_part_example_a() {
    let stream = parse("D2FE28").unwrap();
    assert_eq!(first_part(stream).unwrap(), 6);
}

#[test]
fn first_part_example_b() {
    let stream = parse("8A004A801A8002F478").unwrap();
    assert_eq!(first_part(stream).unwrap(), 16);
}

#[test]
fn first_part_example_c() {
    let stream = parse("620080001611562C8802118E34").unwrap();
    assert_eq!(first_part(stream).unwrap(), 12);
}

#[test]
fn first_part_example_d() {
    let stream = parse("C0015000016115A2E0802F182340").unwrap();
    assert_eq!(first_part(stream).unwrap(), 23);
}

#[test]
fn first_part_example_e() {
    let stream = parse("A0016C880162017C3686B18A3D4780").unwrap();
    assert_eq!(first_part(stream).unwrap(), 31);
}

#[test]
fn first_part_main() {
    let stream = parse(MAIN).unwrap();
    assert_eq!(first_part(stream).unwrap(), 891);
}

#[test]
fn second_part_example_a() {
    let stream = parse("C200B40A82").unwrap();
    assert_eq!(second_part(stream).unwrap(), 3);
}

#[test]
fn second_part_example_b() {
    let stream = parse("04005AC33890").unwrap();
    assert_eq!(second_part(stream).unwrap(), 54);
}

#[test]
fn second_part_example_c() {
    let stream = parse("880086C3E88112").unwrap();
    assert_eq!(second_part(stream).unwrap(), 7);
}

#[test]
fn second_part_example_d() {
    let stream = parse("CE00C43D881120").unwrap();
    assert_eq!(second_part(stream).unwrap(), 9);
}

#[test]
fn second_part_example_e() {
    let stream = parse("D8005AC2A8F0").unwrap();
    assert_eq!(second_part(stream).unwrap(), 1);
}

#[test]
fn second_part_example_f() {
    let stream = parse("F600BC2D8F").unwrap();
    assert_eq!(second_part(stream).unwrap(), 0);
}

#[test]
fn second_part_example_g() {
    let stream = parse("9C005AC2F8F0").unwrap();
    assert_eq!(second_part(stream).unwrap(), 0);
}

#[test]
fn second_part_example_h() {
    let stream = parse("9C0141080250320F1802104A08").unwrap();
    assert_eq!(second_part(stream).unwrap(), 1);
}

#[test]
fn second_part_main() {
    let stream = parse(MAIN).unwrap();
    assert_eq!(second_part(stream).unwrap(), 673_042_777_597);
}
