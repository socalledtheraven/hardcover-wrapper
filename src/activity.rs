use crate::base_hardcover_item::BaseHardcoverItem;
use crate::date_parsing;
use crate::util::PrivacySetting;
use crate::HardcoverClient;
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;
use time::OffsetDateTime;

const QUERY_FIELDS: &str = r#"
book_id
created_at
data
event
id
likes_count
object_type
privacy_setting_id
uid
user_id"#;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum ActivityType {
    UserBookActivity,
    GoalActivity,
    PromptActivity,
    ListActivity,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Activity {
    pub book_id: Option<u64>,

    #[serde(deserialize_with = "date_parsing::offset_datetime_optional")]
    pub created_at: Option<OffsetDateTime>,
    pub data: Value,
    pub event: ActivityType,
    pub id: u64,
    pub likes_count: u64,
    pub object_type: String,
    pub privacy_setting_id: PrivacySetting,
    pub uid: String,
    pub user_id: u64,
}

impl BaseHardcoverItem for Activity {
    async fn from_id(id: u64, client: &HardcoverClient) -> Result<Self, reqwest::Error> {
        let query = r#"
        query GetActivity($id: Int!) {
          activities_by_pk(id: $id) {"#
            .to_string()
            + QUERY_FIELDS
            + r#"
          }
        }
        "#;

        let data = Self::from_data(query, id, client).await?;

        Ok(Self::from_value(data["activities_by_pk"].clone()))
    }

    fn from_value(data: Value) -> Self {
        serde_json::from_value(data).unwrap()
    }
}

impl Activity {
    pub async fn activities_from_user(
        user_id: u64,
        client: &HardcoverClient,
    ) -> Result<Vec<Self>, reqwest::Error> {
        let query = r#"
        query GetActivitiesOfUser($id: Int!) {
          activities(
              order_by: {created_at: desc}
              where: {user_id: {_eq: $id}}
          ) {"#
            .to_string()
            + QUERY_FIELDS
            + r#"
          }
        }
        "#;

        let mut vars = HashMap::new();
        vars.insert("id", user_id.to_string());

        let resp = client.graphql_req(query, vars).await?;

        println!("Activity response: {resp:#?}");

        let data = &resp["data"]["activities"];

        println!("Activity data: {data:#?}");

        let activities: Vec<Activity> = data
            .as_array()
            .unwrap()
            .iter()
            .map(|activity_data| Activity::from_value(activity_data.clone()))
            .collect();

        Ok(activities)
    }
}
