use serde::{Deserialize, Serialize};
use serde_json::Value;
use time::PlainDateTime;
use crate::base_hardcover_item::BaseHardcoverItem;
use crate::enums::{Gender, RecordState};
use crate::graphql::{GraphQLResponse};

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

impl BaseHardcoverItem for Character {
    async fn from_id(id: u64) -> Result<Self, reqwest::Error> {
        let query = r#"
        query GetCharacter($id: bigint!) {
          characters(where: {id: {_eq: $id}}, limit: 1) {"#.to_string() + QUERY_FIELDS + r#"
          }
        }
        "#;

        let data = Self::from_data(query, id).await?;

        Ok(Self::new(data["characters"][0].clone()))
    }

    fn new(data: Value) -> Self {
        Character {
            biography: {
                data.get_str("biography")
            },
            books_count: {
                data.get_u64("books_count").unwrap()
            },
            cached_tags: {
                data["cached_tags"].clone()
            },
            canonical_books_count: {
                data.get_u64("canonical_books_count").unwrap()
            },
            canonical_id: {
                data.get_u64("canonical_id")
            },
            created_at: {
                data.get_plaindt("created_at").unwrap()
            },
            gender_id: {
                // there is not a single character with a listed gender in the api
                None
            },
            has_disability: {
                data["has_disability"].as_bool()
            },
            id: {
                data.get_u64("id").unwrap()
            },
            image_id: {
                data.get_u64("image_id")
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
                data.get_str("name").unwrap()
            },
            object_type: {
                data.get_str("object_type").unwrap()
            },
            openlibrary_url: {
                data.get_str("openlibrary_url")
            },
            slug: {
                data.get_str("slug").unwrap()
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
                data.get_plaindt("updated_at").unwrap()
            },
            user_id: {
                data.get_u64("user_id")
            },
        }
    }
}