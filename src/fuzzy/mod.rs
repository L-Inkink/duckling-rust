mod pattern_normalizer;
mod levenshtein;
mod smart_matcher;

#[cfg(feature = "fasttext")]
mod expand;

#[cfg(feature = "fasttext")]
mod model;

pub use pattern_normalizer::PatternNormalizer;
pub use levenshtein::LevenshteinMatcher;
pub use smart_matcher::{SmartMatcher, MatcherConfig};

#[cfg(feature = "fasttext")]
pub use expand::FastTextExpander;

#[cfg(feature = "fasttext")]
pub use model::{ModelManager, ModelRegistry, ModelInfo, ModelError};
