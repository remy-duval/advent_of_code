use std::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};
use std::collections::BinaryHeap;

use super::input::{Bot, Dimension, Point3};

/// Partition the space until a point is found which is in range with the most bots
pub fn partition(bots: &[Bot]) -> Option<Point3> {
    // See the implementation of Ord to understand how this priority queue works
    let mut queue: BinaryHeap<Partition> = BinaryHeap::new();
    queue.push(Partition::initial(bots));

    // Pop the partition with the most bots in range and the smallest size
    while let Some(current) = queue.pop() {
        if current.region.size <= 1 {
            // This partition is a single point, we found the result
            // The Ord guarantees this is the point we want (ties broken by distance to origin)
            return Some(current.region.bottom);
        } else {
            // Else divide and conquer by breaking the current region in smaller ones
            // By definition the sub-regions will be in range of at most the same number of bots
            // This is why the priority queue approach works (it will zero in on the best region)
            current.divide_into(bots, &mut queue);
        }
    }

    None
}

/// A partition of space based on an octahedron
#[derive(Debug, Clone)]
struct Partition {
    /// The region of space that this partition represents
    region: Cube,
    /// The number of bots that are in range of that partition
    bots: usize,
}

impl Partition {
    /// Build a partition from the given region, computing the count of bots in range
    fn new(region: Cube, bots: &[Bot]) -> Self {
        let bots = bots.iter().filter(|&bot| region.intersects(bot)).count();
        Self { region, bots }
    }

    /// Build the initial partition, it should be large enough to be in range of everything
    fn initial(bots: &[Bot]) -> Self {
        let mut min = Point3::new(Dimension::MAX, Dimension::MAX, Dimension::MAX);
        let mut max = Point3::new(Dimension::MIN, Dimension::MIN, Dimension::MIN);
        bots.iter().for_each(|next| {
            min.x = min.x.min(next.pos.x - next.r);
            max.x = max.x.max(next.pos.x + next.r);
            min.y = min.y.min(next.pos.y - next.r);
            max.y = max.y.max(next.pos.y + next.r);
            min.z = min.x.min(next.pos.z - next.r);
            max.z = max.x.max(next.pos.z + next.r);
        });

        let size = {
            let base = (max.x - min.x).max(max.x - min.x).max(max.z - min.z) / 2;
            // Size should be a power of 2 if possible, so compute
            // the smallest power of 2 that is greater than the starting one
            let mut size = 1;
            while size < base {
                size *= 2;
            }
            size
        };

        Self {
            region: Cube { bottom: min, size },
            bots: bots.len(), // All the bots are by definition in range, no need to waste time
        }
    }

    /// The distance from the center of this partition to the origin
    fn origin_distance(&self) -> Dimension {
        self.region.bottom.origin_distance()
    }

    /// Split this partition into smaller ones, adding them to the given binary heap
    fn divide_into(self, bots: &[Bot], into: &mut BinaryHeap<Self>) {
        let size = self.region.size / 2;
        if size <= 0 {
            unreachable!("New size is unexpected {} (required to be >= 1)", size);
        }

        let mut push = |bottom: Point3| {
            into.push(Self::new(Cube::new(bottom, size), bots));
        };

        let point = self.region.bottom;
        push(Point3::new(point.x + size, point.y + size, point.z + size));
        push(Point3::new(point.x + size, point.y + size, point.z));
        push(Point3::new(point.x + size, point.y, point.z + size));
        push(Point3::new(point.x, point.y + size, point.z + size));
        push(Point3::new(point.x, point.y, point.z + size));
        push(Point3::new(point.x, point.y + size, point.z));
        push(Point3::new(point.x + size, point.y, point.z));
        push(point);
    }
}

impl Eq for Partition {}

impl PartialEq for Partition {
    /// Impl of Eq for keeping coherence since we override the default Ord
    fn eq(&self, other: &Self) -> bool {
        self.bots == other.bots
            && self.region.size == other.region.size
            && self.origin_distance() == other.origin_distance()
    }
}

impl Ord for Partition {
    /// This is made for use in a binary heap
    fn cmp(&self, other: &Self) -> Ordering {
        // Order by the maximum bots in range
        self.bots.cmp(&other.bots).then_with(|| {
            // Then for that maximum bots in range find the one with the smallest region
            // To reverse the comparison `other` is compared to `self` instead of usual
            // Finally break ties with the distance to the origin
            other
                .region
                .size
                .cmp(&self.region.size)
                .then_with(|| other.origin_distance().cmp(&self.origin_distance()))
        })
    }
}

impl PartialOrd for Partition {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// A region in space in the easiest form to work with
#[derive(Debug, Clone)]
struct Cube {
    /// The bottom coordinate of the cube
    bottom: Point3,
    /// The height of the cube such that each top coordinate is bottom + size - 1
    size: Dimension,
}

impl Cube {
    /// Build a new cube
    fn new(bottom: Point3, size: Dimension) -> Self {
        Self { bottom, size }
    }

    /// Check if this cube intersects with the range of the given Bot
    fn intersects(&self, bot: &Bot) -> bool {
        // The single coordinate path length from 'from' to between 'bottom' and 'bottom + height'
        fn path(from: Dimension, bottom: Dimension, height: Dimension) -> Dimension {
            let diff = from - bottom;
            if diff < 0 {
                -diff // Go up to reach the range
            } else if diff >= height {
                diff - height // Go down to reach the range
            } else {
                0 // We are already within the range
            }
        }

        // The length of the path to go from the bot center to any part of the cube
        let distance = path(bot.pos.x, self.bottom.x, self.size - 1)
            + path(bot.pos.y, self.bottom.y, self.size - 1)
            + path(bot.pos.z, self.bottom.z, self.size - 1);

        // If the distance is lower than the radius, at least one point of the cube is in range !
        distance <= bot.r
    }
}

#[test]
fn overlaps_test() {
    use super::*;

    let bots = Day::parse(include_str!("example_2.txt")).unwrap().data;
    let base = Cube::new(Point3::new(12, 12, 12), 1);
    assert_eq!(bots.iter().filter(|bot| base.intersects(bot)).count(), 5);
}
