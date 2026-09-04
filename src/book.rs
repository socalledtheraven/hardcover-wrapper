use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use time::{Date, OffsetDateTime, PlainDateTime};
use crate::enums::{RecordState2};
use crate::graphql::{get_bool_from_resp,
                     get_date_from_resp,
                     get_offsetdatetime_from_resp,
                     get_plaindatetime_from_resp,
                     get_str_from_resp,
                     get_str_vec,
                     get_u64_from_resp,
                     graphql_req};
use crate::image::Image;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) enum BookCategory {
    Book,
    Novella,
    ShortStory,
    GraphicNovel,
    FanFiction,
    ResearchPaper,
    Poetry,
    Collection,
    WebNovel,
    LightNovel
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) enum BookStatus {
    OK,
    ToReview,
    Deleted,
    Deduplicated
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) enum LiteraryType {
    Fiction,
    NonFiction
}

const QUERY_FIELDS: &str = r#"
activities_count
alternative_titles
audio_seconds
book_category_id
book_status_id
cached_contributors
cached_featured_series
cached_header_image
cached_image
cached_similar_book_ids
cached_similar_books_updated_at
cached_tags
canonical_id
compilation
created_at
created_by_user_id
curation_status
default_audio_edition_id
default_cover_edition_id
default_ebook_edition_id
default_physical_edition_id
description
editions_count
featured_book_series_id
header_image_id
headline
image_id
id
import_platform_id
is_partial_book
journals_count
links
lists_count
literary_type_id
locked
pages
parent_book_id
prompts_count
rating
ratings_count
ratings_distribution
release_date
release_year
reviews_count
slug
state
subtitle
title
updated_at
users_count
users_read_count
"#;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct Book {
    pub(crate) activities_count: u64,
    pub(crate) alternative_titles: Vec<String>,
    pub(crate) audio_seconds: Option<u64>,
    pub(crate) book_category_id: BookCategory,
    pub(crate) book_status_id: BookStatus,
    pub(crate) cached_contributors: Value,
    pub(crate) cached_featured_series: Option<Value>,
    pub(crate) cached_header_image: Option<Image>,
    pub(crate) cached_image: Image,
    pub(crate) cached_similar_book_ids: Value,
    pub(crate) cached_similar_books_updated_at: Option<PlainDateTime>,
    pub(crate) cached_tags: Value,
    pub(crate) canonical_id: Option<u64>,
    pub(crate) compilation: bool,
    pub(crate) created_at: PlainDateTime,
    pub(crate) created_by_user_id: Option<u64>,
    pub(crate) curation_status: Value,
    pub(crate) default_audio_edition_id: Option<u64>,
    pub(crate) default_cover_edition_id: Option<u64>,
    pub(crate) default_ebook_edition_id: Option<u64>,
    pub(crate) default_physical_edition_id: Option<u64>,
    pub(crate) description: Option<String>,
    pub(crate) editions_count: u64,
    pub(crate) featured_book_series_id: Option<u64>,
    pub(crate) header_image_id: Option<u64>,
    pub(crate) headline: Option<String>,
    pub(crate) id: u64,
    pub(crate) image_id: Option<u64>,
    pub(crate) import_platform_id: u64,
    pub(crate) is_partial_book: Option<bool>,
    pub(crate) journals_count: u64,
    pub(crate) links: Value,
    pub(crate) lists_count: Option<u64>,
    pub(crate) literary_type_id: Option<LiteraryType>,
    pub(crate) locked: bool,
    pub(crate) pages: Option<u64>,
    pub(crate) parent_book_id: Option<u64>,
    pub(crate) prompts_count: u64,
    pub(crate) rating: Option<f64>,
    pub(crate) ratings_count: u64,
    pub(crate) ratings_distribution: Value,
    pub(crate) release_date: Option<Date>,
    pub(crate) release_year: Option<u64>,
    pub(crate) reviews_count: u64,
    pub(crate) slug: Option<String>,
    pub(crate) state: RecordState2,
    pub(crate) subtitle: Option<String>,
    pub(crate) title: Option<String>,
    pub(crate) updated_at: Option<OffsetDateTime>,
    pub(crate) users_count: u64,
    pub(crate) users_read_count: u64,
}

impl Book {
    pub(crate) async fn from_book_id(book_id: u64) -> Result<Self, reqwest::Error> {
        let query = r#"
        query GetBook($book: Int!) {
          books(where: {id: {_eq: $book}}, limit: 1) {"#.to_string() + QUERY_FIELDS + r#"
          }
        }
        "#;

        Self::from_data(query, book_id).await
    }

    async fn from_data<T: ToString>(query: String, user_data: T) -> Result<Self, reqwest::Error> {
        let mut vars = HashMap::new();
        vars.insert("book", user_data.to_string());

        Self::new(query, vars).await
    }

