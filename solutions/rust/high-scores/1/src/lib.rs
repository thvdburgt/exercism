#[derive(Debug)]
pub struct HighScores {
    scores: Vec<u32>,
}

impl HighScores {
    /// Create a new HighScores instance with the given scores.
    pub fn new(scores: &[u32]) -> Self {
        HighScores {
            scores: scores.to_vec(),
        }
    }

    /// Return all the scores as a slice.
    pub fn scores(&self) -> &[u32] {
        &self.scores
    }

    /// Return the latest (last) score.
    pub fn latest(&self) -> Option<u32> {
        self.scores.last().copied()
    }

    /// Returns the highest score.
    pub fn personal_best(&self) -> Option<u32> {
        self.scores.iter().max().copied()
    }

    /// Returns the top three scores in descending order.
    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut top_scores: Vec<u32> = self.scores.to_vec();
        top_scores.sort_unstable_by(|a, b| b.cmp(a)); // Sort in descending order
        top_scores.truncate(3); // Keep only the top 3 scores
        top_scores
    }
}
