use serde::{Deserialize, Serialize};
use serde_json::Value;
use time::{Date, PlainDateTime};
use crate::book::Book;
use crate::contribution::Contribution;
use crate::country::Country;
use crate::enums::{RecordState, RecordState3};
use crate::image::Image;
use crate::language::Language;
use crate::mappings::Mapping;
use crate::publisher::Publisher;
use crate::reading_format::ReadingFormat as Reading;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct Edition {
    // todo!
    alternative_titles: Value,
    asin: Option<String>,
    audio_seconds: Option<u64>,
    book: Box<Book>,
    book_id: u64,
    book_mappings: Vec<Mapping>,
    // todo!
    cached_contributors: Value,
    cached_image: Image,
    // todo!
    cached_tags: Value,
    canonical_id: Option<u64>,
    compilation: bool,
    contributions: Vec<Contribution>,
    country: Country,
    country_id: Option<u64>,
    created_at: PlainDateTime,
    created_by_user_id: Option<u64>,
    // todo!
    curation_status: Value,
    edition_format: Option<EditionFormat>,
    edition_information: Option<String>,
    id: u64,
    cover_image: Option<Image>,
    image_id: Option<u64>,
    images: Vec<Image>,
    isbn_10: Option<String>,
    isbn_10_valid: Option<bool>,
    isbn_13: Option<String>,
    isbn_13_valid: Option<bool>,
    isbns_match: Option<bool>,
    language: Option<Language>,
    language_id: Option<u64>,
    // todo!
    list_books: Vec<Value>,
    lists_count: u64,
    locked: bool,
    normalized_at: Option<PlainDateTime>,
    object_type: String,
    original_book_id: Option<u64>,
    pages: Option<u64>,
    physical_format: Option<String>,
    physical_information: Option<String>,
    publisher: Option<Publisher>,
    publisher_id: Option<u64>,
    rating: Option<f32>,
    reading_format: Option<Reading>,
    // todo!
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