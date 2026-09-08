use serde_json::Value;
use crate::client::GraphQLResponse;

pub struct Platform {
    pub id: u64,
    pub name: String,
    pub url: Option<String>,
}

impl Platform {
    pub fn new(resp: Value) -> Self {
        Platform {
            id: resp.get_u64("id").unwrap(),
            name: resp.get_str("name").unwrap(),
            url: resp.get_str("url"),
        }
    }
}