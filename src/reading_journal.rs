use serde_json::Value;
use time::{Date, PlainDateTime};
use crate::base_hardcover_item::BaseHardcoverItem;
use crate::client::{GraphQLResponse};

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
pub struct ReadingJournal {
    pub action_at: PlainDateTime,
    pub book_id: Option<u64>,
    pub created_at: PlainDateTime,
    pub edition_id: Option<u64>,
    pub entry: Option<String>,
    pub event: Option<String>,
    pub id: u64,
    pub journal_date: Option<Date>,
    pub likes_count: u64,
    pub metadata: Value,
    pub object_type: String,
    pub privacy_setting_id: u64,
    pub updated_at: PlainDateTime,
    pub user_id: Option<u64>,
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

        println!("Data {:#?}", data);

        Ok(Self::new(data["reading_journals_by_pk"].clone()))
    }

    fn new(data: Value) -> Self {
        ReadingJournal {
            action_at: {
                data.get_plaindt("action_at").unwrap()
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