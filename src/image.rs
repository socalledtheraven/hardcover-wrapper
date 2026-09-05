use serde_json::Value;
use crate::graphql::GraphQLResponse;

#[derive(Debug, Clone)]
pub(crate) struct Image {
    color: Option<String>,
    colors: Option<Value>,
    color_name: Option<String>,
    height: Option<u64>,
    id: u64,
    imageable_id: Option<u64>,
    imageable_type: Option<Value>,
    ratio: Option<f64>,
    url: Option<String>,
    width: Option<u64>,
}

impl Image {
    pub(crate) fn new(resp: Value) -> Self {
        Image {
            color: resp.get_str("color"),
            colors: {
                resp.get("colors").cloned()
            },
            color_name: resp.get_str("color_name"),
            height: resp.get_u64("height"),
            id: resp.get_u64("id").unwrap(),
            imageable_id: {
                resp.get_u64("imageable_id")
            },
            imageable_type: {
                resp.get("imageable_type").cloned()
            },
            ratio: {
                resp.get_f64("ratio")
            },
            url: resp.get_str("url"),
            width: resp.get_u64("width"),
        }
    }
}