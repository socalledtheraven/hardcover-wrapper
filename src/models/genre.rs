//! Genre model representing literary categories and tag associations.

use serde::Deserialize;

/// Represents a genre or tag along with book counts.
#[derive(Debug, Clone, Deserialize)]
pub struct Genre {
    /// Number of books categorized under this genre.
    pub count: u64,
    /// Display name of the genre tag.
    pub tag: String,
    /// URL slug for the genre tag.
    pub tag_slug: String,
}
