use serde_json::Value;
use time::{Date, OffsetDateTime};
use crate::base_hardcover_item::BaseHardcoverItem;
use crate::graphql::{GraphQLResponse};

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

#[derive(Debug, Clone)]
pub(crate) struct GoalConditions {
    goal: u64,
    r#type: Option<String>,
    metric: GoalMetric,
    end_date: Date,
    start_date: Date,
    reading_format_id: Option<u64>,
    specific_end_date: Option<bool>,
    specific_start_date: Option<bool>
}

#[derive(Debug, Clone)]
pub(crate) enum GoalMetric {
    Page,
    Book
}


#[derive(Debug, Clone)]
pub(crate) struct Goal {
    archived: bool,
    completed_at: Option<OffsetDateTime>,
    conditions: Option<GoalConditions>,
    description: Option<String>,
    end_date: Date,
    goal: u64,
    id: u64,
    metric: String,
    privacy_setting_id: Option<u64>,
    progress: f64,
    start_date: Date,
    state: String,
    user_id: u64,
}

impl BaseHardcoverItem for Goal {
    async fn from_id(id: u64) -> Result<Self, reqwest::Error> {
        let query = r#"
        query GetGoal($id: Int!) {
          goals_by_pk(id: $id) {"#.to_string() + QUERY_FIELDS + r#"
          }
        }
        "#;

        let data = Self::from_data(query, id).await?;

        Ok(Self::new(data["goals_by_pk"].clone()))
    }


    fn new(data: Value) -> Self {
        Goal {
            archived: {
                data.get_bool("archived").unwrap()
            },
            completed_at: {
                data.get_offsetdt("completed_at")
            },
            conditions: {
                let data = data.get("conditions");
                if data.is_none() {
                    None
                } else {
                    let data = data.unwrap();
                    Some(GoalConditions {
                        goal: {
                            data.get_u64("goal").unwrap()
                        },
                        r#type: {
                            data.get_str("type")
                        },
                        metric: {
                            match data["metric"].as_str().unwrap() {
                                "page" => GoalMetric::Page,
                                "book" => GoalMetric::Book,
                                _ => panic!("Unknown metric type")
                            }
                        },
                        end_date: {
                            data.get_date("endDate").unwrap()
                        },
                        start_date: {
                            data.get_date("startDate").unwrap()
                        },
                        reading_format_id: {
                            data.get_u64("readingFormatId")
                        },
                        specific_end_date: {
                            data.get_bool("specificEndDate")
                        },
                        specific_start_date: {
                            data.get_bool("specificStartDate")
                        },
                    })
                }
            },
            description: {
                data.get_str("description")
            },
            end_date: {
                data.get_date("end_date").unwrap()
            },
            goal: {
                data.get_u64("goal").unwrap()
            },
            id: {
                data.get_u64("id").unwrap()
            },
            metric: {
                data.get_str("metric").unwrap()
            },
            privacy_setting_id: {
                data.get_u64("privacy_setting_id")
            },
            progress: {
                data.get_f64("progress").unwrap()
            },
            start_date: {
                data.get_date("start_date").unwrap()
            },
            state: {
                data.get_str("state").unwrap()
            },
            user_id: {
                data.get_u64("user_id").unwrap()
            }
        }
    }
}