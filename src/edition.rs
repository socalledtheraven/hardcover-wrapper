use serde_json::Value;
use time::{Date, PlainDateTime};
use crate::base_hardcover_item::BaseHardcoverItem;
use crate::util::RecordState3;
use crate::graphql::{GraphQLResponse};
use crate::image::Image;

const QUERY_FIELDS: &str = r#"
alternative_titles
asin
audio_seconds
book_id
cached_contributors
cached_image
cached_tags
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
enum EditionFormat {
    Hardcover,
    Paperback,
    Ebook,
    Audiobook,
}

#[derive(Debug, Clone)]
enum ReadingFormat {
    Physical,
    Audio,
    Both,
    Ebook,
}

#[derive(Debug, Clone)]
pub(crate) struct Edition {
    alternative_titles: Value,
    asin: Option<String>,
    audio_seconds: Option<u64>,
    book_id: u64,
    cached_contributors: Value,
    cached_image: Image,
    cached_tags: Value,
    canonical_id: Option<u64>,
    compilation: bool,
    country_id: Option<u64>,
    created_at: PlainDateTime,
    created_by_user_id: Option<u64>,
    curation_status: Value,
    edition_format: Value,
    edition_information: Option<String>,
    id: u64,
    image_id: Option<u64>,
    isbn_10: Option<String>,
    isbn_10_valid: Option<bool>,
    isbn_13: Option<String>,
    isbn_13_valid: Option<bool>,
    isbns_match: Option<bool>,
    language_id: Option<u64>,
    lists_count: u64,
    locked: bool,
    normalized_at: Option<PlainDateTime>,
    object_type: String,
    original_book_id: Option<u64>,
    pages: Option<u64>,
    physical_format: Option<String>,
    physical_information: Option<String>,
    publisher_id: Option<u64>,
    rating: Option<f64>,
    reading_format_id: ReadingFormat,
    release_date: Option<Date>,
    release_year: Option<u64>,
    score: u64,
    source: Option<String>,
    state: RecordState3,
    subtitle: Option<String>,
    title: Option<String>,
    updated_at: PlainDateTime,
    users_count: u64,
    users_read_count: u64,
}

impl BaseHardcoverItem for Edition {
    async fn from_id(id: u64) -> Result<Self, reqwest::Error> {
        let query = r#"
        query GetEdition($id: Int!) {
          editions_by_pk(id: $id) {"#.to_string() + QUERY_FIELDS + r#"
          }
        }
        "#;

        let data = Self::from_data(query, id).await?;

        Ok(Self::new(data["editions_by_pk"].clone()))
    }

    fn new(data: Value) -> Self {
        Edition{
            alternative_titles: {
                data["alternative_titles"].clone()
            },
            asin: {
                data.get_str("asin")
            },
            audio_seconds: {
                data.get_u64("audio_seconds")
            },
            book_id: {
                data.get_u64("book_id").unwrap()
            },
            cached_contributors: {
                data["cached_contributors"].clone()
            },
            cached_image: {
                Image::new(data["cached_image"].clone())
            },
            cached_tags: {
                data["cached_tags"].clone()
            },
            canonical_id: {
                data.get_u64("canonical_id")
            },
            compilation: {
                data.get_bool("compilation")
            },
            country_id: {
                data.get_u64("country_id")
            },
            created_at: {
                data.get_plaindt("created_at").unwrap()
            },
            created_by_user_id: {
                data.get_u64("created_by_user_id")
            },
            curation_status: {
                data["curation_status"].clone()
            },
            edition_format: {
                data["edition_format"].clone()
            },
            edition_information: {
                data.get_str("edition_information")
            },
            id: {
                data.get_u64("id").unwrap()
            },
            image_id: {
                data.get_u64("image_id")
            },
            isbn_10: {
                data.get_str("isbn_10")
            },
            isbn_10_valid: {
                data["isbn_10_valid"].as_bool()
            },
            isbn_13: {
                data.get_str("isbn_13")
            },
            isbn_13_valid: {
                data["isbn_13_valid"].as_bool()
            },
            isbns_match: {
                data["isbns_match"].as_bool()
            },
            language_id: {
                data.get_u64("language_id")
            },
            lists_count: {
                data.get_u64("lists_count").unwrap()
            },
            locked: {
                data["locked"].as_bool().unwrap_or(false)
            },
            normalized_at: {
                data.get_plaindt("normalized_at")
            },
            object_type: {
                data.get_str("object_type").unwrap()
            },
            original_book_id: {
                data.get_u64("original_book_id")
            },
            pages: {
                data.get_u64("pages")
            },
            physical_format: {
                data.get_str("physical_format")
            },
            physical_information: {
                data.get_str("physical_information")
            },
            publisher_id: {
                data.get_u64("publisher_id")
            },
            rating: {
                data["rating"].as_f64()
            },
            reading_format_id: {
                match data["reading_format_id"].as_u64() {
                    Some(1) => ReadingFormat::Physical,
                    Some(2) => ReadingFormat::Audio,
                    Some(3) => ReadingFormat::Both,
                    Some(4) => ReadingFormat::Ebook,
                    _ => panic!("Unknown reading format"),
                }
            },
            release_date: {
                data.get_date("release_date")
            },
            release_year: {
                data.get_u64("release_year")
            },
            score: {
                data.get_u64("score").unwrap()
            },
            source: {
                data.get_str("source")
            },
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
            subtitle: {
                data.get_str("subtitle")
            },
            title: {
                data.get_str("title")
            },
            updated_at: {
                data.get_plaindt("updated_at").unwrap()
            },
            users_count: {
                data.get_u64("users_count").unwrap()
            },
            users_read_count: {
                data.get_u64("users_read_count").unwrap()
            },
        }
    }
}