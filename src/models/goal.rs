//! Reading goal model representing user targets (e.g. books or pages to read in a year).

use crate::utils::base_hardcover_item::BaseHardcoverItem;
use crate::utils::date_parsing;
use crate::HardcoverClient;
use serde::Deserialize;
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

/// Target conditions and metrics for a reading goal.
#[derive(Debug, Clone, Deserialize)]
pub struct GoalConditions {
    /// Target numeric goal count.
    pub goal: u64,
    /// Goal type descriptor.
    pub r#type: Option<String>,
    /// Unit metric (pages or books).
    pub metric: GoalMetric,
    /// End date of the goal window.
    #[serde(rename = "endDate", deserialize_with = "date_parsing::date")]
    pub end_date: Date,
    /// Start date of the goal window.
    #[serde(rename = "startDate", deserialize_with = "date_parsing::date")]
    pub start_date: Date,
    /// Optional reading format restriction ID.
    #[serde(rename = "readingFormatId")]
    pub reading_format_id: Option<u64>,
    /// Whether a specific end date is enforced.
    #[serde(rename = "specificEndDate")]
    pub specific_end_date: Option<bool>,
    /// Whether a specific start date is enforced.
    #[serde(rename = "specificStartDate")]
    pub specific_start_date: Option<bool>,
}

/// Unit metric for tracking reading goals.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GoalMetric {
    /// Measured in number of pages.
    Page,
    /// Measured in number of completed books.
    Book,
}

/// Represents a reading goal set by a user.
#[derive(Debug, Clone, Deserialize)]
pub struct Goal {
    /// Whether this goal has been archived.
    pub archived: bool,
    /// Timestamp when this goal was completed, if finished.
    #[serde(deserialize_with = "date_parsing::offset_datetime_optional")]
    pub completed_at: Option<OffsetDateTime>,
    /// Specific conditions and constraints for the goal.
    pub conditions: Option<GoalConditions>,
    /// User description or notes for the goal.
    pub description: Option<String>,
    /// End date of the goal period.
    #[serde(deserialize_with = "date_parsing::date")]
    pub end_date: Date,
    /// Target goal quantity.
    pub goal: u64,
    /// Unique identifier for the goal.
    pub id: u64,
    /// Metric description string.
    pub metric: String,
    /// Privacy setting ID.
    pub privacy_setting_id: Option<u64>,
    /// Current completion progress (e.g. 0.0 to 1.0 or count).
    pub progress: f64,
    /// Start date of the goal period.
    #[serde(deserialize_with = "date_parsing::date")]
    pub start_date: Date,
    /// Goal status state string.
    pub state: String,
    /// User ID of the goal owner.
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

        let variables = serde_json::json!({ "id": id });
        let data = Self::from_data(query, variables, client).await?;

        Ok(Self::from_value(data["goals_by_pk"].clone()))
    }

    fn from_value(data: Value) -> Self {
        serde_json::from_value(data).unwrap()
    }
}
