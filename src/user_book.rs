use serde::{Deserialize, Serialize};
use serde_json::Value;
use time::{Date, OffsetDateTime, PlainDateTime};
use crate::book::Book;
use crate::edition::Edition;
use crate::enums::PrivacySetting;
use crate::like::Like;
use crate::user::User;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct UserBook {
    book: Book,
    book_id: u32,
    created_at: OffsetDateTime,
    date_added: Date,
    edition: Edition,
    edition_id: Option<u32>,
    first_read_date: Option<Date>,
    first_started_reading_date: Option<Date>,
    // todo!
    followers: Vec<Value>,
    has_review: bool,
    id: u32,
    imported: Option<bool>,
    last_read_date: Option<Date>,
    likes: Vec<Like>,
    likes_count: u32,
    media_url: Option<String>,
    merged_at: Option<PlainDateTime>,
    mod_status: u32,
    object_type: String,
    original_book_id: Option<u32>,
    original_edition_id: Option<u32>,
    owned: bool,
    owned_copies: Option<u32>,
    privacy_setting: PrivacySetting,
    privacy_setting_id: u32,
    private_notes: Option<String>,
    rating: Option<f32>	,
    read_count: u32,
    // todo!
    reading_journal_summary: Vec<Value>,
    // todo!
    reading_journals: Vec<Value>,
    recommended_by: Option<String>,
    recommended_for: Option<String>,
    referrer: Option<User>,
    referrer_user_id: Option<u32>,
    review: Option<String>,
    review_has_spoilers: bool,
    review_length: u32,
    review_migrated: Option<bool>,
    review_raw: Option<String>,
    // todo!
    review_slate: Value,
    reviewed_at: Option<PlainDateTime>,
    sponsored_review: bool,
    starred: bool,
    status_id: u32,
    updated_at: Option<OffsetDateTime>,
    url: Option<String>,
    user: User,
    // todo!
    user_book_reads: Vec<Value>,
    user_book_status: ReadingStatus,
    user_books: Vec<UserBook>,
    user_id: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
enum ReadingStatus {
	WantToRead,
	CurrentlyReading,
	Read,
	Paused,
	DidNotFinish,
	Ignored,
}