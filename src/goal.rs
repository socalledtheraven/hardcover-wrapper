use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use time::{Date, OffsetDateTime};
use crate::graphql::{graphql_req, GraphQLResponse};

const QUERY_FIELDS: &str = r#"
archived
completed_at
conditions
description
end_date
goal
id
metric
privacy_setting_id
progress
start_date
state
user_id
"#;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct Goal {
    archived: bool,
    completed_at: Option<OffsetDateTime>,
    conditions: Value,
    description: Option<String>,
    end_date: Date,
    goal: u64,
    id: u64,
    metric: String,
    privacy_setting_id: Option<u64>,
    progress: f64,
    start_date: Date,
    state: String,
    user_id: u64,
}

impl Goal {
    pub(crate) async fn from_id(id: u64) -> Result<Self, reqwest::Error> {
        let query = r#"
        query GetAuthor($id: Int!) {
          goals(where: {id: {_eq: $id}}, limit: 1) {"#.to_string() + QUERY_FIELDS + r#"
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

        let data = &resp["data"]["goals"][0];

        Ok(Goal{
            archived: {
                data.get_bool("archived")
            },
            completed_at: {
                data.get_offsetdt("completed_at")
            },
            conditions: {
                data["conditions"].clone()
            },
            description: {
                data.get_str("description")
            },
            end_date: {
                data.get_date("end_date").unwrap()
            },
            goal: {
                data.get_u64("goal").unwrap()
            },
            id: {
                data.get_u64("id").unwrap()
            },
            metric: {
                data.get_str("metric").unwrap()
            },
            privacy_setting_id: {
                data.get_u64("privacy_setting_id")
            },
            progress: {
                data.get_f64("progress").unwrap()
            },
            start_date: {
                data.get_date("start_date").unwrap()
            },
            state: {
                data.get_str("state").unwrap()
            },
            user_id: {
                data.get_u64("user_id").unwrap()
            }
        })
    }
}