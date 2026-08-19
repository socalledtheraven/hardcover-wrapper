use serde::{Deserialize, Serialize};
use serde_json::Value;
use time::{Date, OffsetDateTime, PlainDateTime};
use crate::character::Character;
use crate::contribution::Contribution;
use crate::edition::Edition;
use crate::enums::{RecordState2};
use crate::image::Image;
use crate::list::List;
use crate::mappings::Mapping;
use crate::prompt::{PromptAnswer, PromptBook};
use crate::series::Series;
use crate::tagging::Tagging;
use crate::user_book::UserBook;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct Book {
    activities_count: u32,
    // todo!
    alternative_titles: Value,
    audio_seconds: Option<u32>,
    book_category_id: BookCategory,
    book_characters: Vec<Character>,
    book_mappings: Vec<Mapping>,
    book_series: Vec<Series>,
    // todo!
    book_status: Value,
    book_status_id: BookStatus,
    // todo!
    cached_contributors: Value,
    // todo!
    cached_featured_series: Option<Value>,
    cached_header_image: Image,
    cached_image: Image,
    // todo!
    cached_tags: Value,
    canonical: Option<Box<Book>>,
    canonical_id: Option<u32>,
    // todo!
    collection_import_results: Vec<Value>,
    compilation: bool,
    contributions: Vec<Contribution>,
    created_at: PlainDateTime,
    created_by_user_id: Option<u32>,
    // todo!
    curation_status: Value,
    default_audio_edition: Option<Edition>,
    default_audio_edition_id: Option<u32>,
    default_cover_edition: Option<Edition>,
    default_cover_edition_id: Option<u32>,
    default_ebook_edition: Option<Edition>,
    default_ebook_edition_id: Option<u32>,
    default_physical_edition: Option<Edition>,
    default_physical_edition_id: Option<u32>,
    description: Option<String>,
    editions: Vec<Edition>,
    editions_count: u32,
    featured_book_series: Option<Series>,
    featured_book_series_id: Option<u32>,
    header_image_id: Option<u32>,
    headline: Option<String>,
    id: u32,
    images: Vec<Image>,
    // todo!
    import_platform_id: u32,
    is_partial_book: Option<bool>,
    journals_count: u32,
    // todo!
    links: Value,
    list_books: Vec<List>,
    lists_count: Option<u32>,
    literary_type_id: Option<LiteraryType>,
    locked: bool,
    pages: Option<u32>,
    parent_book: Option<Box<Book>>,
    parent_book_id: Option<u32>,
    prompt_answers: Vec<PromptAnswer>,
    prompt_summaries: Vec<PromptBook>,
    prompts_count: u32,
    rating: Option<f32>,
    ratings_count: u32,
    // todo!
    ratings_distribution: Value,
    release_date: Option<Date>,
    release_year: Option<u32>,
    reviews_count: u32,
    slug: Option<String>,
    state: Option<RecordState2>,
    subtitle: Option<String>,
    // todo!
    taggable_counts: Vec<Value>,
    taggings: Vec<Tagging>,
    title: Option<String>,
    updated_at: Option<OffsetDateTime>,
    user_books: Vec<UserBook>,
    users_count: u32,
    users_read_count: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
enum BookCategory {
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
enum BookStatus {
    OK,
    ToReview,
    Deleted,
    Deduplicated
}

#[derive(Debug, Serialize, Deserialize, Clone)]
enum LiteraryType {
    Fiction,
    NonFiction
}