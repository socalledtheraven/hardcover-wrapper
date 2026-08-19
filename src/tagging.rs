use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct Tagging {
    count: u64,
    id: u64,
    slug: String,
    tag: String,
    // todo!
    tag_category: Value,
    tag_category_id: u64,
    // todo!
    taggings: Vec<Value>,
}