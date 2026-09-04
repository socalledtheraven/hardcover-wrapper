use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use crate::graphql::{graphql_req, GraphQLResponse};

const QUERY_FIELDS: &str = r#"
answers_count
books_count
created_at
description
featured
id
privacy_setting_id
question
slug
user_id
users_count
"#;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct Prompt {
    answers_count: u64,
    books_count: u64,
    created_at: Option<OffsetDateTime>,
    description: String,
    featured: bool,
    id: u64,
    privacy_setting_id: u64,
    question: String,
    slug: String,
    user_id: u64,
    users_count: u64,
}

impl Prompt {
    pub(crate) async fn from_prompt_id(prompt_id: u64) -> Result<Self, reqwest::Error> {
        let query = r#"
        query GetAuthor($id: Int!) {
          prompts_by_pk(id: $id) {"#.to_string() + QUERY_FIELDS + r#"
          }
        }
        "#;

        Self::from_data(query, prompt_id).await
    }

    async fn from_data<T: ToString>(query: String, user_data: T) -> Result<Self, reqwest::Error> {
        let mut vars = HashMap::new();
        vars.insert("id", user_data.to_string());

        Self::new(query, vars).await
    }

    async fn new(query: String, vars: HashMap<&str, String>) -> Result<Self, reqwest::Error> {
        let resp = graphql_req(query, vars).await?;

        let data = &resp["data"]["prompts_by_pk"];

        Ok(Prompt{
            answers_count: {
                data.get_u64("answers_count").unwrap()
            },
            books_count: {
                data.get_u64("books_count").unwrap()
            },
            created_at: {
                data.get_offsetdt("created_at")
            },
            description: {
                data.get_str("description").unwrap()
            },
            featured: {
                data.get_bool("featured")
            },
            id: {
                data.get_u64("id").unwrap()
            },
            privacy_setting_id: {
                data.get_u64("privacy_setting_id").unwrap()
            },
            question: {
                data.get_str("question").unwrap()
            },
            slug: {
                data.get_str("slug").unwrap()
            },
            user_id: {
                data.get_u64("user_id").unwrap()
            },
            users_count: {
                data.get_u64("users_count").unwrap()
            },
        })
    }
}