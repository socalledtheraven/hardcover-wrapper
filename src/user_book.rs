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
pub enum ReadingStatus {
    WantToRead,
    CurrentlyReading,
    Read,
    Paused,
    DidNotFinish,
    Ignored,
}

#[derive(Debug, Clone)]
pub struct UserBook {
    pub book_id: u64,
    pub created_at: OffsetDateTime,
    pub date_added: Date,
    pub edition_id: Option<u64>,
    pub first_read_date: Option<Date>,
    pub first_started_reading_date: Option<Date>,
    pub has_review: bool,
    pub id: u64,
    pub imported: Option<bool>,
    pub last_read_date: Option<Date>,
    pub likes_count: u64,
    pub media_url: Option<String>,
    pub merged_at: Option<PlainDateTime>,
    pub mod_status: u64,
    pub object_type: String,
    pub original_book_id: Option<u64>,
    pub original_edition_id: Option<u64>,
    pub owned: bool,
    pub owned_copies: Option<u64>,
    pub privacy_setting_id: PrivacySetting,
    pub private_notes: Option<String>,
    pub rating: Option<f64>	,
    pub read_count: u64,
    pub recommended_by: Option<String>,
    pub recommended_for: Option<String>,
    pub referrer_user_id: Option<u64>,
    pub review: Option<String>,
    pub review_has_spoilers: bool,
    pub review_length: u64,
    pub review_migrated: Option<bool>,
    pub review_raw: Option<String>,
    // we leave this as Value because it's the data for the Slate editor
    pub review_slate: Value,
    pub reviewed_at: Option<PlainDateTime>,
    pub sponsored_review: bool,
    pub starred: bool,
    pub status_id: ReadingStatus,
    pub updated_at: Option<OffsetDateTime>,
    pub url: Option<String>,
    pub user_id: u64,
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