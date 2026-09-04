use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use time::PlainDateTime;
use crate::enums::{Gender, RecordState};
use crate::graphql::{get_plaindatetime_from_resp, get_str_from_resp, get_u64_from_resp, graphql_req};

const QUERY_FIELDS: &str = r#"
biography
books_count
cached_tags
canonical_books_count
canonical_id
created_at
gender_id
has_disability
id
image_id
is_lgbtq
is_poc
locked
name
object_type
openlibrary_url
slug
state
updated_at
user_id
"#;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct Character {
    biography: Option<String>,
    books_count: u64,
    cached_tags: Value,
    canonical_books_count: u64,
    canonical_id: Option<u64>,
    created_at: PlainDateTime,
    gender_id: Option<Gender>,
    has_disability: Option<bool>,
    id: u64,
    image_id: Option<u64>,
    is_lgbtq: Option<bool>,
    is_poc: Option<bool>,
    locked: Option<bool>,
    name: String,
    object_type: String,
    openlibrary_url: Option<String>,
    slug: String,
    state: RecordState,
    updated_at: PlainDateTime,
    user_id: Option<u64>,
}

impl Character {
    pub(crate) async fn from_character_id(character_id: u64) -> Result<Self, reqwest::Error> {
        let query = r#"
        query GetBook($character: bigint!) {
          characters(where: {id: {_eq: $character}}, limit: 1) {"#.to_string() + QUERY_FIELDS + r#"
          }
        }
        "#;

        Self::from_data(query, character_id).await
    }

    async fn from_data<T: ToString>(query: String, user_data: T) -> Result<Self, reqwest::Error> {
        let mut vars = HashMap::new();
        vars.insert("character", user_data.to_string());

        Self::new(query, vars).await
    }

    async fn new(query: String, vars: HashMap<&str, String>) -> Result<Self, reqwest::Error>  {
        let resp = graphql_req(query, vars).await?;

        let data = &resp["data"]["characters"][0];

        Ok(Character{
            biography: {
                get_str_from_resp(data, "biography")
            },
            books_count: {
                get_u64_from_resp(data, "books_count").unwrap()
            },
            cached_tags: {
                data["cached_tags"].clone()
            },
            canonical_books_count: {
                get_u64_from_resp(data, "canonical_books_count").unwrap()
            },
            canonical_id: {
                get_u64_from_resp(data, "canonical_id")
            },
            created_at: {
                get_plaindatetime_from_resp(data, "created_at").unwrap()
            },
            gender_id: {
                // there is not a single character with a listed gender in the api
                None
            },
            has_disability: {
                data["has_disability"].as_bool()
            },
            id: {
                get_u64_from_resp(data, "id").unwrap()
            },
            image_id: {
                get_u64_from_resp(data, "image_id")
            },
            is_lgbtq: {
                data["is_lgbtq"].as_bool()
            },
            is_poc: {
                data["is_poc"].as_bool()
            },
            locked: {
                data["locked"].as_bool()
            },
            name: {
                get_str_from_resp(data, "name").unwrap()
            },
            object_type: {
                get_str_from_resp(data, "object_type").unwrap()
            },
            openlibrary_url: {
                get_str_from_resp(data, "openlibrary_url")
            },
            slug: {
                get_str_from_resp(data, "slug").unwrap()
            },
            state: {
                match data["state"].as_str() {
                    Some("active") => RecordState::Active,
                    Some("duplicate") => RecordState::Duplicate,
                    // ahh, error handling
                    _ => panic!("Unknown record state"),
                }
            },
            updated_at: {
                get_plaindatetime_from_resp(data, "updated_at").unwrap()
            },
            user_id: {
                get_u64_from_resp(data, "user_id")
            },
        })
    }
}