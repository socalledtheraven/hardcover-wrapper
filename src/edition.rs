use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use time::{Date, PlainDateTime};
use crate::enums::RecordState3;
use crate::graphql::{get_bool_from_resp, get_date_from_resp, get_plaindatetime_from_resp, get_str_from_resp, get_u64_from_resp, graphql_req};
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
cover_image
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

#[derive(Debug, Serialize, Deserialize, Clone)]
enum EditionFormat {
    Hardcover,
    Paperback,
    Ebook,
    Audiobook,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
enum ReadingFormat {
    Physical,
    Audio,
    Both,
    Ebook,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
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
    cover_image: Value,
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

impl Edition {
    pub(crate) async fn from_edition_id(edition_id: u64) -> Result<Self, reqwest::Error> {
        let query = r#"
        query GetAuthor($edition: Int!) {
          editions(where: {id: {_eq: $edition}}, limit: 1) {"#.to_string() + QUERY_FIELDS + r#"
          }
        }
        "#;

        Self::from_data(query, edition_id).await
    }

    async fn from_data<T: ToString>(query: String, user_data: T) -> Result<Self, reqwest::Error> {
        let mut vars = HashMap::new();
        vars.insert("edition", user_data.to_string());

        Self::new(query, vars).await
    }

    async fn new(query: String, vars: HashMap<&str, String>) -> Result<Self, reqwest::Error>  {
        let resp = graphql_req(query, vars).await?;

        let data = &resp["data"]["editions"][0];

        Ok(Edition{
            alternative_titles: {
                data["alternative_titles"].clone()
            },
            asin: {
                get_str_from_resp(data, "asin")
            },
            audio_seconds: {
                get_u64_from_resp(data, "audio_seconds")
            },
            book_id: {
                get_u64_from_resp(data, "book_id").unwrap()
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
                get_u64_from_resp(data, "canonical_id")
            },
            compilation: {
                get_bool_from_resp(data, "compilation")
            },
            country_id: {
                get_u64_from_resp(data, "country_id")
            },
            created_at: {
                get_plaindatetime_from_resp(data, "created_at").unwrap()
            },
            created_by_user_id: {
                get_u64_from_resp(data, "created_by_user_id")
            },
            curation_status: {
                data["curation_status"].clone()
            },
            edition_format: {
                data["edition_format"].clone()
            },
            edition_information: {
                get_str_from_resp(data, "edition_information")
            },
            id: {
                get_u64_from_resp(data, "id").unwrap()
            },
            cover_image: {
                data["cover_image"].clone()
            },
            image_id: {
                get_u64_from_resp(data, "image_id")
            },
            isbn_10: {
                get_str_from_resp(data, "isbn_10")
            },
            isbn_10_valid: {
                data["isbn_10_valid"].as_bool()
            },
            isbn_13: {
                get_str_from_resp(data, "isbn_13")
            },
            isbn_13_valid: {
                data["isbn_13_valid"].as_bool()
            },
            isbns_match: {
                data["isbns_match"].as_bool()
            },
            language_id: {
                get_u64_from_resp(data, "language_id")
            },
            lists_count: {
                get_u64_from_resp(data, "lists_count").unwrap()
            },
            locked: {
                data["locked"].as_bool().unwrap_or(false)
            },
            normalized_at: {
                get_plaindatetime_from_resp(data, "normalized_at")
            },
            object_type: {
                get_str_from_resp(data, "object_type").unwrap()
            },
            original_book_id: {
                get_u64_from_resp(data, "original_book_id")
            },
            pages: {
                get_u64_from_resp(data, "pages")
            },
            physical_format: {
                get_str_from_resp(data, "physical_format")
            },
            physical_information: {
                get_str_from_resp(data, "physical_information")
            },
            publisher_id: {
                get_u64_from_resp(data, "publisher_id")
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
                get_date_from_resp(data, "release_date")
            },
            release_year: {
                get_u64_from_resp(data, "release_year")
            },
            score: {
                get_u64_from_resp(data, "score").unwrap()
            },
            source: {
                get_str_from_resp(data, "source")
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
                get_str_from_resp(data, "subtitle")
            },
            title: {
                get_str_from_resp(data, "title")
            },
            updated_at: {
                get_plaindatetime_from_resp(data, "updated_at").unwrap()
            },
            users_count: {
                get_u64_from_resp(data, "users_count").unwrap()
            },
            users_read_count: {
                get_u64_from_resp(data, "users_read_count").unwrap()
            },
        })
    }
}