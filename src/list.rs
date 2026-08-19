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
    books_count: u32,
    created_at: Option<PlainDateTime>,
    default_view: String,
    description: Option<String>,
    featured: bool,
    featured_profile: bool,
    // todo!
    followed_lists: Vec<Value>,
    // todo!
    followers: Vec<Value>,
    followers_count: Option<u32>,
    id: u32,
    imported: bool,
    likes: Vec<Like>,
    likes_count: u32,
    list_books: Vec<ListBook>,
    name: String,
    object_type: String,
    privacy_setting: PrivacySetting,
    privacy_setting_id: u32,
    public: bool,
    ranked: bool,
    slug: Option<String>,
    updated_at: Option<OffsetDateTime>,
    url: Option<String>,
    user: User,
    user_id: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct ListBook {
    id: Option<u32>,
    list_id: Option<u32>,
    book_id: Option<u32>,
    edition_id: Option<u32>,
    position: Option<u32>,
    date_added: Option<OffsetDateTime>,
    book: Option<Book>,
    list: Option<List>,
    edition: Option<Edition>,
}