//! UserBook model representing a book in a user's library, shelf state, reviews, and ratings.

use crate::utils::base_hardcover_item::BaseHardcoverItem;
use crate::utils::date_parsing;
use crate::utils::enums::{PrivacySetting, ReadingStatus};
use crate::HardcoverClient;
use reqwest::Error;
use serde::Deserialize;
use serde_json::Value;
use time::{Date, OffsetDateTime, PlainDateTime};

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

/// Represents a book entry on a user's personal shelf/library.
#[derive(Debug, Clone, Deserialize)]
pub struct UserBook {
    /// ID of the saved book.
    pub book_id: u64,
    /// Creation timestamp.
    #[serde(deserialize_with = "date_parsing::offset_datetime")]
    pub created_at: OffsetDateTime,
    /// Date when the book was added to the user's shelf.
    #[serde(deserialize_with = "date_parsing::date")]
    pub date_added: Date,
    /// Specific edition ID selected by the user.
    pub edition_id: Option<u64>,
    /// Date when the user first completed reading the book.
    #[serde(deserialize_with = "date_parsing::date_optional")]
    pub first_read_date: Option<Date>,
    /// Date when the user started reading the book.
    #[serde(deserialize_with = "date_parsing::date_optional")]
    pub first_started_reading_date: Option<Date>,
    /// Whether the user wrote a review for this book.
    pub has_review: bool,
    /// Unique identifier for this user-book relationship.
    pub id: u64,
    /// Whether this entry was imported from an external service.
    pub imported: Option<bool>,
    /// Date when the user last finished reading the book.
    #[serde(deserialize_with = "date_parsing::date_optional")]
    pub last_read_date: Option<Date>,
    /// Number of likes received by this shelf entry or review.
    pub likes_count: u64,
    /// Media URL (e.g. video review link).
    pub media_url: Option<String>,
    /// Timestamp when merged, if applicable.
    #[serde(deserialize_with = "date_parsing::plain_datetime_optional")]
    pub merged_at: Option<PlainDateTime>,
    /// Moderation status number.
    pub mod_status: u64,
    /// GraphQL object type name.
    pub object_type: String,
    /// Original book ID if reassigned.
    pub original_book_id: Option<u64>,
    /// Original edition ID if reassigned.
    pub original_edition_id: Option<u64>,
    /// Whether the user owns a copy of this book.
    pub owned: bool,
    /// Number of copies owned.
    pub owned_copies: Option<u64>,
    /// Privacy visibility level.
    pub privacy_setting_id: PrivacySetting,
    /// Private user notes not shown to the public.
    pub private_notes: Option<String>,
    /// User's star rating score (e.g. 1.0 to 5.0).
    pub rating: Option<f64>,
    /// Number of times the user has re-read this book.
    pub read_count: u64,
    /// Who recommended this book to the user.
    pub recommended_by: Option<String>,
    /// Who the user recommends this book to.
    pub recommended_for: Option<String>,
    /// User ID of the user who referred this book.
    pub referrer_user_id: Option<u64>,
    /// Formatted review body text.
    pub review: Option<String>,
    /// Whether the review contains spoilers.
    pub review_has_spoilers: bool,
    /// Character length of the review.
    pub review_length: u64,
    /// Whether the review was migrated from another platform.
    pub review_migrated: Option<bool>,
    /// Raw unformatted review text.
    pub review_raw: Option<String>,
    /// Slate editor JSON representation of the rich text review.
    pub review_slate: Value,
    /// Timestamp when the review was submitted.
    #[serde(deserialize_with = "date_parsing::plain_datetime_optional")]
    pub reviewed_at: Option<PlainDateTime>,
    /// Whether this review is a sponsored/promotional post.
    pub sponsored_review: bool,
    /// Whether this book is starred/favorited.
    pub starred: bool,
    /// Reading status shelf (e.g. WantToRead, CurrentlyReading, Read).
    #[serde(rename = "status_id")]
    pub status: ReadingStatus,
    /// Last update timestamp.
    #[serde(deserialize_with = "date_parsing::offset_datetime_optional")]
    pub updated_at: Option<OffsetDateTime>,
    /// Web URL to this shelf entry.
    pub url: Option<String>,
    /// User ID of the shelf owner.
    pub user_id: u64,
}

impl BaseHardcoverItem for UserBook {
    async fn from_id(id: u64, client: &HardcoverClient) -> Result<Self, Error> {
        let query = r#"
        query GetUserBook($id: Int!) {
          user_books_by_pk(id: $id) {"#
            .to_string()
            + QUERY_FIELDS
            + r#"
          }
        }
        "#;

        let variables = serde_json::json!({ "id": id });
        let data = Self::from_data(query, variables, client).await?;

        Ok(Self::from_value(data["user_books_by_pk"].clone()))
    }

    fn from_value(data: Value) -> Self {
        serde_json::from_value(data).unwrap()
    }
}
