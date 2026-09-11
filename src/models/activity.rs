//! Activity model representing user actions and social events.

use crate::utils::base_hardcover_item::BaseHardcoverItem;
use crate::utils::date_parsing;
use crate::utils::enums::PrivacySetting;
use crate::HardcoverClient;
use serde::Deserialize;
use serde_json::Value;
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

/// Type of activity event triggered by a user.
#[derive(Debug, Clone, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum ActivityType {
    /// Reading log, status change, or review on a book.
    UserBookActivity,
    /// Progress update or achievement on a reading goal.
    GoalActivity,
    /// Answer or response to a prompt.
    PromptActivity,
    /// Creation or update of a book list.
    ListActivity,
}

/// Represents a user activity feed item in Hardcover.
#[derive(Debug, Clone, Deserialize)]
pub struct Activity {
    /// ID of the associated book, if applicable.
    pub book_id: Option<u64>,

    /// Timestamp when this activity occurred.
    #[serde(deserialize_with = "date_parsing::offset_datetime_optional")]
    pub created_at: Option<OffsetDateTime>,
    /// Additional JSON payload data specific to the event type.
    pub data: Value,
    /// Type of activity event.
    pub event: ActivityType,
    /// Unique identifier of the activity.
    pub id: u64,
    /// Number of likes received by this activity.
    pub likes_count: u64,
    /// GraphQL object type name.
    pub object_type: String,
    /// Privacy visibility setting of the activity.
    pub privacy_setting_id: PrivacySetting,
    /// Unique string identifier (UID) for the activity event.
    pub uid: String,
    /// User ID of the actor who produced this activity.
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

        let variables = serde_json::json!({ "id": id });

        let data = Self::from_data(query, variables, client).await?;

        Ok(Self::from_value(data["activities_by_pk"].clone()))
    }

    fn from_value(data: Value) -> Self {
        serde_json::from_value(data).unwrap()
    }
}

impl Activity {
    /// Fetches all activities authored by the specified user ID in descending chronological order.
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
        
        let variables = serde_json::json!({ "id": user_id });

        let resp = client.graphql_req(query, variables).await?;

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
