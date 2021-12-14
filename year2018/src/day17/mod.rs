use commons::eyre::Result;

mod spring;

pub const TITLE: &str = "Day 17: Reservoir Research";

pub fn run(raw: String) -> Result<()> {
    let mut scan = parse(&raw)?;
    scan.fill();
    println!("The scan contains {} wet tiles", scan.wet_tiles());
    println!("The scan contains {} water tiles", scan.water());

    Ok(())
}

fn parse(s: &str) -> Result<spring::Scan> {
    s.parse()
}

#[cfg(test)]
mod tests;
