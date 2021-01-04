use super::*;

const EXAMPLE: &str = include_str!("example.txt");
const MAIN: &str = include_str!("data.txt");

#[test]
fn first_part_example() {
    let recipes = Day::parse(EXAMPLE).unwrap().data;
    let containers = find_containers(&recipes);
    let result = first_part(&recipes, &containers);
    assert_eq!(result, 5);
}

#[test]
fn first_part_main() {
    let recipes = Day::parse(MAIN).unwrap().data;
    let containers = find_containers(&recipes);
    let result = first_part(&recipes, &containers);
    assert_eq!(result, 2170);
}

#[test]
fn second_part_example() {
    let recipes = Day::parse(EXAMPLE).unwrap().data;
    let containers = find_containers(&recipes);
    let result = second_part(containers);
    assert_eq!(result, "mxmxvkd,sqjhc,fvjkl");
}

#[test]
fn second_part_main() {
    let recipes = Day::parse(MAIN).unwrap().data;
    let containers = find_containers(&recipes);
    let result = second_part(containers);
    assert_eq!(result, "nfnfk,nbgklf,clvr,fttbhdr,qjxxpr,hdsm,sjhds,xchzh");
}