//! Publisher model representing book publishing companies and imprints.

use crate::utils::base_hardcover_item::BaseHardcoverItem;
use crate::utils::date_parsing;
use crate::HardcoverClient;
use serde::Deserialize;
use serde_json::Value;
use time::PlainDateTime;

const QUERY_FIELDS: &str = r#"
canonical_id
created_at
editions_count
id
locked
name
object_type
parent_id
slug
state
updated_at
user_id
"#;

/// Represents a book publisher or imprint.
#[derive(Debug, Clone, Deserialize)]
pub struct Publisher {
    /// Canonical publisher ID if merged.
    pub canonical_id: Option<u64>,
    /// Creation timestamp.
    #[serde(deserialize_with = "date_parsing::plain_datetime")]
    pub created_at: PlainDateTime,
    /// Total number of editions released by this publisher.
    pub editions_count: u64,
    /// Unique identifier for the publisher.
    pub id: u64,
    /// Whether this record is locked against community edits.
    pub locked: bool,
    /// Name of the publishing house or imprint.
    pub name: Option<String>,
    /// GraphQL object type name.
    pub object_type: String,
    /// Parent publisher ID if this is an imprint or subsidiary.
    pub parent_id: Option<u64>,
    /// URL slug for the publisher page.
    pub slug: String,
    /// Record state.
    pub state: String,
    /// Last update timestamp.
    #[serde(deserialize_with = "date_parsing::plain_datetime")]
    pub updated_at: PlainDateTime,
    /// Contributor user ID who created the publisher record.
    pub user_id: Option<u64>,
}

impl BaseHardcoverItem for Publisher {
    async fn from_id(id: u64, client: &HardcoverClient) -> Result<Self, reqwest::Error> {
        let query = r#"
        query GetPublisher($id: bigint!) {
          publishers_by_pk(id: $id) {"#
            .to_string()
            + QUERY_FIELDS
            + r#"
          }
        }
        "#;

        let variables = serde_json::json!({ "id": id });
        let data = Self::from_data(query, variables, client).await?;

        Ok(Self::from_value(data["publishers_by_pk"].clone()))
    }

    fn from_value(data: Value) -> Self {
        serde_json::from_value(data).unwrap()
    }
}
