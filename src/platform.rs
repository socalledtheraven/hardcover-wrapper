use serde_json::Value;
use crate::graphql::GraphQLResponse;

pub(crate) struct Platform {
    id: u64,
    name: String,
    url: Option<String>,
}

impl Platform {
    pub(crate) fn new(resp: Value) -> Self {
        Platform {
            id: resp.get_u64("id").unwrap(),
            name: resp.get_str("name").unwrap(),
            url: resp.get_str("url"),
        }
    }
}