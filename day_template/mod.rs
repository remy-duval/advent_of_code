use commons::{err, error::{Context, Error, Result}};

pub const TITLE: &str = "";

pub fn run(raw: String) -> Result<()> {
    let data = parse(&raw)?;
    println!("1. TODO");
    println!("2. TODO");

    Err(err!("TODO"))
}

fn parse(s: &str) -> Result<String> {
    Ok(s.parse()?)
}

#[cfg(test)]
mod tests;
