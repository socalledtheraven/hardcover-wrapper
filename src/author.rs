use serde::{Deserialize, Serialize};
use crate::image::Image;
use crate::user::User;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct Author {
    alias: Author,
    alias_id: u32,
    alternate_names: Vec<String>,
    bio: String,
    books_count: u32,
    born_date: date,
    born_year: u32,
    cached_image: jsonb,
    canonical: Author,
    canonical_id: u32,
    contributions: Contribution,
    creator: User,
    death_date:	date,
    death_year: u32,
    gender_id: u32,
    id: u32,
    identifiers: jsonb,
    image: Image,
    image_id: u32,
    is_bipoc: bool,
    is_lgbtq: bool,
    links: jsonb,
    location: String,
    locked:	bool,
    name: String,
    name_personal: String,
    object_type: String,
    slug: String,
    state: String,
    title: String,
    user_id: u32,
    users_count: u32,
}