use crate::date_parsing;
use serde::Deserialize;
use crate::base_hardcover_item::BaseHardcoverItem;
use crate::util::PrivacySetting;
use crate::HardcoverClient;
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

#[derive(Debug, Clone, Deserialize)]
pub struct List {
    pub books_count: u64,
    #[serde(deserialize_with = "date_parsing::plain_datetime_optional")]
    pub created_at: Option<PlainDateTime>,
    pub default_view: String,
    pub description: Option<String>,
    pub featured: bool,
    pub featured_profile: bool,
    pub followers_count: Option<u64>,
    pub id: u64,
    pub imported: bool,
    pub likes_count: u64,
    pub name: String,
    pub object_type: String,
    pub privacy_setting_id: PrivacySetting,
    pub public: bool,
    pub ranked: bool,
    pub slug: Option<String>,
    #[serde(deserialize_with = "date_parsing::offset_datetime_optional")]
    pub updated_at: Option<OffsetDateTime>,
    pub url: Option<String>,
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

        let data = Self::from_data(query, id, client).await?;

        Ok(Self::new(data["lists_by_pk"].clone()))
    }

    fn new(data: Value) -> Self {
        serde_json::from_value(data).unwrap()
    }
}
