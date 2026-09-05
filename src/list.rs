use serde::{Deserialize, Serialize};
use serde_json::Value;
use time::{OffsetDateTime, PlainDateTime};
use crate::base_hardcover_item::BaseHardcoverItem;
use crate::enums::PrivacySetting;
use crate::graphql::{GraphQLResponse};

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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct List {
    books_count: u64,
    created_at: Option<PlainDateTime>,
    default_view: String,
    description: Option<String>,
    featured: bool,
    featured_profile: bool,
    followers_count: Option<u64>,
    id: u64,
    imported: bool,
    likes_count: u64,
    name: String,
    object_type: String,
    privacy_setting_id: PrivacySetting,
    public: bool,
    ranked: bool,
    slug: Option<String>,
    updated_at: Option<OffsetDateTime>,
    url: Option<String>,
    user_id: u64,
}

impl BaseHardcoverItem for List {
    async fn from_id(id: u64) -> Result<Self, reqwest::Error> {
        let query = r#"
        query GetList($id: Int!) {
          lists_by_pk(id: $id) {"#.to_string() + QUERY_FIELDS + r#"
          }
        }
        "#;

        let data = Self::from_data(query, id).await?;

        Ok(Self::new(data["lists_by_pk"][0].clone()))
    }

    fn new(data: Value) -> Self {
        List {
            books_count: {
                data.get_u64("books_count").unwrap()
            },
            created_at: {
                data.get_plaindt("created_at")
            },
            default_view: {
                data.get_str("default_view").unwrap()
            },
            description: {
                data.get_str("description")
            },
            featured: {
                data.get_bool("featured")
            },
            featured_profile: {
                data.get_bool("featured_profile")
            },
            followers_count: {
                data.get_u64("followers_count")
            },
            id: {
                data.get_u64("id").unwrap()
            },
            imported: {
                data.get_bool("imported")
            },
            likes_count: {
                data.get_u64("likes_count").unwrap()
            },
            name: {
                data.get_str("name").unwrap()
            },
            object_type: {
                data.get_str("object_type").unwrap()
            },
            privacy_setting_id: {
                data.get_privacysetting("privacy_setting_id")
            },
            public: {
                data.get_bool("public")
            },
            ranked: {
                data.get_bool("ranked")
            },
            slug: {
                data.get_str("slug")
            },
            updated_at: {
                data.get_offsetdt("updated_at")
            },
            url: {
                data.get_str("url")
            },
            user_id: {
                data.get_u64("user_id").unwrap()
            },
        }
    }
}
