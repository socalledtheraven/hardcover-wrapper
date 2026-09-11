//! Book model representing books and work-level metadata.

use crate::utils::base_hardcover_item::BaseHardcoverItem;
use crate::utils::date_parsing;
use crate::utils::enums::{BookRecordState, Link};
use crate::HardcoverClient;
use serde::Deserialize;
use serde_json::Value;
use time::{Date, OffsetDateTime, PlainDateTime};

/// Category/format type classification of a book work.
#[derive(Debug, Clone, Deserialize)]
#[serde(from = "u64")]
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
    LightNovel,
}

impl From<u64> for BookCategory {
    fn from(value: u64) -> Self {
        match value {
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
            _ => panic!("Unknown book_category_id: {value}"),
        }
    }
}

/// Verification/curation status of a book record.
#[derive(Debug, Clone, Deserialize)]
#[serde(from = "u64")]
pub enum BookStatus {
    Ok,
    ToReview,
    Deleted,
    Deduplicated,
}

impl From<u64> for BookStatus {
    fn from(value: u64) -> Self {
        match value {
            1 => BookStatus::Ok,
            2 => BookStatus::ToReview,
            3 => BookStatus::Deleted,
            4 => BookStatus::Deduplicated,
            _ => panic!("Unknown book_status_id: {value}"),
        }
    }
}

/// Classification of a book as fiction or non-fiction.
#[derive(Debug, Clone, Deserialize)]
#[serde(from = "u64")]
pub enum LiteraryType {
    Fiction,
    NonFiction,
}

impl From<u64> for LiteraryType {
    fn from(value: u64) -> Self {
        match value {
            1 => LiteraryType::Fiction,
            2 => LiteraryType::NonFiction,
            _ => panic!("Unknown literary_type_id: {value}"),
        }
    }
}

const QUERY_FIELDS: &str = r#"
activities_count
alternative_titles
audio_seconds
book_category_id
book_status_id
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

/// Aggregate rating count for a specific star rating score.
#[derive(Debug, Clone, Deserialize)]
pub struct Rating {
    /// Number of ratings for this score.
    pub count: u64,
    /// The numeric star rating value (e.g. 1.0 to 5.0).
    pub rating: f64,
}

/// Represents a book entry (work-level) in Hardcover.
#[derive(Debug, Clone, Deserialize)]
pub struct Book {
    /// Number of activity feed entries mentioning this book.
    pub activities_count: u64,
    /// Alternative or foreign titles for the book.
    pub alternative_titles: Vec<String>,
    /// Total duration in seconds if an audiobook edition exists.
    pub audio_seconds: Option<u64>,
    /// Category of the book (e.g. Novella, Graphic Novel).
    pub book_category_id: BookCategory,
    /// Editorial status of the record.
    pub book_status_id: BookStatus,
    /// Canonical book ID if this record was merged.
    pub canonical_id: Option<u64>,
    /// Whether this book is a compilation/anthology.
    pub compilation: bool,

    /// Timestamp when this record was created.
    #[serde(deserialize_with = "date_parsing::plain_datetime")]
    pub created_at: PlainDateTime,
    /// User ID of the contributor who created this record.
    pub created_by_user_id: Option<u64>,
    /// Curation review status number.
    pub curation_status: u64,
    /// Default audiobook edition ID.
    pub default_audio_edition_id: Option<u64>,
    /// Default cover image edition ID.
    pub default_cover_edition_id: Option<u64>,
    /// Default ebook edition ID.
    pub default_ebook_edition_id: Option<u64>,
    /// Default physical edition ID.
    pub default_physical_edition_id: Option<u64>,
    /// Full description or synopsis of the book.
    pub description: Option<String>,
    /// Total number of editions linked to this book.
    pub editions_count: u64,
    /// Featured book series relation ID.
    pub featured_book_series_id: Option<u64>,
    /// Header banner image ID.
    pub header_image_id: Option<u64>,
    /// Short tagline or headline for the book.
    pub headline: Option<String>,
    /// Unique identifier for the book.
    pub id: u64,
    /// Cover image ID.
    pub image_id: Option<u64>,
    /// Source platform ID where the book was originally imported from.
    pub import_platform_id: u64,
    /// Whether this record represents a partial/incomplete book entry.
    pub is_partial_book: Option<bool>,
    /// Number of reading journal entries written for this book.
    pub journals_count: u64,
    /// External links associated with the book.
    pub links: Vec<Link>,
    /// Number of user lists containing this book.
    pub lists_count: Option<u64>,
    /// Literary classification (Fiction / NonFiction).
    pub literary_type_id: Option<LiteraryType>,
    /// Whether this record is locked against community edits.
    pub locked: bool,
    /// Page count of the default edition.
    pub pages: Option<u64>,
    /// Parent book ID if this book is part of a parent collection.
    pub parent_book_id: Option<u64>,
    /// Number of community prompts for this book.
    pub prompts_count: u64,
    /// Average user rating score (out of 5.0).
    pub rating: Option<f64>,
    /// Total number of ratings submitted for this book.
    pub ratings_count: u64,
    /// Breakdown of ratings by score.
    pub ratings_distribution: Vec<Rating>,

    /// Original release date of the book.
    #[serde(deserialize_with = "date_parsing::date_optional")]
    pub release_date: Option<Date>,
    /// Original release year.
    pub release_year: Option<u64>,
    /// Total number of written reviews.
    pub reviews_count: u64,
    /// URL slug for the book on Hardcover.
    pub slug: Option<String>,
    /// Processing state of the book record.
    pub state: BookRecordState,
    /// Subtitle of the book.
    pub subtitle: Option<String>,
    /// Primary title of the book.
    pub title: Option<String>,

    /// Timestamp when this record was last updated.
    #[serde(deserialize_with = "date_parsing::offset_datetime_optional")]
    pub updated_at: Option<OffsetDateTime>,
    /// Number of users who have added this book to their library.
    pub users_count: u64,
    /// Number of users who have read this book.
    pub users_read_count: u64,
}

impl BaseHardcoverItem for Book {
    async fn from_id(id: u64, client: &HardcoverClient) -> Result<Self, reqwest::Error> {
        let query = r#"
        query GetBook($id: Int!) {
          books_by_pk(id: $id) {"#
            .to_string()
            + QUERY_FIELDS
            + r#"
          }
        }
        "#;

        let variables = serde_json::json!({ "id": id });
        let data = Self::from_data(query, variables, client).await?;

        Ok(Self::from_value(data["books_by_pk"].clone()))
    }

    fn from_value(data: Value) -> Self {
        serde_json::from_value(data).unwrap()
    }
}
