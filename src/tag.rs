use crate::base_hardcover_item::BaseHardcoverItem;
use crate::HardcoverClient;
use reqwest::Error;
use serde::Deserialize;
use serde_json::Value;

const QUERY_FIELDS: &str = r#"
    count
    id
    slug
    tag
    tag_category_id
"#;

#[derive(Debug, Clone, Deserialize)]
pub struct Tag {
    pub count: u64,
    pub id: u64,
    pub slug: String,
    pub tag: String,
    pub tag_category_id: u64,
}

impl BaseHardcoverItem for Tag {
    async fn from_id(id: u64, client: &HardcoverClient) -> Result<Self, Error> {
        let query = r#"
        query GetTag($id: bigint!) {
          tags_by_pk(id: $id) {"#
            .to_string()
            + QUERY_FIELDS
            + r#"
          }
        }
        "#;

        let data = Self::from_data(query, id, client).await?;

        Ok(Self::new(data["tags_by_pk"].clone()))
    }

    fn new(data: Value) -> Self {
        serde_json::from_value(data).unwrap()
    }
}
