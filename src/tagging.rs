use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct Tagging {
    count: u32,
    id: u32,
    slug: String,
    tag: String,
    // todo!
    tag_category: Value,
    tag_category_id: u32,
    // todo!
    taggings: Vec<Value>,
}