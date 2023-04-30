use super::*;

#[test]
fn new() {
    let created: Grid<usize> = Grid::new(5, 4);
    assert_eq!(created.width(), 5);
    assert_eq!(created.height(), 0);
    assert_eq!(created.flattened(), &[]);
}

#[test]
fn tabulate() {
    let created = Grid::tabulate(5, 4, |(x, y)| x + 5 * y);
    assert_eq!(created.width(), 5);
    assert_eq!(created.height(), 4);
    assert_eq!(
        created.flattened(),
        &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19]
    )
}

#[test]
fn fill() {
    let created = Grid::fill(2, 2, 3);
    assert_eq!(created.width(), 2);
    assert_eq!(created.height(), 2);
    assert_eq!(created.flattened(), &[3, 3, 3, 3])
}

#[test]
fn with_default() {
    let created: Grid<usize> = Grid::with_default(2, 2);
    assert_eq!(created.width(), 2);
    assert_eq!(created.height(), 2);
    assert_eq!(created.flattened(), &[0, 0, 0, 0])
}

#[test]
fn insert_default_line() {
    let mut grid: Grid<usize> = Grid::new(5, 3);
    grid.insert_default_line();
    grid.insert_default_line();
    let added_line = grid.insert_default_line();
    added_line[2] = 69;
    assert_eq!(
        grid.flattened(),
        &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 69, 0, 0]
    )
}

#[test]
fn width() {
    let vec: Grid<usize> = Grid::new(5, 0);
    assert_eq!(vec.width(), 5);
}

#[test]
fn height() {
    let vec = Grid::tabulate(5, 4, |(x, y)| x + 5 * y);
    assert_eq!(vec.height(), 4);
}

#[test]
fn size() {
    let vec = Grid::tabulate(5, 4, |(x, y)| x + 5 * y);
    assert_eq!(vec.size(), (5, 4));
}

#[test]
fn get() {
    let vec = Grid::tabulate(5, 4, |(x, y)| x + 5 * y);
    assert_eq!(8, *vec.get((3, 1)).unwrap());
    assert_eq!(19, *vec.get((4, 3)).unwrap())
}

#[test]
fn get_mut() {
    let mut vec = Grid::tabulate(5, 4, |(x, y)| x + 5 * y);
    let inside = vec.get_mut((3, 1)).unwrap();
    *inside = 50;
    assert_eq!(50, *vec.get((3, 1)).unwrap());
}

#[test]
fn get_line() {
    let vec = Grid::tabulate(5, 4, |(x, y)| x + 5 * y);
    assert_eq!(&[15, 16, 17, 18, 19], vec.get_line(3).unwrap());
}

#[test]
fn get_line_mut() {
    let mut vec = Grid::tabulate(5, 4, |(x, y)| x + 5 * y);
    let inside = vec.get_line_mut(3).unwrap();
    inside[2] = 2;
    assert_eq!(&[15, 16, 2, 18, 19], vec.get_line(3).unwrap());
}

#[test]
fn push_line() {
    let mut vec = Grid::new(5, 1);
    vec.push_line(vec![1, 2, 3, 4, 5]);
    assert_eq!(1, vec.height());
    assert_eq!(&[1, 2, 3, 4, 5], vec.get_line(0).unwrap());
}

#[test]
#[should_panic]
fn push_line_too_short() {
    let mut vec = Grid::new(5, 1);
    vec.push_line(vec![1, 2, 3, 4]); // Too short
}

#[test]
fn insert_filled_line() {
    let mut grid = Grid::new(5, 3);
    grid.insert_filled_line(|i| i);
    grid.insert_filled_line(|i| 2 * i);
    let added_line = grid.insert_filled_line(|i| 3 * i);
    added_line[2] = 500;
    assert_eq!(
        grid.flattened(),
        &[0, 1, 2, 3, 4, 0, 2, 4, 6, 8, 0, 3, 500, 9, 12]
    )
}

#[test]
fn try_push_line() {
    let mut vec = Grid::new(5, 1);
    vec.try_push_line(vec![1, 2, 3, 4, 5]).unwrap(); // Ok
    vec.try_push_line(vec![1, 2, 3, 4]).unwrap_err(); // Too short
    vec.try_push_line(vec![1, 2, 3, 4, 5, 6]).unwrap_err(); // Too long
    assert_eq!(1, vec.height());
    assert_eq!(&[1, 2, 3, 4, 5], vec.get_line(0).unwrap());
}

#[test]
fn lines() {
    let vec = Grid::tabulate(5, 4, |(x, y)| x + 5 * y);
    let mut lines = vec.lines();
    assert_eq!(lines.next().unwrap(), &[0, 1, 2, 3, 4]);
    assert_eq!(lines.next().unwrap(), &[5, 6, 7, 8, 9]);
    assert_eq!(lines.next().unwrap(), &[10, 11, 12, 13, 14]);
    assert_eq!(lines.next().unwrap(), &[15, 16, 17, 18, 19]);
    assert_eq!(lines.next(), None);
}

#[test]
fn indices() {
    let vec: Grid<u8> = Grid::with_default(2, 2);
    let mut keys = vec.indices();
    assert_eq!(keys.next(), Some((0, 0)));
    assert_eq!(keys.next(), Some((1, 0)));
    assert_eq!(keys.next(), Some((0, 1)));
    assert_eq!(keys.next(), Some((1, 1)));
    assert_eq!(keys.next(), None);
}

#[test]
fn indexed_values() {
    let vec: Grid<u8> = Grid::with_default(2, 2);
    let mut keys = vec.indexed_values();
    assert_eq!(keys.next(), Some(((0, 0), &0)));
    assert_eq!(keys.next(), Some(((1, 0), &0)));
    assert_eq!(keys.next(), Some(((0, 1), &0)));
    assert_eq!(keys.next(), Some(((1, 1), &0)));
    assert_eq!(keys.next(), None);
}

#[test]
fn half_line() {
    let vec = Grid::tabulate(5, 4, |(x, y)| x + 5 * y);
    let mut points = vec.half_line((2, 2), (1, -1));
    assert_eq!(points.next(), Some(((2, 2), &12)));
    assert_eq!(points.next(), Some(((3, 1), &8)));
    assert_eq!(points.next(), Some(((4, 0), &4)));
    assert_eq!(points.next(), None);
}

#[test]
#[should_panic]
fn half_line_with_zero_step() {
    let vec = Grid::tabulate(5, 4, |(x, y)| x + 5 * y);
    vec.half_line((2, 2), (0, 0));
}
