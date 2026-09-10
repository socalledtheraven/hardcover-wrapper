use crate::date_parsing;
use crate::base_hardcover_item::BaseHardcoverItem;
use crate::util::{PrivacySetting, RecommendationType, VibeType};
use crate::HardcoverClient;
use reqwest::Error;
use serde::Deserialize;
use serde_json::Value;
use time::PlainDateTime;

const QUERY_FIELDS: &str = r#"
books_generated_at
created_at
description
featured
featured_at
id
likes_count
object_type
privacy_setting_id
result_type
slug
title
updated_at
user_id
vibe_type
"#;

#[derive(Debug, Clone, Deserialize)]
pub struct Vibe {
    #[serde(deserialize_with = "date_parsing::plain_datetime_optional")]
    pub books_generated_at: Option<PlainDateTime>,
    #[serde(deserialize_with = "date_parsing::plain_datetime")]
    pub created_at: PlainDateTime,
    pub description: Option<String>,
    pub featured: bool,
    #[serde(deserialize_with = "date_parsing::plain_datetime_optional")]
    pub featured_at: Option<PlainDateTime>,
    pub id: u64,
    pub likes_count: u64,
    pub object_type: String,
    pub privacy_setting_id: PrivacySetting,
    pub result_type: RecommendationType,
    pub slug: String,
    pub title: String,
    #[serde(deserialize_with = "date_parsing::plain_datetime")]
    pub updated_at: PlainDateTime,
    pub user_id: u64,
    pub vibe_type: VibeType,
}

impl BaseHardcoverItem for Vibe {
    async fn from_id(id: u64, client: &HardcoverClient) -> Result<Self, Error> {
        let query = r#"
        query GetVibe($id: Int!) {
          vibes_by_pk(id: $id) {"#
            .to_string()
            + QUERY_FIELDS
            + r#"
          }
        }
        "#;

        let data = Self::from_data(query, id, client).await?;

        Ok(Self::new(data["vibes_by_pk"].clone()))
    }

    fn new(data: Value) -> Self {
        serde_json::from_value(data).unwrap()
    }
}
