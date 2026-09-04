use std::collections::HashMap;
use serde_json::Value;
use time::{Date, OffsetDateTime, PlainDateTime};
use crate::graphql::{graphql_req, GraphQLResponse};

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

impl ReadingJournal {
    pub(crate) async fn from_id(id: u64) -> Result<Self, reqwest::Error> {
        let query = r#"
        query GetAuthor($id: bigint!) {
          reading_journals_by_pk(id: $id) {"#.to_string() + QUERY_FIELDS + r#"
          }
        }
        "#;

        Self::from_data(query, id).await
    }

    async fn from_data<T: ToString>(query: String, user_data: T) -> Result<Self, reqwest::Error> {
        let mut vars = HashMap::new();
        vars.insert("id", user_data.to_string());

        Self::new(query, vars).await
    }

    async fn new(query: String, vars: HashMap<&str, String>) -> Result<Self, reqwest::Error> {
        let resp = graphql_req(query, vars).await?;

        let data = &resp["data"]["reading_journals_by_pk"];

        Ok(ReadingJournal{
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
        })
    }
}