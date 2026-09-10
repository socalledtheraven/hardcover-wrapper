use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Clone, Deserialize)]
pub struct Platform {
    pub id: u64,
    pub name: String,
    pub url: Option<String>,
}

impl Platform {
    pub fn new(resp: Value) -> Self {
        serde_json::from_value(resp).unwrap()
    }
}
