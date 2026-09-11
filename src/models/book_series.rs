//! Relationship model connecting books to series and their reading order position.

use crate::utils::base_hardcover_item::BaseHardcoverItem;
use crate::utils::date_parsing;
use crate::HardcoverClient;
use serde::Deserialize;
use serde_json::Value;
use time::PlainDateTime;

const QUERY_FIELDS: &str = r#"
book_id
compilation
created_at
details
featured
id
position
series_id
updated_at
"#;

/// Represents the relationship linking a book to a series, including its ordinal position.
#[derive(Debug, Clone, Deserialize)]
pub struct BookSeries {
    /// ID of the book.
    pub book_id: u64,
    /// Whether this entry represents a compilation in the series.
    pub compilation: bool,
    /// Creation timestamp.
    #[serde(deserialize_with = "date_parsing::plain_datetime")]
    pub created_at: PlainDateTime,
    /// Additional notes or details regarding this entry's place in the series.
    pub details: String,
    /// Whether this is a primary/featured series entry for the book.
    pub featured: bool,
    /// Unique identifier for this book-series relationship.
    pub id: u64,
    /// Numeric position / order in the series (e.g. 1.0, 1.5, 2.0).
    pub position: f64,
    /// ID of the parent series.
    pub series_id: u64,
    /// Last update timestamp.
    #[serde(deserialize_with = "date_parsing::plain_datetime")]
    pub updated_at: PlainDateTime,
}

impl BaseHardcoverItem for BookSeries {
    async fn from_id(id: u64, client: &HardcoverClient) -> Result<Self, reqwest::Error> {
        let query = r#"
        query GetBookSeries($id: Int!) {
          book_series_by_pk(id: $id) {"#
            .to_string()
            + QUERY_FIELDS
            + r#"
          }
        }
        "#;

        let variables = serde_json::json!({ "id": id });
        let data = Self::from_data(query, variables, client).await?;

        Ok(Self::from_value(data["book_series_by_pk"].clone()))
    }

    fn from_value(data: Value) -> Self {
        serde_json::from_value(data).unwrap()
    }
}
