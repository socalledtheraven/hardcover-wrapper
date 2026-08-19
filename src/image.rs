use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct Image {
    color: String,
    color_name: String,
    height: u64,
    id: u64,
    url: String,
    width: u64,
}

impl Image {
    pub(crate) fn new(resp: Value) -> Self {
        Image {
            color: resp["color"].as_str().unwrap().to_string(),
            color_name: resp["color_name"].as_str().unwrap().to_string(),
            height: resp["height"].as_u64().unwrap(),
            id: resp["id"].as_u64().unwrap(),
            url: resp["url"].as_str().unwrap().to_string(),
            width: resp["width"].as_u64().unwrap(),
        }
    }
}