use crate::base_hardcover_item::BaseHardcoverItem;
use crate::date_parsing;
use crate::HardcoverClient;
use serde::Deserialize;
use serde_json::Value;
use time::PlainDateTime;

const QUERY_FIELDS: &str = r#"
canonical_id
created_at
editions_count
id
locked
name
object_type
parent_id
slug
state
updated_at
user_id
"#;

#[derive(Debug, Clone, Deserialize)]
pub struct Publisher {
    pub canonical_id: Option<u64>,
    #[serde(deserialize_with = "date_parsing::plain_datetime")]
    pub created_at: PlainDateTime,
    pub editions_count: u64,
    pub id: u64,
    pub locked: bool,
    pub name: Option<String>,
    pub object_type: String,
    pub parent_id: Option<u64>,
    pub slug: String,
    pub state: String,
    #[serde(deserialize_with = "date_parsing::plain_datetime")]
    pub updated_at: PlainDateTime,
    pub user_id: Option<u64>,
}

impl BaseHardcoverItem for Publisher {
    async fn from_id(id: u64, client: &HardcoverClient) -> Result<Self, reqwest::Error> {
        let query = r#"
        query GetPublisher($id: bigint!) {
          publishers_by_pk(id: $id) {"#
            .to_string()
            + QUERY_FIELDS
            + r#"
          }
        }
        "#;

        let data = Self::from_data(query, id, client).await?;

        Ok(Self::from_value(data["publishers_by_pk"].clone()))
    }

    fn from_value(data: Value) -> Self {
        serde_json::from_value(data).unwrap()
    }
}
