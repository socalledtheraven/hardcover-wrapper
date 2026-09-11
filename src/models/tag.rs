//! Tag model representing user-created and algorithmic categorization tags.

use crate::utils::base_hardcover_item::BaseHardcoverItem;
use crate::HardcoverClient;
use reqwest::Error;
use serde::Deserialize;
use serde_json::Value;

const QUERY_FIELDS: &str = r#"
    count
    id
    slug
    tag
    tag_category_id
"#;

/// Represents a tag label applied to books or lists.
#[derive(Debug, Clone, Deserialize)]
pub struct Tag {
    /// Total count of items associated with this tag.
    pub count: u64,
    /// Unique identifier for the tag.
    pub id: u64,
    /// URL slug for the tag.
    pub slug: String,
    /// Tag text / name.
    pub tag: String,
    /// Tag category ID.
    pub tag_category_id: u64,
}

impl BaseHardcoverItem for Tag {
    async fn from_id(id: u64, client: &HardcoverClient) -> Result<Self, Error> {
        let query = r#"
        query GetTag($id: bigint!) {
          tags_by_pk(id: $id) {"#
            .to_string()
            + QUERY_FIELDS
            + r#"
          }
        }
        "#;

        let variables = serde_json::json!({ "id": id });
        let data = Self::from_data(query, variables, client).await?;

        Ok(Self::from_value(data["tags_by_pk"].clone()))
    }

    fn from_value(data: Value) -> Self {
        serde_json::from_value(data).unwrap()
    }
}
