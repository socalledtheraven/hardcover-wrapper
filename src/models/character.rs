//! Character model representing fictional or real characters appearing in books.

use crate::utils::base_hardcover_item::BaseHardcoverItem;
use crate::utils::date_parsing;
use crate::utils::enums::{Gender, RecordState};
use crate::HardcoverClient;
use serde::Deserialize;
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

/// Represents a book character profile.
#[derive(Debug, Clone, Deserialize)]
pub struct Character {
    /// Biography or background description of the character.
    pub biography: Option<String>,
    /// Total number of books featuring this character.
    pub books_count: u64,
    /// Number of canonical books featuring this character.
    pub canonical_books_count: u64,
    /// Canonical character ID if deduplicated.
    pub canonical_id: Option<u64>,
    /// Creation timestamp.
    #[serde(deserialize_with = "date_parsing::plain_datetime")]
    pub created_at: PlainDateTime,
    /// Gender identification.
    #[serde(rename = "gender_id")]
    pub gender: Option<Gender>,
    /// Whether the character is identified as having a disability.
    pub has_disability: Option<bool>,
    /// Unique identifier for the character.
    pub id: u64,
    /// Portrait image ID.
    pub image_id: Option<u64>,
    /// Whether the character is identified as LGBTQ+.
    pub is_lgbtq: Option<bool>,
    /// Whether the character is identified as a person of color (POC).
    pub is_poc: Option<bool>,
    /// Whether this record is locked from editing.
    pub locked: Option<bool>,
    /// Full name of the character.
    pub name: String,
    /// GraphQL object type name.
    pub object_type: String,
    /// OpenLibrary URL for the character.
    pub openlibrary_url: Option<String>,
    /// URL slug for the character's Hardcover page.
    pub slug: String,
    /// Record state.
    pub state: RecordState,
    /// Last update timestamp.
    #[serde(deserialize_with = "date_parsing::plain_datetime")]
    pub updated_at: PlainDateTime,
    /// Contributor user ID who created the character entry.
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

        let variables = serde_json::json!({ "id": id });
        let data = Self::from_data(query, variables, client).await?;

        Ok(Self::from_value(data["characters_by_pk"].clone()))
    }

    fn from_value(data: Value) -> Self {
        serde_json::from_value(data).unwrap()
    }
}
