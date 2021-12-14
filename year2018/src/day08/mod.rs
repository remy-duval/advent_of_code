use itertools::Itertools;

use commons::eyre::{Result, WrapErr};

mod folder;

pub const TITLE: &str = "Day 8: Memory Maneuver";

pub fn run(raw: String) -> Result<()> {
    let tree = parse(&raw)?;
    println!("The metadata sum is {}", tree.metadata_sum());
    println!("The root node value is {}", tree.root_node_value());
    Ok(())
}

fn parse(s: &str) -> Result<Tree> {
    Ok(Tree(
        s.split_whitespace()
            .map(str::parse)
            .try_collect()
            .wrap_err("failed to parse the tree")?,
    ))
}

/// A flattened tree
struct Tree(pub Vec<u32>);

impl Tree {
    /// Sum the metadata values in the tree for each node
    fn metadata_sum(&self) -> u32 {
        folder::fold(folder::MetadataSum, self).unwrap_or_default()
    }

    /// The value of the root node
    fn root_node_value(&self) -> u32 {
        folder::fold(folder::NodeValues, self).unwrap_or_default()
    }
}

#[cfg(test)]
mod tests;
