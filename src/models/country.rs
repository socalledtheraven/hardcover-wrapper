//! Country model representing geographic regions and ISO codes.

use crate::utils::base_hardcover_item::BaseHardcoverItem;
use crate::utils::date_parsing;
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

/// Represents a country record with geographic metadata.
#[derive(Debug, Clone, Deserialize)]
pub struct Country {
    /// Two-letter country code (ISO 3166-1 alpha-2).
    pub code2: Option<String>,
    /// Three-letter country code (ISO 3166-1 alpha-3).
    pub code3: Option<String>,
    /// Creation timestamp.
    #[serde(deserialize_with = "date_parsing::plain_datetime")]
    pub created_at: PlainDateTime,
    /// Unique identifier for the country.
    pub id: u64,
    /// Intermediate geographic region name.
    pub intermediate_region: Option<String>,
    /// Intermediate geographic region code.
    pub intermediate_region_code: Option<String>,
    /// Full ISO 3166 standard code.
    pub iso_3166: Option<String>,
    /// Country name.
    pub name: Option<String>,
    /// International telephone calling code.
    pub phone_code: Option<String>,
    /// Macro geographic region (e.g. Europe, Asia, Americas).
    pub region: Option<String>,
    /// Macro geographic region code.
    pub region_code: Option<String>,
    /// Sub-regional geographic division name.
    pub sub_region: Option<String>,
    /// Sub-regional geographic division code.
    pub sub_region_code: Option<String>,
    /// Last update timestamp.
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

        let variables = serde_json::json!({ "id": id });
        let data = Self::from_data(query, variables, client).await?;

        Ok(Self::from_value(data["countries_by_pk"].clone()))
    }

    fn from_value(data: Value) -> Self {
        serde_json::from_value(data).unwrap()
    }
}
