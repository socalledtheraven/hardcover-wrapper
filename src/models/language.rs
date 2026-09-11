//! Language model representing linguistic codes and names.

use serde::Deserialize;
use serde_json::Value;

/// Represents a language entry in Hardcover.
#[derive(Debug, Clone, Deserialize)]
pub struct Language {
    /// Two-letter language code (ISO 639-1).
    pub code2: Option<String>,
    /// Three-letter language code (ISO 639-2/3).
    pub code3: Option<String>,
    /// Unique identifier for the language.
    pub id: u64,
    /// Name of the language.
    pub language: String,
}

impl Language {
    /// Creates a `Language` instance from a raw JSON value.
    pub fn new(resp: Value) -> Self {
        serde_json::from_value(resp).unwrap()
    }
}
