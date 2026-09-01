use serde::{Deserialize, Serialize};
use serde_json::Value;
use time::PlainDateTime;
use crate::book::Book;
use crate::contribution::Contribution;
use crate::enums::{Gender, RecordState};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct Character {
    biography: Option<String>,
    book_characters: Vec<Book>,
    books_count: u64,
    // todo!
    cached_tags: Value,
    canonical_books_count: u64,
    canonical_id: Option<u64>,
    contributions: Vec<Contribution>,
    created_at: PlainDateTime,
    gender_id: Option<Gender>,
    has_disability: Option<bool>,
    id: u64,
    // todo!
    image_id: Option<u64>,
    is_lgbtq: Option<bool>,
    is_poc: Option<bool>,
    locked: Option<bool>,
    name: String,
    object_type: String,
    openlibrary_url: Option<String>,
    slug: String,
    state: RecordState,
    updated_at: PlainDateTime,
    user_id: Option<u64>,
}