use crate::base_hardcover_item::BaseHardcoverItem;
use crate::client::GraphQLResponse;
use crate::util::RecordState3;
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

#[derive(Debug, Clone)]
pub enum EditionFormat {
    Hardcover,
    Paperback,
    Ebook,
    Audiobook,
}

#[derive(Debug, Clone)]
pub enum ReadingFormat {
    Physical,
    Audio,
    Both,
    Ebook,
}

#[derive(Debug, Clone)]
pub struct Edition {
    pub alternative_titles: Vec<String>,
    pub asin: Option<String>,
    pub audio_seconds: Option<u64>,
    pub book_id: u64,
    pub canonical_id: Option<u64>,
    pub compilation: bool,
    pub country_id: Option<u64>,
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
    pub normalized_at: Option<PlainDateTime>,
    pub object_type: String,
    pub original_book_id: Option<u64>,
    pub pages: Option<u64>,
    pub physical_format: Option<String>,
    pub physical_information: Option<String>,
    pub publisher_id: Option<u64>,
    pub rating: Option<f64>,
    pub reading_format_id: ReadingFormat,
    pub release_date: Option<Date>,
    pub release_year: Option<u64>,
    pub score: u64,
    pub source: Option<String>,
    pub state: RecordState3,
    pub subtitle: Option<String>,
    pub title: Option<String>,
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
        Edition {
            alternative_titles: { data.get_str_vec("alternative_titles") },
            asin: { data.get_str("asin") },
            audio_seconds: { data.get_u64("audio_seconds") },
            book_id: { data.get_u64("book_id").unwrap() },
            canonical_id: { data.get_u64("canonical_id") },
            compilation: { data.get_bool("compilation").unwrap() },
            country_id: { data.get_u64("country_id") },
            created_at: { data.get_plaindt("created_at").unwrap() },
            created_by_user_id: { data.get_u64("created_by_user_id") },
            curation_status: { data.get_u64("curation_status").unwrap() },
            edition_format: { data.get_str("edition_format") },
            edition_information: { data.get_str("edition_information") },
            id: { data.get_u64("id").unwrap() },
            image_id: { data.get_u64("image_id") },
            isbn_10: { data.get_str("isbn_10") },
            isbn_10_valid: { data["isbn_10_valid"].as_bool() },
            isbn_13: { data.get_str("isbn_13") },
            isbn_13_valid: { data["isbn_13_valid"].as_bool() },
            isbns_match: { data.get_bool("isbns_match") },
            language_id: { data.get_u64("language_id") },
            lists_count: { data.get_u64("lists_count").unwrap() },
            locked: { data.get_bool("locked").unwrap() },
            normalized_at: { data.get_plaindt("normalized_at") },
            object_type: { data.get_str("object_type").unwrap() },
            original_book_id: { data.get_u64("original_book_id") },
            pages: { data.get_u64("pages") },
            physical_format: { data.get_str("physical_format") },
            physical_information: { data.get_str("physical_information") },
            publisher_id: { data.get_u64("publisher_id") },
            rating: { data["rating"].as_f64() },
            reading_format_id: {
                match data["reading_format_id"].as_u64() {
                    Some(1) => ReadingFormat::Physical,
                    Some(2) => ReadingFormat::Audio,
                    Some(3) => ReadingFormat::Both,
                    Some(4) => ReadingFormat::Ebook,
                    _ => panic!("Unknown reading format"),
                }
            },
            release_date: { data.get_date("release_date") },
            release_year: { data.get_u64("release_year") },
            score: { data.get_u64("score").unwrap() },
            source: { data.get_str("source") },
            state: {
                match data["state"].as_str() {
                    Some("pending") => RecordState3::Pending,
                    Some("linking") => RecordState3::Linking,
                    Some("linked") => RecordState3::Linked,
                    Some("normalized") => RecordState3::Normalized,
                    Some("error") => RecordState3::Error,
                    Some("duplicate") => RecordState3::Duplicate,
                    _ => panic!("Unknown record state"),
                }
            },
            subtitle: { data.get_str("subtitle") },
            title: { data.get_str("title") },
            updated_at: { data.get_plaindt("updated_at").unwrap() },
            users_count: { data.get_u64("users_count").unwrap() },
            users_read_count: { data.get_u64("users_read_count").unwrap() },
        }
    }
}
