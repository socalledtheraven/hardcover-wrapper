//! Author model representing book creators, illustrators, translators, and contributors.

use crate::utils::base_hardcover_item::BaseHardcoverItem;
use crate::utils::date_parsing;
use crate::utils::enums::{Gender, Identifiers, Link, RecordState};
use crate::HardcoverClient;
use serde::Deserialize;
use serde_json::Value;
use time::Date;

const QUERY_FIELDS: &str = r#"
alias_id
alternate_names
bio
books_count
born_date
born_year
canonical_id
death_date
death_year
gender_id
id
identifiers
image_id
is_bipoc
is_lgbtq
links
location
locked
name
name_personal
object_type
slug
state
title
user_id
users_count
"#;

/// Represents an author or contributor profile in Hardcover.
#[derive(Debug, Clone, Deserialize)]
pub struct Author {
    /// Alias author ID if this profile is an alias.
    pub alias_id: Option<u64>,
    /// Alternate names or pen names.
    pub alternate_names: Vec<String>,
    /// Biography or descriptive text.
    pub bio: Option<String>,
    /// Total number of books authored or contributed to.
    pub books_count: u64,

    /// Date of birth.
    #[serde(deserialize_with = "date_parsing::date_optional")]
    pub born_date: Option<Date>,
    /// Year of birth.
    pub born_year: Option<u64>,
    /// Canonical author ID if this record was merged or aliased.
    pub canonical_id: Option<u64>,

    /// Date of death.
    #[serde(deserialize_with = "date_parsing::date_optional")]
    pub death_date: Option<Date>,
    /// Year of death.
    pub death_year: Option<u64>,
    /// Gender identification.
    #[serde(rename = "gender_id")]
    pub gender: Option<Gender>,
    /// Unique identifier for the author.
    pub id: u64,
    /// Identifiers across external services (e.g. Goodreads, Audible, OpenLibrary).
    pub identifiers: Identifiers,
    /// Image ID for the author's portrait or avatar.
    pub image_id: Option<u64>,
    /// Whether the author identifies as BIPOC (Black, Indigenous, Person of Color).
    pub is_bipoc: Option<bool>,
    /// Whether the author identifies as LGBTQ+.
    pub is_lgbtq: Option<bool>,
    /// Web links associated with the author.
    pub links: Vec<Link>,
    /// Location/country of origin.
    pub location: Option<String>,
    /// Whether this record is locked from community editing.
    pub locked: bool,
    /// Display name of the author.
    pub name: String,
    /// Personal/legal name of the author.
    pub name_personal: Option<String>,
    /// GraphQL object type name.
    pub object_type: String,
    /// URL slug for the author's Hardcover page.
    pub slug: Option<String>,
    /// State of the author record.
    pub state: RecordState,
    /// Professional or honorific title.
    pub title: Option<String>,
    /// Associated Hardcover user ID if the author has a verified user profile.
    pub user_id: Option<u64>,
    /// Number of users following this author.
    pub users_count: u64,
}

impl BaseHardcoverItem for Author {
    async fn from_id(id: u64, client: &HardcoverClient) -> Result<Self, reqwest::Error> {
        let query = r#"
        query GetAuthor($id: Int!) {
          authors_by_pk(id: $id) {"#
            .to_string()
            + QUERY_FIELDS
            + r#"
          }
        }
        "#;

        let variables = serde_json::json!({ "id": id });
        let data = Self::from_data(query, variables, client).await?;

        Ok(Self::from_value(data["authors_by_pk"].clone()))
    }

    fn from_value(data: Value) -> Self {
        serde_json::from_value(data).unwrap()
    }
}
