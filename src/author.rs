use serde::{Deserialize, Serialize};
use serde_json::Value;
use time::Date;
use crate::contribution::Contribution;
use crate::enums::{Gender, RecordState};
use crate::image::Image;
use crate::user::User;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct Author {
    alias_id: Option<u64>,
    // todo!
    alternate_names: Value,
    bio: Option<String>,
    books_count: u64,
    born_date: Option<Date>,
    born_year: Option<u64>,
    cached_image: Image,
    canonical_id: Option<u64>,
    contributions: Vec<Contribution>,
    creator: Option<User>,
    death_date:	Option<Date>,
    death_year: Option<u64>,
    gender_id: Option<Gender>,
    id: u64,
    // todo!
    identifiers: Value,
    image: Option<Image>,
    image_id: Option<u64>,
    is_bipoc: Option<bool>,
    is_lgbtq: Option<bool>,
    // todo!
    links: Value,
    location: Option<String>,
    locked:	bool,
    name: String,
    name_personal: Option<String>,
    object_type: String,
    slug: Option<String>,
    state: RecordState,
    title: Option<String>,
    user_id: Option<u64>,
    users_count: u64,
}