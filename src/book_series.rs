use serde_json::Value;
use time::PlainDateTime;
use crate::base_hardcover_item::BaseHardcoverItem;
use crate::graphql::{GraphQLResponse};

const QUERY_FIELDS: &str = r#"
book_id
compilation
created_at
details
featured
id
position
series_id
updated_at
"#;

pub(crate) struct BookSeries {
    book_id: u64,
    compilation: bool,
    created_at: PlainDateTime,
    details: String,
    featured: bool,
    id: u64,
    position: f64,
    series_id: u64,
    updated_at: PlainDateTime
}

impl BaseHardcoverItem for BookSeries {
    async fn from_id(id: u64) -> Result<Self, reqwest::Error> {
        let query = r#"
        query GetBookSeries($id: Int!) {
          book_series_by_pk(id: $id) {"#.to_string() + QUERY_FIELDS + r#"
          }
        }
        "#;

        let data = Self::from_data(query, id).await?;

        Ok(Self::new(data["book_series_by_pk"].clone()))
    }

    fn new(data: Value) -> Self {
        BookSeries {
            book_id: {
                data.get_u64("book_id").unwrap()
            },
            compilation: {
                data.get_bool("compilation").unwrap()
            },
            created_at: {
                data.get_plaindt("created_at").unwrap()
            },
            details: {
                data.get_str("details").unwrap()
            },
            featured: {
                data.get_bool("featured").unwrap()
            },
            id: {
                data.get_u64("id").unwrap()
            },
            position: {
                data["position"].as_f64().unwrap()
            },
            series_id: {
                data.get_u64("series_id").unwrap()
            },
            updated_at: {
                data.get_plaindt("updated_at").unwrap()
            },
        }
    }
}