use crate::base_hardcover_item::BaseHardcoverItem;
use crate::date_parsing;
use crate::HardcoverClient;
use serde::Deserialize;
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

#[derive(Debug, Clone, Deserialize)]
pub struct Country {
    pub code2: Option<String>,
    pub code3: Option<String>,
    #[serde(deserialize_with = "date_parsing::plain_datetime")]
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
    #[serde(deserialize_with = "date_parsing::plain_datetime")]
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

        Ok(Self::from_value(data["countries_by_pk"].clone()))
    }

    fn from_value(data: Value) -> Self {
        serde_json::from_value(data).unwrap()
    }
}
