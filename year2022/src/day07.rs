use commons::error::Result;
use commons::{err, WrapErr};

pub const TITLE: &str = "Day 7: No Space Left On Device";

type Size = u32;
type Index = u16;
type Name<'a> = &'a str;
const SMALL_DIRECTORY: Size = 100_000;
const TOTAL_SPACE: Size = 70_000_000;
const REQUIRED_SPACE: Size = 30_000_000;

pub fn run(raw: String) -> Result<()> {
    let data = parse(&raw)?;
    let first = first_part(&data);
    println!("1. The sum of sizes of small directories is {first}");
    let second = second_part(&data);
    println!("2. The size of the smallest directory to delete to free enough space is {second}");

    Ok(())
}

fn first_part(nodes: &[Node]) -> Size {
    nodes
        .iter()
        .map(|node| node.size)
        .filter(|size| *size <= SMALL_DIRECTORY)
        .sum()
}

fn second_part(nodes: &[Node]) -> Size {
    let used_space = nodes.first().map_or(0, |node| node.size);
    let free_space = TOTAL_SPACE.saturating_sub(used_space);
    let space_to_free = REQUIRED_SPACE.saturating_sub(free_space);
    nodes
        .iter()
        .map(|node| node.size)
        .filter(|size| *size >= space_to_free)
        .min()
        .unwrap_or_default()
}

#[derive(Debug, Clone)]
struct Node<'a> {
    name: Name<'a>,
    parent: Index,
    size: Size,
    children_offset: Index,
    children_count: Index,
}

fn parse(s: &str) -> Result<Vec<Node>> {
    // Build the directory hierarchy as a flattened tree
    // - the root is first element
    // - each element points to its parent
    let mut directories: Vec<Node> = vec![Node {
        name: "/",
        parent: 0,
        size: 0,
        children_offset: 0,
        children_count: 0,
    }];
    let mut current: usize = 0;
    let mut lines = s.lines().map(str::trim).peekable();
    while let Some(line) = lines.next() {
        if let Some(command) = line.strip_prefix("$ ") {
            if let Some(destination) = command.strip_prefix("cd ") {
                match destination {
                    "/" => current = 0,
                    ".." => current = directories[current].parent as usize,
                    name => {
                        let dir = &directories[current];
                        let start = current + dir.children_offset as usize;
                        let end = start + dir.children_count as usize;
                        let children = &directories[start..end];
                        if let Some(i) = children.iter().position(|n| n.name == name) {
                            current = start + i;
                        } else {
                            return Err(err!("{line}: no such directory"));
                        }
                    }
                }
            } else if command == "ls" {
                // All the lines until the next command are the directory content
                // - if it is a file: increase the directory size value and forget it
                // - if it is a directory: add it as a child in the flattened tree
                let directory_start = directories.len();
                while let Some(entry) = lines.next_if(|e| !e.starts_with('$')) {
                    if let Some(name) = entry.strip_prefix("dir ") {
                        directories.push(Node {
                            name,
                            parent: current as Index,
                            size: 0,
                            children_offset: 0,
                            children_count: 0,
                        });
                    } else if let Some((size, _)) = entry.split_once(' ') {
                        let size: Size =
                            size.parse().wrap_err_with(|| format!("{line}: bad size"))?;
                        directories[current].size += size;
                    } else {
                        return Err(err!("{line}: not a file or a directory"));
                    }
                }

                let directory_size = directories.len() - directory_start;
                let directory_offset = directory_start - current;
                let directory = &mut directories[current];
                directory.children_offset = directory_offset as Index;
                directory.children_count = directory_size as Index;
            } else {
                return Err(err!("{line}: unknown command"));
            }
        } else {
            return Err(err!("{line}: not a command"));
        }
    }

    // Propagate sizes from sub-directories to their parents
    for i in (1..directories.len()).rev() {
        let Node { parent, size, .. } = directories[i];
        directories[parent as usize].size += size;
    }

    Ok(directories)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../examples/07.txt");
    const MAIN: &str = include_str!("../inputs/07.txt");

    #[test]
    fn first_part_example() {
        let data = parse(EXAMPLE).unwrap();
        assert_eq!(first_part(&data), 95437);
    }

    #[test]
    fn first_part_main() {
        let data = parse(MAIN).unwrap();
        assert_eq!(first_part(&data), 1315285);
    }

    #[test]
    fn second_part_example() {
        let data = parse(EXAMPLE).unwrap();
        assert_eq!(second_part(&data), 24933642);
    }

    #[test]
    fn second_part_main() {
        let data = parse(MAIN).unwrap();
        assert_eq!(second_part(&data), 9847279);
    }
}
