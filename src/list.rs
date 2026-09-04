use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use time::{OffsetDateTime, PlainDateTime};
use crate::enums::PrivacySetting;
use crate::graphql::{graphql_req, GraphQLResponse};

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

impl List {
    pub(crate) async fn from_list_id(list_id: u64) -> Result<Self, reqwest::Error> {
        let query = r#"
        query GetAuthor($list: Int!) {
          lists_by_pk(id: $list) {"#.to_string() + QUERY_FIELDS + r#"
          }
        }
        "#;

        Self::from_data(query, list_id).await
    }

    async fn from_data<T: ToString>(query: String, user_data: T) -> Result<Self, reqwest::Error> {
        let mut vars = HashMap::new();
        vars.insert("list", user_data.to_string());

        Self::new(query, vars).await
    }

    async fn new(query: String, vars: HashMap<&str, String>) -> Result<Self, reqwest::Error> {
        let resp = graphql_req(query, vars).await?;

        let data = &resp["data"]["lists_by_pk"];

        Ok(List{
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
        })
    }
}
