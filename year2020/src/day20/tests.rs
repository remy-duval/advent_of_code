use super::*;

const EXAMPLE: &str = include_str!("example_tiles.txt");
const EXAMPLE_IMAGE: &str = include_str!("example_image.txt");
const MAIN: &str = include_str!("data.txt");

#[test]
fn row_test() {
    let data = [true, false, false, true, true, true, true, true];
    let row = Row::new(&data);
    assert!(row.is_set(0));
    assert!(!row.is_set(1));
    assert!(!row.is_set(2));
    assert!(row.is_set(3));
    assert!(row.is_set(4));
    assert!(row.is_set(5));
    assert!(row.is_set(6));
    assert!(row.is_set(7));
    assert_eq!(row.to_string(), "#..#####");
}

#[test]
fn border_test() {
    let data = [
        true, false, true, false, true, true, true, true, true, false,
    ];
    let border = Border::new(&data);
    assert!(border.is_set(0));
    assert!(!border.is_set(1));
    assert!(border.is_set(2));
    assert!(!border.is_set(3));
    assert!(border.is_set(4));
    assert!(border.is_set(5));
    assert!(border.is_set(6));
    assert!(border.is_set(7));
    assert!(border.is_set(8));
    assert!(!border.is_set(9));
    assert_eq!(border.to_string(), "#.#.#####.");
}

#[test]
fn first_part_example() {
    let tiles = parse(EXAMPLE).unwrap();
    let image = match_tiles(tiles, 3).unwrap();
    let corners = first_part(&image, 3).unwrap();
    assert_eq!(corners, 20_899_048_083_289);
}

#[test]
fn first_part_main() {
    let tiles = parse(MAIN).unwrap();
    let image = match_tiles(tiles, IMAGE_WIDTH).unwrap();
    let corners = first_part(&image, IMAGE_WIDTH).unwrap();
    assert_eq!(corners, 140_656_720_229_539);
}

#[test]
fn assemble_image_example() {
    let tiles = parse(EXAMPLE).unwrap();
    let image = match_tiles(tiles, 3).unwrap();
    let mut image = FullImage::assemble(image, 3);
    let wanted = FullImage {
        data: EXAMPLE_IMAGE
            .lines()
            .map(|line| line.chars().collect_vec())
            .collect_vec(),
        width: 24,
    };

    if image != wanted {
        assert!((0..4).any(|_| {
            if image.clone().flipped() == wanted {
                return true;
            }

            image = image.rotated_right();
            if image == wanted {
                return true;
            }

            false
        }));
    }
}

#[test]
fn second_part_example() {
    let tiles = parse(EXAMPLE).unwrap();
    let image = match_tiles(tiles, 3).unwrap();
    let image = FullImage::assemble(image, 3);
    let roughness = second_part(image);
    assert_eq!(roughness, 273);
}

#[test]
fn second_part_main() {
    let tiles = parse(MAIN).unwrap();
    let image = match_tiles(tiles, IMAGE_WIDTH).unwrap();
    let image = FullImage::assemble(image, IMAGE_WIDTH);
    let roughness = second_part(image);
    assert_eq!(roughness, 1_885);
}
