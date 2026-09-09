use crate::base_hardcover_item::BaseHardcoverItem;
use crate::client::GraphQLResponse;
use crate::HardcoverClient;
use serde_json::Value;
use time::PlainDateTime;

const QUERY_FIELDS: &str = r#"
code2
code3
created_at
id
intermediate_region
intermediate_region_code
iso_3166
name
phone_code
region
region_code
sub_region
sub_region_code
updated_at
"#;

#[derive(Debug, Clone)]
pub struct Country {
    pub code2: Option<String>,
    pub code3: Option<String>,
    pub created_at: PlainDateTime,
    pub id: u64,
    pub intermediate_region: Option<String>,
    pub intermediate_region_code: Option<String>,
    pub iso_3166: Option<String>,
    pub name: Option<String>,
    pub phone_code: Option<String>,
    pub region: Option<String>,
    pub region_code: Option<String>,
    pub sub_region: Option<String>,
    pub sub_region_code: Option<String>,
    pub updated_at: PlainDateTime,
}

impl BaseHardcoverItem for Country {
    async fn from_id(id: u64, client: &HardcoverClient) -> Result<Self, reqwest::Error> {
        let query = r#"
        query GetCountry($id: Int!) {
          countries_by_pk(id: $id) {"#
            .to_string()
            + QUERY_FIELDS
            + r#"
          }
        }
        "#;

        let data = Self::from_data(query, id, client).await?;

        Ok(Self::new(data["countries_by_pk"].clone()))
    }

    fn new(data: Value) -> Self {
        Country {
            code2: { data.get_str("code2") },
            code3: { data.get_str("code3") },
            created_at: { data.get_plaindt("created_at").unwrap() },
            id: { data.get_u64("id").unwrap() },
            intermediate_region: { data.get_str("intermediate_region") },
            intermediate_region_code: { data.get_str("intermediate_region_code") },
            iso_3166: { data.get_str("iso_3166") },
            name: { data.get_str("name") },
            phone_code: { data.get_str("phone_code") },
            region: { data.get_str("region") },
            region_code: { data.get_str("region_code") },
            sub_region: { data.get_str("sub_region") },
            sub_region_code: { data.get_str("sub_region_code") },
            updated_at: { data.get_plaindt("updated_at").unwrap() },
        }
    }
}
