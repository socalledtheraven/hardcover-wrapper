use crate::base_hardcover_item::BaseHardcoverItem;
use crate::date_parsing;
use crate::util::{PrivacySetting, ReadingStatus};
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

#[derive(Debug, Clone, Deserialize)]
pub struct UserBook {
    pub book_id: u64,
    #[serde(deserialize_with = "date_parsing::offset_datetime")]
    pub created_at: OffsetDateTime,
    #[serde(deserialize_with = "date_parsing::date")]
    pub date_added: Date,
    pub edition_id: Option<u64>,
    #[serde(deserialize_with = "date_parsing::date_optional")]
    pub first_read_date: Option<Date>,
    #[serde(deserialize_with = "date_parsing::date_optional")]
    pub first_started_reading_date: Option<Date>,
    pub has_review: bool,
    pub id: u64,
    pub imported: Option<bool>,
    #[serde(deserialize_with = "date_parsing::date_optional")]
    pub last_read_date: Option<Date>,
    pub likes_count: u64,
    pub media_url: Option<String>,
    #[serde(deserialize_with = "date_parsing::plain_datetime_optional")]
    pub merged_at: Option<PlainDateTime>,
    pub mod_status: u64,
    pub object_type: String,
    pub original_book_id: Option<u64>,
    pub original_edition_id: Option<u64>,
    pub owned: bool,
    pub owned_copies: Option<u64>,
    pub privacy_setting_id: PrivacySetting,
    pub private_notes: Option<String>,
    pub rating: Option<f64>,
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
    #[serde(deserialize_with = "date_parsing::plain_datetime_optional")]
    pub reviewed_at: Option<PlainDateTime>,
    pub sponsored_review: bool,
    pub starred: bool,
    #[serde(rename = "status_id")]
    pub status: ReadingStatus,
    #[serde(deserialize_with = "date_parsing::offset_datetime_optional")]
    pub updated_at: Option<OffsetDateTime>,
    pub url: Option<String>,
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
