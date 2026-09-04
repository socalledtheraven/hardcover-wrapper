use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::graphql::GraphQLResponse;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct Language {
    code2: Option<String>,
    code3: Option<String>,
    id: u64,
    language: String,
}

impl Language {
    pub(crate) fn new(resp: Value) -> Self {
        Language {
            code2: resp.get_str("code2"),
            code3: resp.get_str("code3"),
            id: resp.get_u64("id").unwrap(),
            language: resp.get_str("language").unwrap(),
        }
    }
}