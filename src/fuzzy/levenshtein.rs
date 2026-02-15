pub struct LevenshteinMatcher {
    threshold: f32,
}

impl LevenshteinMatcher {
    pub fn new(threshold: f32) -> Self {
        Self { threshold }
    }

    /// Calculate Levenshtein edit distance between two strings
    pub fn distance(&self, s1: &str, s2: &str) -> usize {
        let len1 = s1.chars().count();
        let len2 = s2.chars().count();

        if len1 == 0 {
            return len2;
        }
        if len2 == 0 {
            return len1;
        }

        let mut matrix = vec![vec![0; len2 + 1]; len1 + 1];

        // Initialize first row and column
        for (i, row) in matrix.iter_mut().enumerate().take(len1 + 1) {
            row[0] = i;
        }
        for (j, cell) in matrix[0].iter_mut().enumerate().take(len2 + 1) {
            *cell = j;
        }

        // Fill matrix
        let chars1: Vec<char> = s1.chars().collect();
        let chars2: Vec<char> = s2.chars().collect();

        for i in 1..=len1 {
            for j in 1..=len2 {
                let cost = if chars1[i - 1] == chars2[j - 1] { 0 } else { 1 };

                matrix[i][j] = *[
                    matrix[i - 1][j] + 1,      // Delete
                    matrix[i][j - 1] + 1,      // Insert
                    matrix[i - 1][j - 1] + cost, // Replace
                ].iter().min().unwrap();
            }
        }

        matrix[len1][len2]
    }

    /// Calculate similarity (0.0 to 1.0)
    pub fn similarity(&self, s1: &str, s2: &str) -> f32 {
        let distance = self.distance(s1, s2);
        let max_len = s1.chars().count().max(s2.chars().count());

        if max_len == 0 {
            return 1.0;
        }

        1.0 - (distance as f32 / max_len as f32)
    }

    /// Find most similar word from candidates
    pub fn correct(&self, input: &str, candidates: &[&str]) -> Option<String> {
        let mut best_match: Option<(String, f32)> = None;

        for candidate in candidates {
            let sim = self.similarity(input, candidate);

            if sim >= self.threshold {
                if let Some((_, best_sim)) = &best_match {
                    if sim > *best_sim {
                        best_match = Some((candidate.to_string(), sim));
                    }
                } else {
                    best_match = Some((candidate.to_string(), sim));
                }
            }
        }

        best_match.map(|(word, _)| word)
    }
}

impl Default for LevenshteinMatcher {
    fn default() -> Self {
        Self::new(0.85)
    }
}
