//! FastText-based text expander for abbreviation and synonym expansion
//!
//! This module provides intelligent text expansion using fastText embeddings.
//! It is only available when the "fasttext" feature is enabled.

use finalfusion::embeddings::Embeddings;
use finalfusion::vocab::{SimpleVocab, Vocab};
use finalfusion::storage::NdArray;
use finalfusion::io::ReadEmbeddings;
use std::collections::HashMap;
use std::path::Path;
use std::io::BufReader;
use std::sync::RwLock;

/// Error type for FastText operations
#[derive(Debug)]
pub enum FastTextError {
    LoadFailed(String),
    ModelNotLoaded,
    QueryFailed(String),
}

impl std::fmt::Display for FastTextError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FastTextError::LoadFailed(msg) => write!(f, "Load failed: {}", msg),
            FastTextError::ModelNotLoaded => write!(f, "Model not loaded"),
            FastTextError::QueryFailed(msg) => write!(f, "Query failed: {}", msg),
        }
    }
}

impl std::error::Error for FastTextError {}

/// Cached embeddings index for fast lookup
struct EmbeddingsIndex {
    /// Word to embedding index mapping
    word_indices: HashMap<String, usize>,
    /// Vocabulary size
    vocab_size: usize,
    /// Dimension of embeddings
    embedding_dim: usize,
}

impl EmbeddingsIndex {
    fn new(embeddings: &Embeddings<SimpleVocab, NdArray>) -> Self {
        let vocab = embeddings.vocab();
        let words: Vec<String> = vocab.words().iter().map(|w| w.to_string()).collect();
        let vocab_size = words.len();

        // Create word to index mapping
        let mut word_indices = HashMap::new();
        for (i, word) in words.into_iter().enumerate() {
            word_indices.insert(word, i);
        }

        // Get embedding dimension
        let embedding_dim = embeddings
            .embedding(vocab.words().first().unwrap_or(&"".to_string()))
            .map(|e| e.len())
            .unwrap_or(0);

        Self {
            word_indices,
            vocab_size,
            embedding_dim,
        }
    }

    fn get_index(&self, word: &str) -> Option<usize> {
        self.word_indices.get(word).copied()
    }
}

/// FastText-based text expander
///
/// Uses fastText embeddings to find similar words for abbreviation expansion.
/// For example, "明早" -> "明天早上", "国考" -> "国家考试"
pub struct FastTextExpander {
    embeddings: Option<Embeddings<SimpleVocab, NdArray>>,
    /// Cached index for fast lookup
    index: Option<EmbeddingsIndex>,
    similarity_threshold: f32,
    max_candidates: usize,
    /// Pre-defined abbreviation mappings (fallback when model not available)
    abbreviation_map: HashMap<String, Vec<String>>,
}

impl FastTextExpander {
    /// Load a fastText model from file
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, FastTextError> {
        let file = std::fs::File::open(path.as_ref())
            .map_err(|e| FastTextError::LoadFailed(format!("Failed to open file: {}", e)))?;

        let embeddings = Embeddings::read_embeddings(&mut BufReader::new(file))
            .map_err(|e| FastTextError::LoadFailed(format!("Failed to load embeddings: {:?}", e)))?;

        // Build index for fast lookup
        let index = EmbeddingsIndex::new(&embeddings);

        let mut expander = Self {
            embeddings: Some(embeddings),
            index: Some(index),
            similarity_threshold: 0.75,
            max_candidates: 5,
            abbreviation_map: HashMap::new(),
        };

        // Load default abbreviation mappings
        expander.load_default_mappings();

        Ok(expander)
    }

    /// Create a new expander with default settings (no model, mappings only)
    pub fn new() -> Self {
        let mut expander = Self {
            embeddings: None,
            index: None,
            similarity_threshold: 0.75,
            max_candidates: 5,
            abbreviation_map: HashMap::new(),
        };
        expander.load_default_mappings();
        expander
    }

