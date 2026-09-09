use crate::base_hardcover_item::BaseHardcoverItem;
use crate::client::GraphQLResponse;
use crate::HardcoverClient;
use reqwest::Error;
use serde_json::Value;

const QUERY_FIELDS: &str = r#"
    count
    id
    slug
    tag
    tag_category_id
"#;

#[derive(Debug, Clone)]
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
        Tag {
            count: { data.get_u64("count").unwrap() },
            id: { data.get_u64("id").unwrap() },
            slug: { data.get_str("slug").unwrap() },
            tag: { data.get_str("tag").unwrap() },
            tag_category_id: { data.get_u64("tag_category_id").unwrap() },
        }
    }
}
