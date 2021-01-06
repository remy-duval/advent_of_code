use std::str::FromStr;

use itertools::Itertools;

use commons::Problem;

mod folder;

pub struct Day;

impl Problem for Day {
    type Input = Tree;
    type Err = anyhow::Error;
    const TITLE: &'static str = "Day 8: Memory Maneuver";

    fn solve(tree: Self::Input) -> Result<(), Self::Err> {
        println!("The metadata sum is {}", tree.metadata_sum());
        println!("The root node value is {}", tree.root_node_value());

        Ok(())
    }
}

/// A flattened tree
#[derive(Debug, Clone)]
pub struct Tree(pub Vec<u32>);

impl Tree {
    /// Sum the metadata values in the tree for each node
    pub fn metadata_sum(&self) -> u32 {
        folder::fold(folder::MetadataSum, self).unwrap_or_default()
    }

    /// The value of the root node
    pub fn root_node_value(&self) -> u32 {
        folder::fold(folder::NodeValues, self).unwrap_or_default()
    }
}

impl FromStr for Tree {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.split_whitespace().map(str::parse).try_collect()?))
    }
}

#[cfg(test)]
mod tests;
