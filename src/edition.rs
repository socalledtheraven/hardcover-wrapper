use crate::date_parsing;
use serde::Deserialize;
use crate::base_hardcover_item::BaseHardcoverItem;
use crate::util::{Gender, RecordState3};
use crate::HardcoverClient;
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

#[derive(Debug, Clone, Deserialize)]
pub enum EditionFormat {
    Hardcover,
    Paperback,
    Ebook,
    Audiobook,
}

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

#[derive(Debug, Clone, Deserialize)]
pub struct Edition {
    pub alternative_titles: Vec<String>,
    pub asin: Option<String>,
    pub audio_seconds: Option<u64>,
    pub book_id: u64,
    pub canonical_id: Option<u64>,
    pub compilation: bool,
    pub country_id: Option<u64>,
    #[serde(deserialize_with = "date_parsing::plain_datetime")]
    pub created_at: PlainDateTime,
    pub created_by_user_id: Option<u64>,
    pub curation_status: u64,
    pub edition_format: Option<String>,
    pub edition_information: Option<String>,
    pub id: u64,
    pub image_id: Option<u64>,
    pub isbn_10: Option<String>,
    pub isbn_10_valid: Option<bool>,
    pub isbn_13: Option<String>,
    pub isbn_13_valid: Option<bool>,
    pub isbns_match: Option<bool>,
    pub language_id: Option<u64>,
    pub lists_count: u64,
    pub locked: bool,
    #[serde(deserialize_with = "date_parsing::plain_datetime_optional")]
    pub normalized_at: Option<PlainDateTime>,
    pub object_type: String,
    pub original_book_id: Option<u64>,
    pub pages: Option<u64>,
    pub physical_format: Option<String>,
    pub physical_information: Option<String>,
    pub publisher_id: Option<u64>,
    pub rating: Option<f64>,
    pub reading_format_id: ReadingFormat,
    #[serde(deserialize_with = "date_parsing::date_optional")]
    pub release_date: Option<Date>,
    pub release_year: Option<u64>,
    pub score: u64,
    pub source: Option<String>,
    pub state: RecordState3,
    pub subtitle: Option<String>,
    pub title: Option<String>,
    #[serde(deserialize_with = "date_parsing::plain_datetime")]
    pub updated_at: PlainDateTime,
    pub users_count: u64,
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

        let data = Self::from_data(query, id, client).await?;

        Ok(Self::new(data["editions_by_pk"].clone()))
    }

    fn new(data: Value) -> Self {
        serde_json::from_value(data).unwrap()
    }
}
