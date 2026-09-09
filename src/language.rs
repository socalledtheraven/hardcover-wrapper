use crate::client::GraphQLResponse;
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct Language {
    pub code2: Option<String>,
    pub code3: Option<String>,
    pub id: u64,
    pub language: String,
}

impl Language {
    pub fn new(resp: Value) -> Self {
        Language {
            code2: resp.get_str("code2"),
            code3: resp.get_str("code3"),
            id: resp.get_u64("id").unwrap(),
            language: resp.get_str("language").unwrap(),
        }
    }
}
