//! Like model representing user likes/reactions on items.

use crate::utils::base_hardcover_item::BaseHardcoverItem;
use crate::utils::date_parsing;
use crate::HardcoverClient;
use serde::Deserialize;
use serde_json::Value;
use time::OffsetDateTime;

const QUERY_FIELDS: &str = r#"
created_at
id
likeable_id
likeable_type
user_id
"#;

/// Represents a user like on an activity, list, or journal entry.
#[derive(Debug, Clone, Deserialize)]
pub struct Like {
    /// Timestamp when the item was liked.
    #[serde(deserialize_with = "date_parsing::offset_datetime_optional")]
    pub created_at: Option<OffsetDateTime>,
    /// Unique identifier for the like record.
    pub id: u64,
    /// ID of the liked entity.
    pub likeable_id: u64,
    /// Type string of the liked entity.
    pub likeable_type: String,
    /// User ID of the user who liked the entity.
    pub user_id: u64,
}

impl BaseHardcoverItem for Like {
    async fn from_id(id: u64, client: &HardcoverClient) -> Result<Self, reqwest::Error> {
        let query = r#"
        query GetLike($id: Int!) {
          likes_by_pk(id: $id) {"#
            .to_string()
            + QUERY_FIELDS
            + r#"
          }
        }
        "#;

        let variables = serde_json::json!({ "id": id });
        let data = Self::from_data(query, variables, client).await?;

        Ok(Self::from_value(data["likes_by_pk"].clone()))
    }

    fn from_value(resp: Value) -> Self {
        serde_json::from_value(resp).unwrap()
    }
}
