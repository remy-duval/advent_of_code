use super::*;

const DATA: &str = include_str!("data.txt");

#[test]
fn run_until_nat_packet_test() {
    let memory = Day::parse(DATA).unwrap().data;
    let packet = run_until_nat_packet(&memory).unwrap();

    assert_eq!(Packet::new((255, 20771, 14834)), packet);
}

#[test]
fn run_until_duplicate_wakeup_test() {
    let memory = Day::parse(DATA).unwrap().data;
    let packet = run_until_duplicate_wakeup(&memory).unwrap();

    assert_eq!(Packet::new((0, 20771, 10215)), packet);
}