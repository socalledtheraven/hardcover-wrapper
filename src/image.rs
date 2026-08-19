use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct Image {
    color: Option<String>,
    // todo!
    colors: Option<Value>,
    color_name: Option<String>,
    height: Option<u64>,
    id: u64,
    imageable_id: Option<u64>,
    // todo!
    imageable_type: Option<Value>,
    ratio: Option<f64>,
    url: Option<String>,
    width: Option<u64>,
}

impl Image {
    pub(crate) fn new(resp: Value) -> Self {
        Image {
            color: resp["color"].as_str().map(|s| s.to_string()),
            colors: None,
            color_name: resp["color_name"].as_str().map(|s| s.to_string()),
            height: resp["height"].as_u64(),
            id: resp["id"].as_u64().expect("REASON"),
            imageable_id: None,
            imageable_type: None,
            ratio: None,
            url: resp["url"].as_str().map(|s| s.to_string()),
            width: resp["width"].as_u64(),
        }
    }
}