use serde_json::Value;
use time::PlainDateTime;
use crate::base_hardcover_item::BaseHardcoverItem;
use crate::util::{Gender, RecordState};
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

#[derive(Debug, Clone)]
pub(crate) struct Character {
    biography: Option<String>,
    books_count: u64,
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
          characters_by_pk(id: $id) {"#.to_string() + QUERY_FIELDS + r#"
          }
        }
        "#;

        let data = Self::from_data(query, id).await?;

        Ok(Self::new(data["characters_by_pk"].clone()))
    }

    fn new(data: Value) -> Self {
        Character {
            biography: {
                data.get_str("biography")
            },
            books_count: {
                data.get_u64("books_count").unwrap()
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
                data.get_bool("locked")
            },
            id: {
                data.get_u64("id").unwrap()
            },
            image_id: {
                data.get_u64("image_id")
            },
            is_lgbtq: {
                data.get_bool("locked")
            },
            is_poc: {
                data.get_bool("locked")
            },
            locked: {
                data.get_bool("locked")
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