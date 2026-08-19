use serde::{Deserialize, Serialize};
use serde_json::Value;
use time::{OffsetDateTime, PlainDateTime};
use crate::book::Book;
use crate::edition::Edition;
use crate::enums::PrivacySetting;
use crate::like::Like;
use crate::user::User;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct List {
    books_count: u64,
    created_at: Option<PlainDateTime>,
    default_view: String,
    description: Option<String>,
    featured: bool,
    featured_profile: bool,
    // todo!
    followed_lists: Vec<Value>,
    // todo!
    followers: Vec<Value>,
    followers_count: Option<u64>,
    id: u64,
    imported: bool,
    likes: Vec<Like>,
    likes_count: u64,
    list_books: Vec<ListBook>,
    name: String,
    object_type: String,
    privacy_setting: PrivacySetting,
    privacy_setting_id: u64,
    public: bool,
    ranked: bool,
    slug: Option<String>,
    updated_at: Option<OffsetDateTime>,
    url: Option<String>,
    user: User,
    user_id: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct ListBook {
    id: Option<u64>,
    list_id: Option<u64>,
    book_id: Option<u64>,
    edition_id: Option<u64>,
    position: Option<u64>,
    date_added: Option<OffsetDateTime>,
    book: Option<Book>,
    list: Option<List>,
    edition: Option<Edition>,
}