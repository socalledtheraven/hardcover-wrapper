use crate::date_parsing;
use serde::Deserialize;
use crate::base_hardcover_item::BaseHardcoverItem;
use crate::HardcoverClient;
use serde_json::Value;
use time::{Date, OffsetDateTime};

const QUERY_FIELDS: &str = r#"
archived
completed_at
conditions
description
end_date
goal
id
metric
privacy_setting_id
progress
start_date
state
user_id
"#;

#[derive(Debug, Clone, Deserialize)]
pub struct GoalConditions {
    pub goal: u64,
    pub r#type: Option<String>,
    pub metric: GoalMetric,
    #[serde(rename = "endDate", deserialize_with = "date_parsing::date")]
    pub end_date: Date,
    #[serde(rename = "startDate", deserialize_with = "date_parsing::date")]
    pub start_date: Date,
    #[serde(rename = "readingFormatId")]
    pub reading_format_id: Option<u64>,
    #[serde(rename = "specificEndDate")]
    pub specific_end_date: Option<bool>,
    #[serde(rename = "specificStartDate")]
    pub specific_start_date: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GoalMetric {
    Page,
    Book,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Goal {
    pub archived: bool,
    #[serde(deserialize_with = "date_parsing::offset_datetime_optional")]
    pub completed_at: Option<OffsetDateTime>,
    pub conditions: Option<GoalConditions>,
    pub description: Option<String>,
    #[serde(deserialize_with = "date_parsing::date")]
    pub end_date: Date,
    pub goal: u64,
    pub id: u64,
    pub metric: String,
    pub privacy_setting_id: Option<u64>,
    pub progress: f64,
    #[serde(deserialize_with = "date_parsing::date")]
    pub start_date: Date,
    pub state: String,
    pub user_id: u64,
}

impl BaseHardcoverItem for Goal {
    async fn from_id(id: u64, client: &HardcoverClient) -> Result<Self, reqwest::Error> {
        let query = r#"
        query GetGoal($id: Int!) {
          goals_by_pk(id: $id) {"#
            .to_string()
            + QUERY_FIELDS
            + r#"
          }
        }
        "#;

        let data = Self::from_data(query, id, client).await?;

        Ok(Self::new(data["goals_by_pk"].clone()))
    }

    fn new(data: Value) -> Self {
        serde_json::from_value(data).unwrap()
    }
}
