use crate::base_hardcover_item::BaseHardcoverItem;
use crate::date_parsing;
use crate::HardcoverClient;
use serde::Deserialize;
use serde_json::Value;
use time::PlainDateTime;

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

#[derive(Debug, Clone, Deserialize)]
pub struct BookSeries {
    pub book_id: u64,
    pub compilation: bool,
    #[serde(deserialize_with = "date_parsing::plain_datetime")]
    pub created_at: PlainDateTime,
    pub details: String,
    pub featured: bool,
    pub id: u64,
    pub position: f64,
    pub series_id: u64,
    #[serde(deserialize_with = "date_parsing::plain_datetime")]
    pub updated_at: PlainDateTime,
}

impl BaseHardcoverItem for BookSeries {
    async fn from_id(id: u64, client: &HardcoverClient) -> Result<Self, reqwest::Error> {
        let query = r#"
        query GetBookSeries($id: Int!) {
          book_series_by_pk(id: $id) {"#
            .to_string()
            + QUERY_FIELDS
            + r#"
          }
        }
        "#;

        let variables = serde_json::json!({ "id": id });
        let data = Self::from_data(query, variables, client).await?;

        Ok(Self::from_value(data["book_series_by_pk"].clone()))
    }

    fn from_value(data: Value) -> Self {
        serde_json::from_value(data).unwrap()
    }
}
