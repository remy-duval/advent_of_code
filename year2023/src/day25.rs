use std::collections::BTreeMap;

use commons::error::Result;
use commons::WrapErr;

pub const TITLE: &str = "Day 25: Snowverload";

pub fn run(raw: String) -> Result<()> {
    let data = parse(&raw)?;
    let first = first_part(&data);
    println!("1. The product of the two group sizes is {first}");
    Ok(())
}

/// Start with all nodes on one side
/// Repeatedly find the node most connected to the other side and move it other
/// When there are exactly 3 connections to the other side, we found a minimal cut
fn first_part(g: &[Vec<usize>]) -> usize {
    let mut s = vec![true; g.len()];
    let s_len = loop {
        let mut max_node = None;
        let mut max_neighbours = None;
        let total_neighbours: usize = (0..s.len())
            .filter(|&node| s[node])
            .map(|node| {
                let neighbours = g[node]
                    .iter()
                    .filter(|&&e| matches!(s.get(e), Some(false)))
                    .count();
                if max_neighbours.map_or(true, |m| neighbours > m) {
                    max_neighbours = Some(neighbours);
                    max_node = Some(node);
                }
                neighbours
            })
            .sum();

        if total_neighbours == 3 {
            break s.iter().filter(|&&o| o).count();
        } else if let Some(max_node) = max_node {
            s[max_node] = false;
        }
    };

    s_len * (g.len() - s_len)
}

fn parse(s: &str) -> Result<Vec<Vec<usize>>> {
    let mut name_to_id: BTreeMap<&str, usize> = BTreeMap::new();
    let mut vertices: Vec<Vec<usize>> = vec![];
    s.lines().try_for_each(|line| {
        line.split_once(':')
            .and_then(|(from, to)| {
                let components: Vec<usize> = std::iter::once(from)
                    .chain(to.split_ascii_whitespace())
                    .map(str::trim)
                    .filter(|n| !n.is_empty())
                    .map(|name| {
                        *name_to_id.entry(name).or_insert_with(|| {
                            let id = vertices.len();
                            vertices.push(vec![]);
                            id
                        })
                    })
                    .collect();

                match components.as_slice() {
                    [from, to @ ..] => {
                        vertices.get_mut(*from)?.extend_from_slice(to);
                        for i in to {
                            vertices.get_mut(*i)?.push(*from);
                        }
                        Some(())
                    }
                    _ => None,
                }
            })
            .wrap_err_with(|| format!("invalid line {line:?}"))
    })?;

    Ok(vertices)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../examples/25.txt");
    const MAIN: &str = include_str!("../inputs/25.txt");

    #[test]
    fn first_part_example() {
        let data = parse(EXAMPLE).unwrap();
        assert_eq!(first_part(&data), 54);
    }

    #[test]
    fn first_part_main() {
        let data = parse(MAIN).unwrap();
        assert_eq!(first_part(&data), 495_607);
    }
}
