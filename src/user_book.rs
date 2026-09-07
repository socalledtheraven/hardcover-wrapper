use reqwest::Error;
use serde_json::Value;
use time::{Date, OffsetDateTime, PlainDateTime};
use crate::base_hardcover_item::BaseHardcoverItem;
use crate::util::PrivacySetting;
use crate::graphql::GraphQLResponse;

const QUERY_FIELDS: &str = r#"
book_id
created_at
date_added
edition_id
first_read_date
first_started_reading_date
has_review
id
imported
last_read_date
likes_count
media_url
merged_at
mod_status
object_type
original_book_id
original_edition_id
owned
owned_copies
privacy_setting_id
private_notes
rating
read_count
recommended_by
recommended_for
referrer_user_id
review
review_has_spoilers
review_length
review_migrated
review_raw
review_slate
reviewed_at
sponsored_review
starred
status_id
updated_at
url
user_id
"#;

#[derive(Debug, Clone)]
enum ReadingStatus {
    WantToRead,
    CurrentlyReading,
    Read,
    Paused,
    DidNotFinish,
    Ignored,
}

#[derive(Debug, Clone)]
pub(crate) struct UserBook {
    book_id: u64,
    created_at: OffsetDateTime,
    date_added: Date,
    edition_id: Option<u64>,
    first_read_date: Option<Date>,
    first_started_reading_date: Option<Date>,
    has_review: bool,
    id: u64,
    imported: Option<bool>,
    last_read_date: Option<Date>,
    likes_count: u64,
    media_url: Option<String>,
    merged_at: Option<PlainDateTime>,
    mod_status: u64,
    object_type: String,
    original_book_id: Option<u64>,
    original_edition_id: Option<u64>,
    owned: bool,
    owned_copies: Option<u64>,
    privacy_setting_id: PrivacySetting,
    private_notes: Option<String>,
    rating: Option<f64>	,
    read_count: u64,
    recommended_by: Option<String>,
    recommended_for: Option<String>,
    referrer_user_id: Option<u64>,
    review: Option<String>,
    review_has_spoilers: bool,
    review_length: u64,
    review_migrated: Option<bool>,
    review_raw: Option<String>,
    // we leave this as Value because it's the data for the Slate editor
    review_slate: Value,
    reviewed_at: Option<PlainDateTime>,
    sponsored_review: bool,
    starred: bool,
    status_id: ReadingStatus,
    updated_at: Option<OffsetDateTime>,
    url: Option<String>,
    user_id: u64,
}

impl BaseHardcoverItem for UserBook {
    async fn from_id(id: u64) -> Result<Self, Error> {
        let query = r#"
        query GetUserBook($id: Int!) {
          user_books_by_pk(id: $id) {"#.to_string() + QUERY_FIELDS + r#"
          }
        }
        "#;

        let data = Self::from_data(query, id).await?;

        Ok(Self::new(data["user_books_by_pk"].clone()))
    }

    fn new(data: Value) -> Self {
        UserBook {
            book_id: {
                data.get_u64("book_id").unwrap()
            },
            created_at: {
                data.get_offsetdt("created_at").unwrap()
            },
            date_added: {
                data.get_date("date_added").unwrap()
            },
            edition_id: {
                data.get_u64("edition_id")
            },
            first_read_date: {
                data.get_date("first_read_date")
            },
            first_started_reading_date: {
                data.get_date("first_started_reading_date")
            },
            has_review: {
                data.get_bool("has_review").unwrap()
            },
            id: {
                data.get_u64("id").unwrap()
            },
            imported: {
                data.get_bool("imported")
            },
            last_read_date: {
                data.get_date("last_read_date")
            },
            likes_count: {
                data.get_u64("likes_count").unwrap()
            },
            media_url: {
                data.get_str("media_url")
            },
            merged_at: {
                data.get_plaindt("merged_at")
            },
            mod_status: {
                data.get_u64("mod_status").unwrap()
            },
            object_type: {
                data.get_str("object_type").unwrap()
            },
            original_book_id: {
                data.get_u64("original_book_id")
            },
            original_edition_id: {
                data.get_u64("original_edition_id")
            },
            owned: {
                data.get_bool("owned").unwrap()
            },
            owned_copies: {
                data.get_u64("owned_copies")
            },
            privacy_setting_id: {
                data.get_privacysetting("privacy_setting_id")
            },
            private_notes: {
                data.get_str("private_notes")
            },
            rating: {
                data.get_f64("rating")
            },
            read_count: {
                data.get_u64("read_count").unwrap()
            },
            recommended_by: {
                data.get_str("recommended_by")
            },
            recommended_for: {
                data.get_str("recommended_for")
            },
            referrer_user_id: {
                data.get_u64("referrer_user_id")
            },
            review: {
                data.get_str("review")
            },
            review_has_spoilers: {
                data.get_bool("review_has_spoilers").unwrap()
            },
            review_length: {
                data.get_u64("review_length").unwrap()
            },
            review_migrated: {
                data.get_bool("review_migrated")
            },
            review_raw: {
                data.get_str("review_raw")
            },
            review_slate: {
                data["review_slate"].clone()
            },
            reviewed_at: {
                data.get_plaindt("reviewed_at")
            },
            sponsored_review: {
                data.get_bool("sponsored_review").unwrap()
            },
            starred: {
                data.get_bool("starred").unwrap()
            },
            status_id: {
                match data["status_id"].as_u64() {
                    Some(1) => ReadingStatus::WantToRead,
                    Some(2) => ReadingStatus::CurrentlyReading,
                    Some(3) => ReadingStatus::Read,
                    Some(4) => ReadingStatus::Paused,
                    Some(5) => ReadingStatus::DidNotFinish,
                    Some(6) => ReadingStatus::Ignored,
                    _ => panic!("Unknown reading status"),
                }
            },
            updated_at: {
                data.get_offsetdt("updated_at")
            },
            url: {
                data.get_str("url")
            },
            user_id: {
                data.get_u64("user_id").unwrap()
            },
        }
    }
}