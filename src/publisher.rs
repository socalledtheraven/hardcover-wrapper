use crate::base_hardcover_item::BaseHardcoverItem;
use crate::client::GraphQLResponse;
use crate::HardcoverClient;
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

#[derive(Debug, Clone)]
pub struct Publisher {
    pub canonical_id: Option<u64>,
    pub created_at: PlainDateTime,
    pub editions_count: u64,
    pub id: u64,
    pub locked: bool,
    pub name: Option<String>,
    pub object_type: String,
    pub parent_id: Option<u64>,
    pub slug: String,
    pub state: String,
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

        Ok(Self::new(data["publishers_by_pk"].clone()))
    }

    fn new(data: Value) -> Self {
        Publisher {
            canonical_id: { data.get_u64("canonical_id") },
            created_at: { data.get_plaindt("created_at").unwrap() },
            editions_count: { data.get_u64("editions_count").unwrap() },
            id: { data.get_u64("id").unwrap() },
            locked: { data.get_bool("locked").unwrap() },
            name: { data.get_str("name") },
            object_type: { data.get_str("object_type").unwrap() },
            parent_id: { data.get_u64("parent_id") },
            slug: { data.get_str("slug").unwrap() },
            state: { data.get_str("state").unwrap() },
            updated_at: { data.get_plaindt("updated_at").unwrap() },
            user_id: { data.get_u64("user_id") },
        }
    }
}
