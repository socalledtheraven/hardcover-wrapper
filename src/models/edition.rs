//! Edition model representing specific publications, formats, and ISBNs of a book.

use crate::utils::base_hardcover_item::BaseHardcoverItem;
use crate::utils::date_parsing;
use crate::utils::enums::EditionRecordState;
use crate::HardcoverClient;
use serde::Deserialize;
use serde_json::Value;
use time::{Date, PlainDateTime};

const QUERY_FIELDS: &str = r#"
alternative_titles
asin
audio_seconds
book_id
canonical_id
compilation
country_id
created_at
created_by_user_id
curation_status
edition_format
edition_information
id
image_id
isbn_10
isbn_10_valid
isbn_13
isbn_13_valid
isbns_match
language_id
lists_count
locked
normalized_at
object_type
original_book_id
pages
physical_format
physical_information
publisher_id
rating
reading_format_id
release_date
release_year
score
source
state
subtitle
title
updated_at
users_count
users_read_count
"#;

/// Physical or digital edition publication format.
#[derive(Debug, Clone, Deserialize)]
pub enum EditionFormat {
    Hardcover,
    Paperback,
    Ebook,
    Audiobook,
}

/// Medium/reading format category of an edition.
#[derive(Debug, Clone, Deserialize)]
#[serde(from = "u64")]
pub enum ReadingFormat {
    Physical,
    Audio,
    Both,
    Ebook,
}

impl From<u64> for ReadingFormat {
    fn from(value: u64) -> Self {
        match value {
            1 => ReadingFormat::Physical,
            2 => ReadingFormat::Audio,
            3 => ReadingFormat::Both,
            4 => ReadingFormat::Ebook,
            _ => panic!("Unknown reading format"),
        }
    }
}

/// Represents a specific published edition of a book.
#[derive(Debug, Clone, Deserialize)]
pub struct Edition {
    /// Alternative titles specific to this edition.
    pub alternative_titles: Vec<String>,
    /// Amazon Standard Identification Number (ASIN).
    pub asin: Option<String>,
    /// Duration in seconds for audiobook editions.
    pub audio_seconds: Option<u64>,
    /// Parent book ID.
    pub book_id: u64,
    /// Canonical edition ID if deduplicated.
    pub canonical_id: Option<u64>,
    /// Whether this edition is a compilation/omnibus.
    pub compilation: bool,
    /// Country ID of publication.
    pub country_id: Option<u64>,
    /// Creation timestamp.
    #[serde(deserialize_with = "date_parsing::plain_datetime")]
    pub created_at: PlainDateTime,
    /// User ID of contributor who created the edition.
    pub created_by_user_id: Option<u64>,
    /// Curation review status.
    pub curation_status: u64,
    /// Format string descriptor (e.g. "Paperback", "Kindle Edition").
    pub edition_format: Option<String>,
    /// Additional edition notes (e.g. "Special Illustrated Edition").
    pub edition_information: Option<String>,
    /// Unique identifier for the edition.
    pub id: u64,
    /// Cover image ID.
    pub image_id: Option<u64>,
    /// 10-digit ISBN.
    pub isbn_10: Option<String>,
    /// Whether the ISBN-10 checksum is valid.
    pub isbn_10_valid: Option<bool>,
    /// 13-digit ISBN.
    pub isbn_13: Option<String>,
    /// Whether the ISBN-13 checksum is valid.
    pub isbn_13_valid: Option<bool>,
    /// Whether ISBN-10 and ISBN-13 match the same work.
    pub isbns_match: Option<bool>,
    /// Language ID of this edition.
    pub language_id: Option<u64>,
    /// Number of lists containing this edition.
    pub lists_count: u64,
    /// Whether this edition is locked against editing.
    pub locked: bool,
    /// Normalization timestamp.
    #[serde(deserialize_with = "date_parsing::plain_datetime_optional")]
    pub normalized_at: Option<PlainDateTime>,
    /// GraphQL object type name.
    pub object_type: String,
    /// Original book ID if reassigned.
    pub original_book_id: Option<u64>,
    /// Page count of this edition.
    pub pages: Option<u64>,
    /// Physical binding/format string.
    pub physical_format: Option<String>,
    /// Physical dimensions and weight details.
    pub physical_information: Option<String>,
    /// Publisher ID.
    pub publisher_id: Option<u64>,
    /// Average user rating score.
    pub rating: Option<f64>,
    /// Reading format classification (Physical, Audio, Ebook, etc.).
    pub reading_format_id: ReadingFormat,
    /// Release date of this edition.
    #[serde(deserialize_with = "date_parsing::date_optional")]
    pub release_date: Option<Date>,
    /// Release year of this edition.
    pub release_year: Option<u64>,
    /// Popularity/relevance score.
    pub score: u64,
    /// Data source identifier.
    pub source: Option<String>,
    /// Processing/linking state of the edition.
    pub state: EditionRecordState,
    /// Subtitle of this edition.
    pub subtitle: Option<String>,
    /// Title of this edition.
    pub title: Option<String>,
    /// Last update timestamp.
    #[serde(deserialize_with = "date_parsing::plain_datetime")]
    pub updated_at: PlainDateTime,
    /// Number of users who have this edition in their library.
    pub users_count: u64,
    /// Number of users who have read this edition.
    pub users_read_count: u64,
}

impl BaseHardcoverItem for Edition {
    async fn from_id(id: u64, client: &HardcoverClient) -> Result<Self, reqwest::Error> {
        let query = r#"
        query GetEdition($id: Int!) {
          editions_by_pk(id: $id) {"#
            .to_string()
            + QUERY_FIELDS
            + r#"
          }
        }
        "#;

        let variables = serde_json::json!({ "id": id });
        let data = Self::from_data(query, variables, client).await?;

        Ok(Self::from_value(data["editions_by_pk"].clone()))
    }

    fn from_value(data: Value) -> Self {
        serde_json::from_value(data).unwrap()
    }
}
