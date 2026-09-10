use crate::base_hardcover_item::BaseHardcoverItem;
use crate::date_parsing;
use crate::util::{Link, BookRecordState};
use crate::HardcoverClient;
use serde::Deserialize;
use serde_json::Value;
use time::{Date, OffsetDateTime, PlainDateTime};

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

#[derive(Debug, Clone, Deserialize)]
pub struct Rating {
    pub count: u64,
    pub rating: f64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Book {
    pub activities_count: u64,
    pub alternative_titles: Vec<String>,
    pub audio_seconds: Option<u64>,
    pub book_category_id: BookCategory,
    pub book_status_id: BookStatus,
    pub canonical_id: Option<u64>,
    pub compilation: bool,

    #[serde(deserialize_with = "date_parsing::plain_datetime")]
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

    #[serde(deserialize_with = "date_parsing::date_optional")]
    pub release_date: Option<Date>,
    pub release_year: Option<u64>,
    pub reviews_count: u64,
    pub slug: Option<String>,
    pub state: BookRecordState,
    pub subtitle: Option<String>,
    pub title: Option<String>,

    #[serde(deserialize_with = "date_parsing::offset_datetime_optional")]
    pub updated_at: Option<OffsetDateTime>,
    pub users_count: u64,
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
