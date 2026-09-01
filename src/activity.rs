use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use time::OffsetDateTime;
use crate::enums::PrivacySetting;
use crate::goal::Goal;
use crate::graphql::{get_offsetdatetime_from_resp, graphql_req};
use crate::list::List;
use crate::prompt::Prompt;
use crate::user_book::UserBook;

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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct Activity {
    pub(crate) book_id: Option<u64>,
    pub(crate) created_at: Option<OffsetDateTime>,
    pub(crate) data: Value,
    pub(crate) event: ActivityType,
    pub(crate) id: u64,
    pub(crate) likes_count: u64,
    pub(crate) object_type: String,
    pub(crate) privacy_setting_id: PrivacySetting,
    pub(crate) uid: String,
    pub(crate) user_id: u64,
}

impl Activity {
    pub(crate) async fn of_user(user_id: u64) -> Result<Vec<Self>, reqwest::Error> {
        let query = r#"
        query GetActivitiesOfUser($user: Int!) {
          activities(
              order_by: {created_at: desc}
              where: {user_id: {_eq: $user}}
          ) {"#.to_string() + QUERY_FIELDS + r#"
          }
        }
        "#;

        let mut vars = HashMap::new();
        vars.insert("user", user_id.to_string());

        let resp = graphql_req(query, vars).await?;

        println!("Activity response: {resp:#?}");

        let data = &resp["data"]["activities"];

        println!("Activity data: {data:#?}");

        let activities: Vec<Activity> = data
            .as_array()
            .unwrap()
            .iter()
            .map(|activity_data| {
                Activity::from_graphql(activity_data, user_id)
            })
            .collect();
        
        Ok(activities)
    }
    
    fn from_graphql(data: &Value, user_id: u64) -> Self {
        Activity {
            book_id: data["book_id"].as_u64(),
            created_at: {
                get_offsetdatetime_from_resp(data, "created_at")
            },
            event: {
                match data["event"].as_str() {
                    Some("UserBookActivity") => ActivityType::UserBookActivity,
                    Some("GoalActivity") => ActivityType::GoalActivity,
                    Some("PromptActivity") => ActivityType::PromptActivity,
                    Some("ListActivity") => ActivityType::ListActivity,
                    _ => panic!("Unknown activity type"),
                }
            },
            id: data["id"].as_u64().expect("REASON"),
            likes_count: data["likes_count"].as_u64().expect("REASON"),
            object_type: data["object_type"].to_string(),
            data: {
                // todo!
                data["data"].clone()
            },
            privacy_setting_id: {
                match data["privacy_setting_id"].as_u64() {
                    Some(1) => PrivacySetting::Public,
                    Some(2) => PrivacySetting::FollowersOnly,
                    Some(3) => PrivacySetting::Private,
                    _ => panic!("Unknown privacy setting"),
                }
            },
            uid: {
                data["uid"].as_str().expect("REASON").to_string()
            },
            user_id,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) enum ActivityType {
    UserBookActivity,
    GoalActivity,
    PromptActivity,
    ListActivity
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) enum ActivityData {
    UserBookActivityData(UserBook),
    GoalActivityData(Goal),
    PromptActivityData(Prompt),
    ListActivityData(List),
}