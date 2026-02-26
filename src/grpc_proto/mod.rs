// Auto-generated from proto/duckling.proto
// DO NOT EDIT

use serde::{Deserialize, Serialize};

/// Parse request for a single text
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParseRequest {
    /// Text to parse
    pub text: String,
    /// BCP-47 locale code, e.g. "en", "fr", "zh"
    pub locale: String,
}

/// Parse response with results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParseResponse {
    /// Parse results
    pub results: Vec<ParseResult>,
    /// Number of results
    pub count: u32,
}

/// Batch parse request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchParseRequest {
    /// List of texts to parse (max 100 items)
    pub texts: Vec<String>,
    /// BCP-47 locale code
    pub locale: String,
}

/// Batch parse response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchParseResponse {
    /// Batch parse results
    pub results: Vec<BatchParseResult>,
    /// Total number of results
    pub total_count: u32,
}

/// Single batch result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchParseResult {
    /// Index of the text in the request
    pub index: u32,
    /// Original input text
    pub text: String,
    /// Parse results for this text
    pub results: Vec<ParseResult>,
    /// Number of results for this text
    pub count: u32,
}

/// Single parse result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParseResult {
    /// Parsed value representation
    pub value: String,
    /// Byte start position
    pub byte_start: u32,
    /// Byte end position
    pub byte_end: u32,
    /// Character start position
    pub char_start: u32,
    /// Character end position
    pub char_end: u32,
}

/// Health check request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthRequest {}

/// Health check response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthResponse {
    /// Server version
    pub version: String,
    /// Whether server is healthy
    pub healthy: bool,
}
