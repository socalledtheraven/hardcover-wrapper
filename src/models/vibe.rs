//! Vibe model representing mood-based and algorithmic book recommendations.

use crate::utils::base_hardcover_item::BaseHardcoverItem;
use crate::utils::date_parsing;
use crate::utils::enums::{PrivacySetting, RecommendationType, VibeType};
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

/// Represents a vibe (curated or dynamic mood/theme recommendation list).
#[derive(Debug, Clone, Deserialize)]
pub struct Vibe {
    /// Timestamp when recommendations were generated.
    #[serde(deserialize_with = "date_parsing::plain_datetime_optional")]
    pub books_generated_at: Option<PlainDateTime>,
    /// Creation timestamp.
    #[serde(deserialize_with = "date_parsing::plain_datetime")]
    pub created_at: PlainDateTime,
    /// Description of the vibe.
    pub description: Option<String>,
    /// Whether this vibe is featured.
    pub featured: bool,
    /// Timestamp when featured.
    #[serde(deserialize_with = "date_parsing::plain_datetime_optional")]
    pub featured_at: Option<PlainDateTime>,
    /// Unique identifier for the vibe.
    pub id: u64,
    /// Number of likes received by this vibe.
    pub likes_count: u64,
    /// GraphQL object type name.
    pub object_type: String,
    /// Privacy visibility setting.
    pub privacy_setting_id: PrivacySetting,
    /// Type of recommendations produced.
    pub result_type: RecommendationType,
    /// URL slug for the vibe.
    pub slug: String,
    /// Title of the vibe.
    pub title: String,
    /// Last update timestamp.
    #[serde(deserialize_with = "date_parsing::plain_datetime")]
    pub updated_at: PlainDateTime,
    /// User ID of the creator.
    pub user_id: u64,
    /// Category / mechanism of the vibe.
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

        let variables = serde_json::json!({ "id": id });
        let data = Self::from_data(query, variables, client).await?;

        Ok(Self::from_value(data["vibes_by_pk"].clone()))
    }

    fn from_value(data: Value) -> Self {
        serde_json::from_value(data).unwrap()
    }
}
