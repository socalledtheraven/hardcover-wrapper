//! Image model representing uploaded assets, color palettes, and dimensions.

use serde::Deserialize;
use serde_json::Value;

/// Entity type to which an image is attached.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum Imageable {
    Author,
    Book,
    Edition,
}

/// Represents an image resource stored on Hardcover.
#[derive(Debug, Clone, Deserialize)]
pub struct Image {
    /// Dominant hex color code of the image.
    pub color: Option<String>,
    /// Palette of extracted color hex codes.
    pub colors: Option<Vec<String>>,
    /// Descriptive name of the dominant color.
    pub color_name: Option<String>,
    /// Pixel height of the image.
    pub height: Option<u64>,
    /// Unique identifier for the image.
    pub id: u64,
    /// ID of the entity that owns this image.
    pub imageable_id: Option<u64>,
    /// Type of the owning entity.
    pub imageable_type: Option<Imageable>,
    /// Aspect ratio (width / height).
    pub ratio: Option<f64>,
    /// Public URL of the image.
    pub url: Option<String>,
    /// Pixel width of the image.
    pub width: Option<u64>,
}

impl Image {
    /// Creates an `Image` instance from a raw JSON value.
    pub fn new(resp: Value) -> Self {
        serde_json::from_value(resp).unwrap()
    }
}
