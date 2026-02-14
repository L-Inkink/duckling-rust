mod pattern_normalizer;
mod levenshtein;

#[cfg(feature = "fasttext")]
mod expand;

#[cfg(feature = "fasttext")]
mod model;

pub use pattern_normalizer::PatternNormalizer;
pub use levenshtein::LevenshteinMatcher;

#[cfg(feature = "fasttext")]
pub use expand::FastTextExpander;

#[cfg(feature = "fasttext")]
pub use model::{ModelManager, ModelRegistry, ModelInfo, ModelError};
