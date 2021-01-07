/// A ring of marbles
#[derive(Debug, Clone)]
pub struct Ring {
    current: usize,
    nodes: Vec<Node>,
}

impl Ring {
    /// Create a new ring of marbles
    pub fn new(max_point: usize) -> Self {
        Self {
            current: 0,
            nodes: vec![Default::default(); max_point + 1],
        }
    }

    /// Play the game with the given player scores for `steps` steps
    pub fn play(&mut self, mut players: Vec<usize>, steps: usize) -> Vec<usize> {
        let mut current: usize = 0;
        (0..steps).for_each(|step| {
            players[current] += self.next(step + 1);
            current += 1;
            if current >= players.len() {
                current = 0;
            }
        });

        players
    }

    /// Perform the next step of the game
    /// * if value is multiple of 23, remove the 7th prev before marble, current becomes 6th prev
    /// * if value is not multiple of 23, insert it after the 1st next marble
    ///
    /// ### Returns
    /// The gained score for the player, which is the sum of removed marbles
    pub fn next(&mut self, value: usize) -> usize {
        assert!(value < self.nodes.len());
        assert!(self.current < self.nodes.len());

        if value % 23 == 0 {
            // 7 steps counter clockwise
            (0..7).for_each(|_| self.current = self.nodes[self.current].before);
            let score = value + self.current;

            // The surrounding marbles
            let before = self.nodes[self.current].before;
            let after = self.nodes[self.current].after;

            // Remove the current marble by setting the after and before of surrounding marbles
            self.nodes[before].after = after;
            self.nodes[after].before = before;
            self.current = after;
            score
        } else {
            // The surrounding marbles
            let before = self.nodes[self.current].after;
            let after = self.nodes[before].after;
            // Add the value marble by setting the after and before of surrounding marbles
            self.nodes[before].after = value;
            self.nodes[after].before = value;
            // Set the value marble before and after to correct values
            self.nodes[value].before = before;
            self.nodes[value].after = after;
            self.current = value;
            0
        }
    }
}

/// A marble in the node ring
#[derive(Debug, Clone, Default)]
struct Node {
    before: usize,
    after: usize,
}
