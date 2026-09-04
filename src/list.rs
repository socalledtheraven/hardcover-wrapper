use serde::{Deserialize, Serialize};
use serde_json::Value;
use time::{OffsetDateTime, PlainDateTime};
use crate::enums::PrivacySetting;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct List {
    books_count: u64,
    created_at: Option<PlainDateTime>,
    default_view: String,
    description: Option<String>,
    featured: bool,
    featured_profile: bool,
    followers_count: Option<u64>,
    id: u64,
    imported: bool,
    likes_count: u64,
    name: String,
    object_type: String,
    privacy_setting_id: PrivacySetting,
    public: bool,
    ranked: bool,
    slug: Option<String>,
    updated_at: Option<OffsetDateTime>,
    url: Option<String>,
    user_id: u64,
}