    /// Load default Chinese abbreviation mappings
    fn load_default_mappings(&mut self) {
        // Time-related abbreviations
        self.abbreviation_map.insert("明早".to_string(), vec!["明天早上".to_string()]);
        self.abbreviation_map.insert("明晚".to_string(), vec!["明天晚上".to_string()]);
        self.abbreviation_map.insert("今早".to_string(), vec!["今天早上".to_string()]);
        self.abbreviation_map.insert("今晚".to_string(), vec!["今天晚上".to_string()]);
        self.abbreviation_map.insert("明天下午".to_string(), vec!["明天中午".to_string(), "下午".to_string()]);

        // Common abbreviations
        self.abbreviation_map.insert("国考".to_string(), vec!["国家公务员考试".to_string()]);
        self.abbreviation_map.insert("高考".to_string(), vec!["全国统一高考".to_string()]);
        self.abbreviation_map.insert("中考".to_string(), vec!["初中升高考试".to_string()]);
        self.abbreviation_map.insert("考研".to_string(), vec!["研究生入学考试".to_string()]);
        self.abbreviation_map.insert("公考".to_string(), vec!["公务员考试".to_string()]);
    }

    /// Set the similarity threshold (0.0 - 1.0)
    pub fn with_threshold(mut self, threshold: f32) -> Self {
        self.similarity_threshold = threshold;
        self
    }

    /// Set maximum number of candidates to return
    pub fn with_max_candidates(mut self, max: usize) -> Self {
        self.max_candidates = max;
        self
    }

    /// Expand an abbreviation to possible full forms
    pub fn expand(&self, input: &str) -> Vec<String> {
        // First check predefined mappings
        if let Some(mappings) = self.abbreviation_map.get(input) {
            return mappings.clone();
        }

        // If no model loaded, return empty
        let embeddings = match &self.embeddings {
            Some(e) => e,
            None => return vec![],
        };

        // Use fastText to find similar words
        self.find_similar(embeddings, input)
    }

    /// Find similar words using fastText embeddings
    fn find_similar(&self, embeddings: &Embeddings<SimpleVocab, NdArray>, word: &str) -> Vec<String> {
        // Get the query embedding
        let query_embedding = match embeddings.embedding(word) {
            Some(emb) => {
                let slice = emb.as_slice();
                if slice.is_none() {
                    return vec![];
                }
                slice.unwrap().to_vec()
            }
            None => return vec![],
        };

        if query_embedding.is_empty() {
            return vec![];
        }

        // Use optimized iteration - first check if word exists in vocabulary
        let vocab = embeddings.vocab();
        let words: Vec<&String> = vocab.words().iter().collect();

        // Find similar words with early termination for efficiency
        let mut candidates: Vec<(String, f32)> = Vec::new();
        let threshold = self.similarity_threshold;

        for w in &words {
            if *w == word {
                continue;
            }

            if let Some(embedding) = embeddings.embedding(w) {
                let emb_slice = embedding.as_slice().unwrap_or(&[]);
                if emb_slice.is_empty() {
                    continue;
                }
                let similarity = cosine_similarity(&query_embedding, emb_slice);
                if similarity >= threshold {
                    candidates.push((w.to_string(), similarity));
                    // Early termination if we have enough high-similarity candidates
                    if candidates.len() > self.max_candidates * 10 {
                        break;
                    }
                }
            }
        }

        // Sort by similarity and take top candidates
        candidates.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        candidates.truncate(self.max_candidates);

        candidates.into_iter().map(|(w, _)| w).collect()
    }

    /// Check if a word has possible expansions
    pub fn has_expansion(&self, word: &str) -> bool {
        self.abbreviation_map.contains_key(word) ||
        (self.embeddings.is_some() && {
            let embeddings = self.embeddings.as_ref().unwrap();
            embeddings.embedding(word).is_some()
        })
    }
}

impl Default for FastTextExpander {
    fn default() -> Self {
        Self::new()
    }
}

/// Calculate cosine similarity between two embeddings
fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    if a.len() != b.len() || a.is_empty() {
        return 0.0;
    }

    let dot_product: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
    let norm_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
    let norm_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();

    if norm_a == 0.0 || norm_b == 0.0 {
        return 0.0;
    }

    dot_product / (norm_a * norm_b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_expander() {
        let expander = FastTextExpander::new();

        // Test predefined mappings
        let expansions = expander.expand("明早");
        assert!(!expansions.is_empty());
        assert!(expansions.contains(&"明天早上".to_string()));
    }

    #[test]
    fn test_has_expansion() {
        let expander = FastTextExpander::new();

        assert!(expander.has_expansion("明早"));
        assert!(!expander.has_expansion("不存在的词"));
    }

    #[test]
    fn test_threshold() {
        let expander = FastTextExpander::new().with_threshold(0.9);
        assert_eq!(expander.similarity_threshold, 0.9);
    }
}
