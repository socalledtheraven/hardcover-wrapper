use crate::date_parsing;
use serde::Deserialize;
use crate::base_hardcover_item::BaseHardcoverItem;
use crate::client::GraphQLResponse;
use crate::HardcoverClient;
use serde_json::Value;
use time::OffsetDateTime;

const QUERY_FIELDS: &str = r#"
created_at
id
likeable_id
likeable_type
user_id
"#;

#[derive(Debug, Clone, Deserialize)]
pub struct Like {
    #[serde(deserialize_with = "date_parsing::offset_datetime_optional")]
    pub created_at: Option<OffsetDateTime>,
    pub id: u64,
    pub likeable_id: u64,
    pub likeable_type: String,
    pub user_id: u64,
}

impl BaseHardcoverItem for Like {
    async fn from_id(id: u64, client: &HardcoverClient) -> Result<Self, reqwest::Error> {
        let query = r#"
        query GetLike($id: Int!) {
          likes_by_pk(id: $id) {"#
            .to_string()
            + QUERY_FIELDS
            + r#"
          }
        }
        "#;

        let data = Self::from_data(query, id, client).await?;

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
