use crate::date_parsing;
use serde::Deserialize;
use crate::base_hardcover_item::BaseHardcoverItem;
use crate::util::{Gender, RecordState};
use crate::HardcoverClient;
use serde_json::Value;
use time::PlainDateTime;

const QUERY_FIELDS: &str = r#"
biography
books_count
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

#[derive(Debug, Clone, Deserialize)]
pub struct Character {
    pub biography: Option<String>,
    pub books_count: u64,
    pub canonical_books_count: u64,
    pub canonical_id: Option<u64>,
    #[serde(deserialize_with = "date_parsing::plain_datetime")]
    pub created_at: PlainDateTime,
    #[serde(rename = "gender_id")]
    pub gender: Option<Gender>,
    pub has_disability: Option<bool>,
    pub id: u64,
    pub image_id: Option<u64>,
    pub is_lgbtq: Option<bool>,
    pub is_poc: Option<bool>,
    pub locked: Option<bool>,
    pub name: String,
    pub object_type: String,
    pub openlibrary_url: Option<String>,
    pub slug: String,
    pub state: RecordState,
    #[serde(deserialize_with = "date_parsing::plain_datetime")]
    pub updated_at: PlainDateTime,
    pub user_id: Option<u64>,
}

impl BaseHardcoverItem for Character {
    async fn from_id(id: u64, client: &HardcoverClient) -> Result<Self, reqwest::Error> {
        let query = r#"
        query GetCharacter($id: bigint!) {
          characters_by_pk(id: $id) {"#
            .to_string()
            + QUERY_FIELDS
            + r#"
          }
        }
        "#;

        let data = Self::from_data(query, id, client).await?;

        Ok(Self::new(data["characters_by_pk"].clone()))
    }

    fn new(data: Value) -> Self {
        serde_json::from_value(data).unwrap()
    }
}
