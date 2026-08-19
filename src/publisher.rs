use serde::{Deserialize, Serialize};
use time::PlainDateTime;
use crate::edition::Edition;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct Publisher {
    canonical_id: Option<u64>,
    created_at: PlainDateTime,
    editions: Vec<Edition>,
    editions_count: u64,
    id: u64,
    locked: bool,
    name: Option<String>,
    object_type: String,
    parent_id: Option<u64>,
    parent_publisher: Option<Publisher>,
    slug: String,
    state: String,
    updated_at: PlainDateTime,
    user_id: Option<u64>,
}