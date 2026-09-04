use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use time::OffsetDateTime;
use crate::graphql::{graphql_req, GraphQLResponse};

const QUERY_FIELDS: &str = r#"
created_at
id
likeable_id
likeable_type
user_id
"#;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct Like {
    created_at: Option<OffsetDateTime>,
    id: u64,
    likeable_id: u64,
    likeable_type: String,
    user_id: u64,
}

impl Like {
    pub(crate) async fn from_id(id: u64) -> Result<Self, reqwest::Error> {
        let query = r#"
        query GetAuthor($id: Int!) {
          likes(where: {id: {_eq: $id}}, limit: 1) {"#.to_string() + QUERY_FIELDS + r#"
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

    async fn new(query: String, vars: HashMap<&str, String>) -> Result<Self, reqwest::Error>  {
        let resp = graphql_req(query, vars).await?;

        let data = &resp["data"]["likes"][0];

        Ok(Self::from_response(data.clone()))
    }

    pub(crate) fn from_response(resp: Value) -> Self {
        Like {
            created_at: resp.get_offsetdt("created_at"),
            id: resp.get_u64("id").unwrap(),
            likeable_id: resp.get_u64("likeable_id").unwrap(),
            likeable_type: resp.get_str("likeable_type").unwrap().to_string(),
            user_id: resp.get_u64("user_id").unwrap(),
        }
    }
}