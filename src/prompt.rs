use serde_json::Value;
use time::OffsetDateTime;
use crate::base_hardcover_item::BaseHardcoverItem;
use crate::client::{GraphQLResponse};

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

#[derive(Debug, Clone)]
pub struct Prompt {
    pub answers_count: u64,
    pub books_count: u64,
    pub created_at: Option<OffsetDateTime>,
    pub description: String,
    pub featured: bool,
    pub id: u64,
    pub privacy_setting_id: u64,
    pub question: String,
    pub slug: String,
    pub user_id: u64,
    pub users_count: u64,
}

impl BaseHardcoverItem for Prompt {
    async fn from_id(id: u64) -> Result<Self, reqwest::Error> {
        let query = r#"
        query GetPrompt($id: Int!) {
          prompts_by_pk(id: $id) {"#.to_string() + QUERY_FIELDS + r#"
          }
        }
        "#;

        let data = Self::from_data(query, id).await?;

        Ok(Self::new(data["prompts_by_pk"].clone()))
    }

    fn new(data: Value) -> Self {
        Prompt {
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
                data.get_bool("featured").unwrap()
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
        }
    }
}