use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct Genre {
    pub(crate) count: u64,
    pub(crate) tag: String,
    pub(crate) tag_slug: String,
}