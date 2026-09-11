//! List model representing user-curated book collections and reading lists.

use crate::utils::base_hardcover_item::BaseHardcoverItem;
use crate::utils::date_parsing;
use crate::utils::enums::PrivacySetting;
use crate::HardcoverClient;
use serde::Deserialize;
use serde_json::Value;
use time::{OffsetDateTime, PlainDateTime};

const QUERY_FIELDS: &str = r#"
books_count
created_at
default_view
description
featured
featured_profile
followers_count
id
imported
likes_count
name
object_type
privacy_setting_id
public
ranked
slug
updated_at
url
user_id
"#;

/// Represents a curated book list in Hardcover.
#[derive(Debug, Clone, Deserialize)]
pub struct List {
    /// Number of books in the list.
    pub books_count: u64,
    /// Creation timestamp.
    #[serde(deserialize_with = "date_parsing::plain_datetime_optional")]
    pub created_at: Option<PlainDateTime>,
    /// Default display layout/view mode (e.g. grid, list).
    pub default_view: String,
    /// Description of the list.
    pub description: Option<String>,
    /// Whether this list is featured sitewide.
    pub featured: bool,
    /// Whether this list is featured on the user's profile.
    pub featured_profile: bool,
    /// Number of users following this list.
    pub followers_count: Option<u64>,
    /// Unique identifier for the list.
    pub id: u64,
    /// Whether this list was imported from another service.
    pub imported: bool,
    /// Number of likes received by this list.
    pub likes_count: u64,
    /// Title / name of the list.
    pub name: String,
    /// GraphQL object type name.
    pub object_type: String,
    /// Privacy visibility level.
    pub privacy_setting_id: PrivacySetting,
    /// Whether this list is publicly visible.
    pub public: bool,
    /// Whether this list is ordered/ranked.
    pub ranked: bool,
    /// URL slug for the list.
    pub slug: Option<String>,
    /// Last update timestamp.
    #[serde(deserialize_with = "date_parsing::offset_datetime_optional")]
    pub updated_at: Option<OffsetDateTime>,
    /// Web URL of the list.
    pub url: Option<String>,
    /// User ID of the list creator.
    pub user_id: u64,
}

impl BaseHardcoverItem for List {
    async fn from_id(id: u64, client: &HardcoverClient) -> Result<Self, reqwest::Error> {
        let query = r#"
        query GetList($id: Int!) {
          lists_by_pk(id: $id) {"#
            .to_string()
            + QUERY_FIELDS
            + r#"
          }
        }
        "#;

        let variables = serde_json::json!({ "id": id });
        let data = Self::from_data(query, variables, client).await?;

        Ok(Self::from_value(data["lists_by_pk"].clone()))
    }

    fn from_value(data: Value) -> Self {
        serde_json::from_value(data).unwrap()
    }
}
