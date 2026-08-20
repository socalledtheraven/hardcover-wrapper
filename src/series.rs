use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::author::Author;
use crate::book::Book;
use crate::user::User;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct Series {
    author: Option<Author>,
    author_id: Option<u64>,
    book_series: Vec<Book>,
    books_count: u64,
    canonical: Option<Box<Series>>,
    canonical_id: Option<u64>,
    creator: Option<User>,
    description: Option<String>,
    id: u64,
    // todo!
    identifiers: Value,
    is_completed: Option<bool>,
    locked: bool,
    name: String,
    object_type: String,
    primary_books_count: Option<u64>,
    slug: String,
    state: String,
    user_id: Option<u64>,
}