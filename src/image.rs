use serde_json::Value;
use crate::graphql::GraphQLResponse;

#[derive(Debug, Clone)]
pub(crate) struct Image {
    color: Option<String>,
    colors: Option<Vec<String>>,
    color_name: Option<String>,
    height: Option<u64>,
    id: u64,
    imageable_id: Option<u64>,
    imageable_type: Option<Imageable>,
    ratio: Option<f64>,
    url: Option<String>,
    width: Option<u64>,
}

#[derive(Debug, Clone)]
pub(crate) enum Imageable {
    Author,
    Book,
    Edition,
}

impl Image {
    pub(crate) fn new(resp: Value) -> Self {
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