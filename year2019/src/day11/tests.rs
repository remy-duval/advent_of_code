use super::*;

const DATA: &str = include_str!("data.txt");

const EXPECTED: &str = " ###  #### ###  #### ###  ###  #  #  ##    \n \
                            #  #    # #  # #    #  # #  # # #  #  #   \n \
                            #  #   #  #  # ###  #  # #  # ##   #      \n \
                            ###   #   ###  #    ###  ###  # #  #      \n \
                            #    #    # #  #    #    # #  # #  #  # @ \n \
                            #    #### #  # #    #    #  # #  #  ##    ";

#[test]
fn solve_test() {
    let memory = parse(DATA).unwrap().data;
    let mut hull: HashMap<Point, u8> = HashMap::new();
    hull.insert(Point::new(0, 0), 1);
    let second_paint: String = paint_hull(&memory, &mut hull).unwrap();

    assert_eq!(EXPECTED, &second_paint);
}
