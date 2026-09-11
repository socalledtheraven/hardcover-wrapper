//! Platform model representing external data sources and import services.

use serde::Deserialize;
use serde_json::Value;

/// Represents an external book service or platform (e.g. Goodreads, StoryGraph).
#[derive(Debug, Clone, Deserialize)]
pub struct Platform {
    /// Unique identifier for the platform.
    pub id: u64,
    /// Name of the platform.
    pub name: String,
    /// Web URL of the platform.
    pub url: Option<String>,
}

impl Platform {
    /// Creates a `Platform` instance from a raw JSON value.
    pub fn new(resp: Value) -> Self {
        serde_json::from_value(resp).unwrap()
    }
}
