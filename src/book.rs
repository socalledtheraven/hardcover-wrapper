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
    activities_count: u64,
    // todo!
    alternative_titles: Value,
    audio_seconds: Option<u64>,
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
    canonical_id: Option<u64>,
    // todo!
    collection_import_results: Vec<Value>,
    compilation: bool,
    contributions: Vec<Contribution>,
    created_at: PlainDateTime,
    created_by_user_id: Option<u64>,
    // todo!
    curation_status: Value,
    default_audio_edition: Option<Edition>,
    default_audio_edition_id: Option<u64>,
    default_cover_edition: Option<Edition>,
    default_cover_edition_id: Option<u64>,
    default_ebook_edition: Option<Edition>,
    default_ebook_edition_id: Option<u64>,
    default_physical_edition: Option<Edition>,
    default_physical_edition_id: Option<u64>,
    description: Option<String>,
    editions_count: u64,
    featured_book_series: Option<Series>,
    featured_book_series_id: Option<u64>,
    header_image_id: Option<u64>,
    headline: Option<String>,
    id: u64,
    images: Vec<Image>,
    // todo!
    import_platform_id: u64,
    is_partial_book: Option<bool>,
    journals_count: u64,
    // todo!
    links: Value,
    list_books: Vec<List>,
    lists_count: Option<u64>,
    literary_type_id: Option<LiteraryType>,
    locked: bool,
    pages: Option<u64>,
    parent_book_id: Option<u64>,
    prompt_answers: Vec<PromptAnswer>,
    prompt_summaries: Vec<PromptBook>,
    prompts_count: u64,
    rating: Option<f32>,
    ratings_count: u64,
    // todo!
    ratings_distribution: Value,
    release_date: Option<Date>,
    release_year: Option<u64>,
    reviews_count: u64,
    slug: Option<String>,
    state: Option<RecordState2>,
    subtitle: Option<String>,
    // todo!
    taggable_counts: Vec<Value>,
    taggings: Vec<Tagging>,
    title: Option<String>,
    updated_at: Option<OffsetDateTime>,
    user_books: Vec<UserBook>,
    users_count: u64,
    users_read_count: u64,
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