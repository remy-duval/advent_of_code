use commons::{ensure, err, Result, WrapErr};

pub const TITLE: &str = "Day 12: Passage Pathing";

pub fn run(raw: String) -> Result<()> {
    let data = parse(&raw)?;
    println!("1. Paths count is {}", first_part(&data)?);
    println!("2. Paths count is {}", second_part(&data)?);
    Ok(())
}

fn parse(s: &str) -> Result<Paths> {
    fn find_or_insert(id: &str, caves: &mut Vec<Cave>) -> usize {
        match caves.iter().position(|c| c.id == id) {
            Some(i) => i,
            None => {
                let id = id.to_owned();
                let small = id.chars().all(|x| x.is_ascii_lowercase());
                let paths = Vec::new();
                caves.push(Cave { id, small, paths });
                caves.len() - 1
            }
        }
    }

    let mut caves: Vec<Cave> = Vec::with_capacity(32);
    s.lines().try_for_each(|line| {
        if let Some((from, to)) = line.split_once('-') {
            let from_id = find_or_insert(from, &mut caves);
            let to_id = find_or_insert(to, &mut caves);
            caves[from_id].paths.push(to_id);
            caves[to_id].paths.push(from_id);
            Ok(())
        } else {
            Err(err!("Missing delimiter '-'"))
        }
    })?;

    let start = caves.iter().position(|c| c.id == "start");
    let start = start.wrap_err("Missing start cave")?;
    let end = caves.iter().position(|c| c.id == "end");
    let end = end.wrap_err("Missing end cave")?;
    Ok(Paths { start, end, caves })
}

fn first_part(paths: &Paths) -> Result<usize> {
    count_paths(paths, false)
}

fn second_part(paths: &Paths) -> Result<usize> {
    count_paths(paths, true)
}

/// Count all possible paths from "start" to "end" that don't visit small caves twice.
/// If `can_visit_one_small_cave_twice`, then at most once small cave can be visited 2 times.
fn count_paths(paths: &Paths, can_visit_small_twice: bool) -> Result<usize> {
    ensure!(paths.caves.len() <= 32, "too many caves (> 32)");
    // A path under construction
    struct Current {
        next: usize,                 // The next path to take
        visits: VisitedSet,          // A bit set of visited paths
        can_visit_small_twice: bool, // true if the next small cave can be visited a second time
    }

    let mut finished = 0;
    // Do a DFS from "start" to "end" to find all paths
    // Since we don't want the shortest path but all paths, this is the simplest algorithm to use
    let mut stack = Vec::with_capacity(32);
    stack.push(Current {
        next: paths.start,
        visits: VisitedSet(0).with(paths.start),
        can_visit_small_twice,
    });

    while let Some(c) = stack.pop() {
        if let Some(next_cave) = paths.caves.get(c.next) {
            next_cave.paths.iter().for_each(|&next| {
                if next == paths.end {
                    finished += 1;
                    return;
                } else if next == paths.start {
                    return;
                }

                let mut can_visit_small_twice = c.can_visit_small_twice;
                if c.visits.has(next) && paths.caves.get(next).map_or(false, |x| x.small) {
                    if !can_visit_small_twice {
                        return;
                    }
                    can_visit_small_twice = false;
                }
                stack.push(Current {
                    next,
                    visits: c.visits.with(next),
                    can_visit_small_twice,
                });
            });
        }
    }

    Ok(finished)
}

/// The paths for the puzzle
struct Paths {
    /// The index of the entry
    start: usize,
    /// The index of the exit
    end: usize,
    /// The caves, each having paths which are the indexes in this vec
    caves: Vec<Cave>,
}

/// A cave in the paths. It has an ID, can be small or big and is connected to other caves
struct Cave {
    id: String,
    small: bool,
    paths: Vec<usize>,
}

/// A bit set of visited index, can account for at most 32 caves though
struct VisitedSet(u32);

impl VisitedSet {
    /// Check if this index is in the visited set
    fn has(&self, i: usize) -> bool {
        let mask = 1 << i;
        (self.0 & mask) != 0
    }

    /// Add this index in the visited set
    fn with(&self, i: usize) -> Self {
        let mask = 1 << i;
        Self(self.0 | mask)
    }
}

#[cfg(test)]
mod tests;
