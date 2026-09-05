use serde::{Deserialize, Serialize};
use serde_json::Value;
use time::OffsetDateTime;
use crate::base_hardcover_item::BaseHardcoverItem;
use crate::graphql::{GraphQLResponse};

const QUERY_FIELDS: &str = r#"
created_at
id
likeable_id
likeable_type
user_id
"#;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct Like {
    created_at: Option<OffsetDateTime>,
    id: u64,
    likeable_id: u64,
    likeable_type: String,
    user_id: u64,
}

impl BaseHardcoverItem for Like {
    async fn from_id(id: u64) -> Result<Self, reqwest::Error> {
        let query = r#"
        query GetLike($id: Int!) {
          likes_by_pk(id: $id) {"#.to_string() + QUERY_FIELDS + r#"
          }
        }
        "#;

        let data = Self::from_data(query, id).await?;

        Ok(Self::new(data["likes_by_pk"].clone()))
    }

    fn new(resp: Value) -> Self {
        Like {
            created_at: resp.get_offsetdt("created_at"),
            id: resp.get_u64("id").unwrap(),
            likeable_id: resp.get_u64("likeable_id").unwrap(),
            likeable_type: resp.get_str("likeable_type").unwrap().to_string(),
            user_id: resp.get_u64("user_id").unwrap(),
        }
    }
}