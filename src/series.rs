use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::author::Author;
use crate::book::Book;
use crate::user::User;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct Series {
    author: Option<Author>,
    author_id: Option<u32>,
    book_series: Vec<Book>,
    books_count: u32,
    canonical: Option<Series>,
    canonical_id: Option<u32>,
    creator: Option<User>,
    description: Option<String>,
    id: u32,
    // todo!
    identifiers: Value,
    is_completed: Option<bool>,
    locked: bool,
    name: String,
    object_type: String,
    primary_books_count: Option<u32>,
    slug: String,
    state: String,
    user_id: Option<u32>,
}