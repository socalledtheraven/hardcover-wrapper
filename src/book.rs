use serde_json::Value;
use time::{Date, OffsetDateTime, PlainDateTime};
use crate::base_hardcover_item::BaseHardcoverItem;
use crate::util::{Link, RecordState2};
use crate::client::{GraphQLResponse};
use crate::HardcoverClient;

#[derive(Debug, Clone)]
pub enum BookCategory {
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

#[derive(Debug, Clone)]
pub enum BookStatus {
    OK,
    ToReview,
    Deleted,
    Deduplicated
}

#[derive(Debug, Clone)]
pub enum LiteraryType {
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

#[derive(Debug, Clone)]
pub struct Rating {
    pub count: u64,
    pub rating: f64,
}

#[derive(Debug, Clone)]
pub struct Book {
    pub activities_count: u64,
    pub alternative_titles: Vec<String>,
    pub audio_seconds: Option<u64>,
    pub book_category_id: BookCategory,
    pub book_status_id: BookStatus,
    pub canonical_id: Option<u64>,
    pub compilation: bool,
    pub created_at: PlainDateTime,
    pub created_by_user_id: Option<u64>,
    pub curation_status: u64,
    pub default_audio_edition_id: Option<u64>,
    pub default_cover_edition_id: Option<u64>,
    pub default_ebook_edition_id: Option<u64>,
    pub default_physical_edition_id: Option<u64>,
    pub description: Option<String>,
    pub editions_count: u64,
    pub featured_book_series_id: Option<u64>,
    pub header_image_id: Option<u64>,
    pub headline: Option<String>,
    pub id: u64,
    pub image_id: Option<u64>,
    pub import_platform_id: u64,
    pub is_partial_book: Option<bool>,
    pub journals_count: u64,
    pub links: Vec<Link>,
    pub lists_count: Option<u64>,
    pub literary_type_id: Option<LiteraryType>,
    pub locked: bool,
    pub pages: Option<u64>,
    pub parent_book_id: Option<u64>,
    pub prompts_count: u64,
    pub rating: Option<f64>,
    pub ratings_count: u64,
    pub ratings_distribution: Vec<Rating>,
    pub release_date: Option<Date>,
    pub release_year: Option<u64>,
    pub reviews_count: u64,
    pub slug: Option<String>,
    pub state: RecordState2,
    pub subtitle: Option<String>,
    pub title: Option<String>,
    pub updated_at: Option<OffsetDateTime>,
    pub users_count: u64,
    pub users_read_count: u64,
}

impl BaseHardcoverItem for Book {
    async fn from_id(id: u64, client: &HardcoverClient) -> Result<Self, reqwest::Error> {
        let query = r#"
        query GetBook($id: Int!) {
          books_by_pk(id: $id) {"#.to_string() + QUERY_FIELDS + r#"
          }
        }
        "#;

        let data = Self::from_data(query, id, client).await?;

        Ok(Self::new(data["books_by_pk"].clone()))
    }

    fn new(data: Value) -> Self {
        Book {
            activities_count: {
                data.get_u64("activities_count").unwrap()
            },
            alternative_titles: {
                data.get_str_vec("alternative_titles")
            },
            audio_seconds: {
                data.get_u64("audio_seconds")
            },
            book_category_id: {
                match data.get_u64("book_category_id").unwrap() {
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
                    _ => panic!("Unknown book_category_id: {}", data["book_category_id"].as_u64().unwrap())
                }
            },
            book_status_id: {
                match data.get_u64("book_status_id").unwrap() {
                    1 => BookStatus::OK,
                    2 => BookStatus::ToReview,
                    3 => BookStatus::Deleted,
                    4 => BookStatus::Deduplicated,
                    _ => panic!("Unknown book_status_id: {}", data["book_status_id"])
                }
            },
            canonical_id: {
                data.get_u64("canonical_id")
            },
            compilation: {
                data.get_bool("compilation").unwrap()
            },
            created_at: {
                data.get_plaindt("created_at").unwrap()
            },
            created_by_user_id: {
                data.get_u64("created_by_user_id")
            },
            curation_status: {
                data.get_u64("curation_status").unwrap()
            },
            default_audio_edition_id: {
                data.get_u64("default_audio_edition_id")
            },
            default_cover_edition_id: {
                data.get_u64("default_cover_edition_id")
            },
            default_ebook_edition_id: {
                data.get_u64("default_ebook_edition_id")
            },
            default_physical_edition_id: {
                data.get_u64("default_physical_edition_id")
            },
            description: {
                data.get_str("description")
            },
            editions_count: {
                data.get_u64("editions_count").unwrap()
            },
            featured_book_series_id: {
                data.get_u64("featured_book_series_id")
            },
            header_image_id: {
                data.get_u64("header_image_id")
            },
            headline: {
                data.get_str("headline")
            },
            id: {
                data.get_u64("id").unwrap()
            },
            image_id: {
                data.get_u64("image_id")
            },
            import_platform_id: {
                data.get_u64("import_platform_id").unwrap()
            },
            is_partial_book: {
                data.get_bool("is_partial_book")
            },
            journals_count: {
                data.get_u64("journals_count").unwrap()
            },
            links: {
                data.get_link_vec("links")
            },
            lists_count: {
                data.get_u64("lists_count")
            },
            literary_type_id: {
                match data.get_u64("literary_type_id") {
                    Some(1) => Some(LiteraryType::Fiction),
                    Some(2) => Some(LiteraryType::NonFiction),
                    _ => None,
                }
            },
            locked: {
                data.get_bool("locked").unwrap()
            },
            pages: {
                data.get_u64("pages")
            },
            parent_book_id: {
                data.get_u64("parent_book_id")
            },
            prompts_count: {
                data.get_u64("prompts_count").unwrap()
            },
            rating: {
                data.get_f64("rating")
            },
            ratings_count: {
                data.get_u64("ratings_count").unwrap()
            },
            ratings_distribution: {
                data.get_rating_vec("ratings_distribution")
            },
            release_date: {
                data.get_date("release_date")
            },
            release_year: {
                data.get_u64("release_year")
            },
            reviews_count: {
                data.get_u64("reviews_count").unwrap()
            },
            slug: {
                data.get_str("slug")
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
                data.get_str("subtitle")
            },
            title: {
                data.get_str("title")
            },
            updated_at: {
                data.get_offsetdt("updated_at")
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