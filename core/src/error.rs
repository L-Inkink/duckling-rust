use thiserror::Error;

#[derive(Error, Debug)]
pub enum RustlingError {
    #[error("Rule error: {0}")]
    Rule(String),

    #[error("Parse error: {0}")]
    Parse(String),

    #[error("Regex error: {0}")]
    Regex(#[from] regex::Error),

    #[error("Invalid pattern: {0}")]
    InvalidPattern(String),

    #[error("No capture found: {0}")]
    NoCapture(String),

    #[error("{0}")]
    Other(String),
}

pub type Result<T> = std::result::Result<T, RustlingError>;

// Helper macro to create errors easily
#[macro_export]
macro_rules! rustling_error {
    ($($arg:tt)*) => {
        $crate::error::RustlingError::Other(format!($($arg)*))
    };
}
