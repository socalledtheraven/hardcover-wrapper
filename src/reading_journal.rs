use serde_json::Value;
use time::{Date, OffsetDateTime, PlainDateTime};
use crate::base_hardcover_item::BaseHardcoverItem;
use crate::graphql::{GraphQLResponse};

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

#[derive(Debug)]
pub(crate) struct ReadingJournal {
    action_at: OffsetDateTime,
    book_id: Option<u64>,
    created_at: PlainDateTime,
    edition_id: Option<u64>,
    entry: Option<String>,
    event: Option<String>,
    id: u64,
    journal_date: Option<Date>,
    likes_count: u64,
    metadata: Value,
    object_type: String,
    privacy_setting_id: u64,
    updated_at: PlainDateTime,
    user_id: Option<u64>,
}

impl BaseHardcoverItem for ReadingJournal {
    async fn from_id(id: u64) -> Result<Self, reqwest::Error> {
        let query = r#"
        query GetReadingJournal($id: bigint!) {
          reading_journals_by_pk(id: $id) {"#.to_string() + QUERY_FIELDS + r#"
          }
        }
        "#;

        let data = Self::from_data(query, id).await?;

        Ok(Self::new(data["reading_journals_by_pk"][0].clone()))
    }

    fn new(data: Value) -> Self {
        ReadingJournal {
            action_at: {
                data.get_offsetdt("action_at").unwrap()
            },
            book_id: {
                data.get_u64("book_id")
            },
            created_at: {
                data.get_plaindt("created_at").unwrap()
            },
            edition_id: {
                data.get_u64("edition_id")
            },
            entry: {
                data.get_str("entry")
            },
            event: {
                data.get_str("event")
            },
            id: {
                data.get_u64("id").unwrap()
            },
            journal_date: {
                data.get_date("journal_date")
            },
            likes_count: {
                data.get_u64("likes_count").unwrap()
            },
            metadata: {
                data["metadata"].clone()
            },
            object_type: {
                data.get_str("object_type").unwrap()
            },
            privacy_setting_id: {
                data.get_u64("privacy_setting_id").unwrap()
            },
            updated_at: {
                data.get_plaindt("updated_at").unwrap()
            },
            user_id: {
                data.get_u64("user_id")
            },
        }
    }
}