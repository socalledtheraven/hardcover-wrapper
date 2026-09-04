use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use time::PlainDateTime;
use crate::graphql::{graphql_req, GraphQLResponse};

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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct Publisher {
    canonical_id: Option<u64>,
    created_at: PlainDateTime,
    editions_count: u64,
    id: u64,
    locked: bool,
    name: Option<String>,
    object_type: String,
    parent_id: Option<u64>,
    slug: String,
    state: String,
    updated_at: PlainDateTime,
    user_id: Option<u64>,
}

impl Publisher {
    pub(crate) async fn from_publisher_id(publisher_id: u64) -> Result<Self, reqwest::Error> {
        let query = r#"
        query GetAuthor($id: bigint!) {
          publishers_by_pk(id: $id) {"#.to_string() + QUERY_FIELDS + r#"
          }
        }
        "#;

        Self::from_data(query, publisher_id).await
    }

    async fn from_data<T: ToString>(query: String, user_data: T) -> Result<Self, reqwest::Error> {
        let mut vars = HashMap::new();
        vars.insert("id", user_data.to_string());

        Self::new(query, vars).await
    }

    async fn new(query: String, vars: HashMap<&str, String>) -> Result<Self, reqwest::Error> {
        let resp = graphql_req(query, vars).await?;

        let data = &resp["data"]["publishers_by_pk"];

        Ok(Publisher{
            canonical_id: {
                data.get_u64("canonical_id")
            },
            created_at: {
                data.get_plaindt("created_at").unwrap()
            },
            editions_count: {
                data.get_u64("editions_count").unwrap()
            },
            id: {
                data.get_u64("id").unwrap()
            },
            locked: {
                data.get_bool("locked")
            },
            name: {
                data.get_str("name")
            },
            object_type: {
                data.get_str("object_type").unwrap()
            },
            parent_id: {
                data.get_u64("parent_id")
            },
            slug: {
                data.get_str("slug").unwrap()
            },
            state: {
                data.get_str("state").unwrap()
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