    async fn new(query: String, vars: HashMap<&str, String>) -> Result<Self, reqwest::Error>  {
        let resp = graphql_req(query, vars).await?;

        let data = &resp["data"]["books"][0];

        Ok(Book{
            activities_count: {
                data["activities_count"].as_u64().unwrap()
            },
            alternative_titles: {
                get_str_vec(data["alternative_titles"].as_array())
            },
            audio_seconds: {
                data["audio_seconds"].as_u64()
            },
            book_category_id: {
                match data["book_category_id"].as_u64().unwrap() {
                    1 => BookCategory::Book,
                    2 => BookCategory::Novella,
                    3 => BookCategory::ShortStory,
                    4 => BookCategory::GraphicNovel,
                    5 => BookCategory::FanFiction,
                    6 => BookCategory::ResearchPaper,
                    7 => BookCategory::Poetry,
                    8 => BookCategory::Collection,
                    9 => BookCategory::WebNovel,
                    10 => BookCategory::LightNovel,
                    // defaults to book
                    _ => BookCategory::Book
                }
            },
            book_status_id: {
                match data["book_status_id"].as_u64().unwrap() {
                    1 => BookStatus::OK,
                    2 => BookStatus::ToReview,
                    3 => BookStatus::Deleted,
                    4 => BookStatus::Deduplicated,
                    // defaults to ok
                    _ => BookStatus::OK
                }
            },
            cached_contributors: {
                data["cached_contributors"].clone()
            },
            cached_featured_series: {
                data.get("cached_featured_series").cloned()
            },
            cached_header_image: {
                if data["cached_header_image"].is_null() {
                    None
                } else {
                    Some(Image::new(data["cached_header_image"].clone()))
                }
            },
            cached_image: {
                Image::new(data["cached_image"].clone())
            },
            cached_similar_book_ids: {
                data["cached_similar_book_ids"].clone()
            },
            cached_similar_books_updated_at: {
                get_plaindatetime_from_resp(data, "cached_similar_books_updated_at")
            },
            cached_tags: {
                data["cached_tags"].clone()
            },
            canonical_id: {
                data["canonical_id"].as_u64()
            },
            compilation: {
                get_bool_from_resp(data, "compilation")
            },
            created_at: {
                get_plaindatetime_from_resp(data, "created_at").unwrap()
            },
            created_by_user_id: {
                data["created_by_user_id"].as_u64()
            },
            curation_status: {
                data["curation_status"].clone()
            },
            default_audio_edition_id: {
                get_u64_from_resp(data, "default_audio_edition_id")
            },
            default_cover_edition_id: {
                get_u64_from_resp(data, "default_cover_edition_id")
            },
            default_ebook_edition_id: {
                get_u64_from_resp(data, "default_ebook_edition_id")
            },
            default_physical_edition_id: {
                get_u64_from_resp(data, "default_physical_edition_id")
            },
            description: {
                get_str_from_resp(data, "description")
            },
            editions_count: {
                get_u64_from_resp(data, "editions_count").unwrap()
            },
            featured_book_series_id: {
                get_u64_from_resp(data, "featured_book_series_id")
            },
            header_image_id: {
                get_u64_from_resp(data, "header_image_id")
            },
            headline: {
                get_str_from_resp(data, "headline")
            },
            id: {
                get_u64_from_resp(data, "id").unwrap()
            },
            image_id: {
                get_u64_from_resp(data, "image_id")
            },
            import_platform_id: {
                get_u64_from_resp(data, "import_platform_id").unwrap()
            },
            is_partial_book: {
                data["is_partial_book"].as_bool()
            },
            journals_count: {
                get_u64_from_resp(data, "journals_count").unwrap()
            },
            links: {
                data["links"].clone()
            },
            lists_count: {
                get_u64_from_resp(data, "lists_count")
            },
            literary_type_id: {
                match data["literary_type_id"].as_u64() {
                    Some(1) => Some(LiteraryType::Fiction),
                    Some(2) => Some(LiteraryType::NonFiction),
                    _ => None,
                }
            },
            locked: {
                get_bool_from_resp(data, "locked")
            },
            pages: {
                get_u64_from_resp(data, "pages")
            },
            parent_book_id: {
                get_u64_from_resp(data, "parent_book_id")
            },
            prompts_count: {
                get_u64_from_resp(data, "prompts_count").unwrap()
            },
            rating: {
                data["rating"].as_f64()
            },
            ratings_count: {
                get_u64_from_resp(data, "ratings_count").unwrap()
            },
            ratings_distribution: {
                data["ratings_distribution"].clone()
            },
            release_date: {
                get_date_from_resp(data, "release_date")
            },
            release_year: {
                get_u64_from_resp(data, "release_year")
            },
            reviews_count: {
                get_u64_from_resp(data, "reviews_count").unwrap()
            },
            slug: {
                get_str_from_resp(data, "slug")
            },
            state: {
                match data["state"].as_str() {
                    Some("pending") => RecordState2::Pending,
                    Some("processing") => RecordState2::Processing,
                    Some("normalized") => RecordState2::Normalized,
                    Some("processed") => RecordState2::Processed,
                    Some("error") => RecordState2::Error,
                    Some("duplicate") => RecordState2::Duplicate,
                    // lacking another option, we do this
                    _ => RecordState2::Error,
                }
            },
            subtitle: {
                get_str_from_resp(data, "subtitle")
            },
            title: {
                get_str_from_resp(data, "title")
            },
            updated_at: {
                get_offsetdatetime_from_resp(data, "updated_at")
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