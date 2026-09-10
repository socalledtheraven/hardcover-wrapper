use serde::Deserialize;
use crate::base_hardcover_item::BaseHardcoverItem;
use crate::client::GraphQLResponse;
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
        List {
            books_count: { data.get_u64("books_count").unwrap() },
            created_at: { data.get_plaindt("created_at") },
            default_view: { data.get_str("default_view").unwrap() },
            description: { data.get_str("description") },
            featured: { data.get_bool("featured").unwrap() },
            featured_profile: { data.get_bool("featured_profile").unwrap() },
            followers_count: { data.get_u64("followers_count") },
            id: { data.get_u64("id").unwrap() },
            imported: { data.get_bool("imported").unwrap() },
            likes_count: { data.get_u64("likes_count").unwrap() },
            name: { data.get_str("name").unwrap() },
            object_type: { data.get_str("object_type").unwrap() },
            privacy_setting_id: { data.get_privacysetting("privacy_setting_id") },
            public: { data.get_bool("public").unwrap() },
            ranked: { data.get_bool("ranked").unwrap() },
            slug: { data.get_str("slug") },
            updated_at: { data.get_offsetdt("updated_at") },
            url: { data.get_str("url") },
            user_id: { data.get_u64("user_id").unwrap() },
        }
    }
}
