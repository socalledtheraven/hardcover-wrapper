use crate::base_hardcover_item::BaseHardcoverItem;
use crate::date_parsing;
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

#[derive(Debug, Clone, Deserialize)]
pub struct ReadingJournal {
    #[serde(deserialize_with = "date_parsing::offset_datetime")]
    pub action_at: OffsetDateTime,
    pub book_id: Option<u64>,
    #[serde(deserialize_with = "date_parsing::plain_datetime")]
    pub created_at: PlainDateTime,
    pub edition_id: Option<u64>,
    pub entry: Option<String>,
    pub event: Option<String>,
    pub id: u64,
    #[serde(deserialize_with = "date_parsing::date_optional")]
    pub journal_date: Option<Date>,
    pub likes_count: u64,
    pub metadata: Value,
    pub object_type: String,
    pub privacy_setting_id: u64,
    #[serde(deserialize_with = "date_parsing::plain_datetime")]
    pub updated_at: PlainDateTime,
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

        let data = Self::from_data(query, id, client).await?;

        Ok(Self::from_value(data["reading_journals_by_pk"].clone()))
    }

    fn from_value(data: Value) -> Self {
        serde_json::from_value(data).unwrap()
    }
}
