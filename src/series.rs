use crate::base_hardcover_item::BaseHardcoverItem;
use crate::util::Identifiers;
use crate::HardcoverClient;
use serde::Deserialize;
use serde_json::Value;

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

#[derive(Debug, Clone, Deserialize)]
pub struct Series {
    pub author_id: Option<u64>,
    pub books_count: u64,
    pub canonical_id: Option<u64>,
    pub description: Option<String>,
    pub id: u64,
    pub identifiers: Identifiers,
    pub is_completed: Option<bool>,
    pub locked: bool,
    pub name: String,
    pub object_type: String,
    pub primary_books_count: Option<u64>,
    pub slug: String,
    pub state: String,
    pub user_id: Option<u64>,
}

impl BaseHardcoverItem for Series {
    async fn from_id(id: u64, client: &HardcoverClient) -> Result<Self, reqwest::Error> {
        let query = r#"
        query GetSeries($id: Int!) {
          series_by_pk(id: $id) {"#
            .to_string()
            + QUERY_FIELDS
            + r#"
          }
        }
        "#;

        let data = Self::from_data(query, id, client).await?;

        Ok(Self::from_value(data["series_by_pk"].clone()))
    }

    fn from_value(data: Value) -> Self {
        serde_json::from_value(data).unwrap()
    }
}
