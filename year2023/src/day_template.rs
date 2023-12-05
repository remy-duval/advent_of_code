use commons::error::Result;

pub const TITLE: &str = "";

pub fn run(raw: String) -> Result<()> {
    let data = parse(raw.into())?;
    println!("1. TODO");
    println!("2.TODO");

    Ok(())
}

fn parse(s: std::borrow::Cow<'static, str>) -> Result<String> {
    Ok(s.into_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../examples/XX.txt");
    const MAIN: &str = include_str!("../inputs/XX.txt");

    #[test]
    fn first_part_example() {
        let data = parse(EXAMPLE.into()).unwrap();
    }

    #[test]
    fn first_part_main() {
        let data = parse(MAIN.into()).unwrap();
    }

    #[test]
    fn second_part_example() {
        let data = parse(EXAMPLE.into()).unwrap();
    }

    #[test]
    fn second_part_main() {
        let data = parse(MAIN.into()).unwrap();
    }
}
