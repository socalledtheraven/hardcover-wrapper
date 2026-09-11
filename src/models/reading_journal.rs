//! Reading journal model representing user reading log entries, sessions, and timestamps.

use crate::utils::base_hardcover_item::BaseHardcoverItem;
use crate::utils::date_parsing;
use crate::HardcoverClient;
use serde::Deserialize;
use serde_json::Value;
use time::{Date, OffsetDateTime, PlainDateTime};

const QUERY_FIELDS: &str = r#"
action_at
book_id
created_at
edition_id
entry
event
id
journal_date
likes_count
metadata
object_type
privacy_setting_id
updated_at
user_id
"#;

/// Represents a reading log or journal session entry.
#[derive(Debug, Clone, Deserialize)]
pub struct ReadingJournal {
    /// Timestamp when the reading action occurred.
    #[serde(deserialize_with = "date_parsing::offset_datetime")]
    pub action_at: OffsetDateTime,
    /// Associated book ID.
    pub book_id: Option<u64>,
    /// Creation timestamp.
    #[serde(deserialize_with = "date_parsing::plain_datetime")]
    pub created_at: PlainDateTime,
    /// Associated edition ID.
    pub edition_id: Option<u64>,
    /// Journal entry text or notes.
    pub entry: Option<String>,
    /// Event descriptor string.
    pub event: Option<String>,
    /// Unique identifier for the reading journal entry.
    pub id: u64,
    /// Date of the reading journal entry.
    #[serde(deserialize_with = "date_parsing::date_optional")]
    pub journal_date: Option<Date>,
    /// Number of likes received by this journal entry.
    pub likes_count: u64,
    /// Extra metadata payload.
    pub metadata: Value,
    /// GraphQL object type name.
    pub object_type: String,
    /// Privacy visibility setting ID.
    pub privacy_setting_id: u64,
    /// Last update timestamp.
    #[serde(deserialize_with = "date_parsing::plain_datetime")]
    pub updated_at: PlainDateTime,
    /// User ID of the journal entry author.
    pub user_id: Option<u64>,
}

impl BaseHardcoverItem for ReadingJournal {
    async fn from_id(id: u64, client: &HardcoverClient) -> Result<Self, reqwest::Error> {
        let query = r#"
        query GetReadingJournal($id: bigint!) {
          reading_journals_by_pk(id: $id) {"#
            .to_string()
            + QUERY_FIELDS
            + r#"
          }
        }
        "#;

        let variables = serde_json::json!({ "id": id });
        let data = Self::from_data(query, variables, client).await?;

        Ok(Self::from_value(data["reading_journals_by_pk"].clone()))
    }

    fn from_value(data: Value) -> Self {
        serde_json::from_value(data).unwrap()
    }
}
