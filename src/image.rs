use serde_json::Value;
use crate::graphql::GraphQLResponse;

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub enum Imageable {
    Author,
    Book,
    Edition,
}

impl Image {
    pub fn new(resp: Value) -> Self {
        Image {
            color: resp.get_str("color"),
            colors: {
                resp.get_opt_str_vec("colors")
            },
            color_name: resp.get_str("color_name"),
            height: resp.get_u64("height"),
            id: resp.get_u64("id").unwrap(),
            imageable_id: {
                resp.get_u64("imageable_id")
            },
            imageable_type: {
                match resp["imageable_type"].as_str() {
                    Some("Author") => Some(Imageable::Author),
                    Some("Book") => Some(Imageable::Book),
                    Some("Edition") => Some(Imageable::Edition),
                    _ => None,
                }
            },
            ratio: {
                resp.get_f64("ratio")
            },
            url: resp.get_str("url"),
            width: resp.get_u64("width"),
        }
    }
}