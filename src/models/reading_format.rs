//! Reading format model representing Hardcover's reading format entities.

use serde::Deserialize;

/// Represents a reading format database entity.
#[derive(Debug, Clone, Deserialize)]
pub struct ReadingFormat {
    /// Unique identifier for the reading format.
    pub id: u64,
    /// Format descriptor string (e.g. "Physical", "Audiobook", "Ebook").
    pub format: String,
}
