//! Prompt model representing reading challenges and community discussion questions.

use crate::utils::base_hardcover_item::BaseHardcoverItem;
use crate::utils::date_parsing;
use crate::HardcoverClient;
use serde::Deserialize;
use serde_json::Value;
use time::OffsetDateTime;

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

/// Represents a reading prompt or community question.
#[derive(Debug, Clone, Deserialize)]
pub struct Prompt {
    /// Number of answers/responses submitted.
    pub answers_count: u64,
    /// Number of books associated with this prompt.
    pub books_count: u64,
    /// Creation timestamp.
    #[serde(deserialize_with = "date_parsing::offset_datetime_optional")]
    pub created_at: Option<OffsetDateTime>,
    /// Detailed description of the prompt.
    pub description: String,
    /// Whether this prompt is featured on the platform.
    pub featured: bool,
    /// Unique identifier for the prompt.
    pub id: u64,
    /// Privacy setting ID.
    pub privacy_setting_id: u64,
    /// Prompt question text.
    pub question: String,
    /// URL slug for the prompt page.
    pub slug: String,
    /// User ID of the prompt author.
    pub user_id: u64,
    /// Number of participating users.
    pub users_count: u64,
}

impl BaseHardcoverItem for Prompt {
    async fn from_id(id: u64, client: &HardcoverClient) -> Result<Self, reqwest::Error> {
        let query = r#"
        query GetPrompt($id: Int!) {
          prompts_by_pk(id: $id) {"#
            .to_string()
            + QUERY_FIELDS
            + r#"
          }
        }
        "#;

        let variables = serde_json::json!({ "id": id });
        let data = Self::from_data(query, variables, client).await?;

        Ok(Self::from_value(data["prompts_by_pk"].clone()))
    }

    fn from_value(data: Value) -> Self {
        serde_json::from_value(data).unwrap()
    }
}
