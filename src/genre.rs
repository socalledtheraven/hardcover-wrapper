use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Genre {
    pub count: u64,
    pub tag: String,
    pub tag_slug: String,
}
