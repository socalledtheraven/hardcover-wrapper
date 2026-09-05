use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::base_hardcover_item::BaseHardcoverItem;
use crate::graphql::GraphQLResponse;

const QUERY_FIELDS: &str = r#"
author_id
books_count
canonical_id
description
id
identifiers
is_completed
locked
name
object_type
primary_books_count
slug
state
user_id
"#;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct Series {
    author_id: Option<u64>,
    books_count: u64,
    canonical_id: Option<u64>,
    description: Option<String>,
    id: u64,
    identifiers: Value,
    is_completed: Option<bool>,
    locked: bool,
    name: String,
    object_type: String,
    primary_books_count: Option<u64>,
    slug: String,
    state: String,
    user_id: Option<u64>,
}

impl BaseHardcoverItem for Series {
    async fn from_id(id: u64) -> Result<Self, reqwest::Error> {
        let query = r#"
        query GetSeries($id: Int!) {
          series_by_pk(id: $id) {"#.to_string() + QUERY_FIELDS + r#"
          }
        }
        "#;

        let data = Self::from_data(query, id).await?;

        Ok(Self::new(data["series_by_pk"].clone()))
    }

    fn new(data: Value) -> Self {
        Series {
            author_id: {
                data.get_u64("author_id")
            },
            books_count: {
                data.get_u64("books_count").unwrap()
            },
            canonical_id: {
                data.get_u64("canonical_id")
            },
            description: {
                data.get_str("description")
            },
            id: {
                data.get_u64("id").unwrap()
            },
            identifiers: {
                data.get("identifiers").unwrap().clone()
            },
            is_completed: {
                data.as_bool()
            },
            locked: {
                data.get_bool("locked")
            },
            name: {
                data.get_str("name").unwrap()
            },
            object_type: {
                data.get_str("object_type").unwrap()
            },
            primary_books_count: {
                data.get_u64("primary_books_count")
            },
            slug: {
                data.get_str("slug").unwrap()
            },
            state: {
                data.get_str("state").unwrap()
            },
            user_id: {
                data.get_u64("user_id")
            },
        }
    }
}