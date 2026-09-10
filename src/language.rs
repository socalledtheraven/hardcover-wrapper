use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Clone, Deserialize)]
pub struct Language {
    pub code2: Option<String>,
    pub code3: Option<String>,
    pub id: u64,
    pub language: String,
}

impl Language {
    pub fn new(resp: Value) -> Self {
        serde_json::from_value(resp).unwrap()
    }
}
