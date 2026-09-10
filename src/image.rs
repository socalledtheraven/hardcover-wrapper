use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Clone, Deserialize)]
pub struct Image {
    pub color: Option<String>,
    pub colors: Option<Vec<String>>,
    pub color_name: Option<String>,
    pub height: Option<u64>,
    pub id: u64,
    pub imageable_id: Option<u64>,
    pub imageable_type: Option<Imageable>,
    pub ratio: Option<f64>,
    pub url: Option<String>,
    pub width: Option<u64>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum Imageable {
    Author,
    Book,
    Edition,
}

impl Image {
    pub fn new(resp: Value) -> Self {
        serde_json::from_value(resp).unwrap()
    }
}
