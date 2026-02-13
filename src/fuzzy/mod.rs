mod pattern_normalizer;
mod levenshtein;

#[cfg(feature = "fasttext")]
mod expand;

pub use pattern_normalizer::PatternNormalizer;
pub use levenshtein::LevenshteinMatcher;

#[cfg(feature = "fasttext")]
pub use expand::FastTextExpander;
