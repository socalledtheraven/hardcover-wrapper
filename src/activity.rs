use std::collections::HashMap;
use serde_json::Value;
use time::{OffsetDateTime};
use crate::base_hardcover_item::BaseHardcoverItem;
use crate::enums::{PrivacySetting, ReadingStatus};
use crate::graphql::{graphql_req, GraphQLResponse};

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

#[derive(Debug, Clone)]
pub(crate) enum ActivityType {
    UserBookActivity,
    GoalActivity,
    PromptActivity,
    ListActivity
}

#[derive(Debug, Clone)]
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

impl BaseHardcoverItem for Activity {
    async fn from_id(id: u64) -> Result<Self, reqwest::Error> {
        let query = r#"
        query GetActivity($id: Int!) {
          activities_by_pk(id: $id) {"#.to_string() + QUERY_FIELDS + r#"
          }
        }
        "#;

        let data = Self::from_data(query, id).await?;

        Ok(Self::new(data["activities_by_pk"].clone()))
    }

    fn new(data: Value) -> Self {
        Activity {
            book_id: {
                data.get_u64("book_id")
            },
            created_at: {
                data.get_offsetdt("created_at")
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
            id: {
                data.get_u64("id").unwrap()
            },
            likes_count: {
                data.get_u64("likes_count").unwrap()
            },
            object_type: {
                data.get_str("object_type").unwrap()
            },
            data: {
                data.get("data").unwrap().clone()
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
                data.get_str("uid").unwrap()
            },
            user_id: {
                data.get_u64("user_id").unwrap()
            }
        }
    }
}

impl Activity {
    async fn activities_from_user(user_id: u64) -> Result<Vec<Self>, reqwest::Error> {
        let query = r#"
        query GetActivitiesOfUser($id: Int!) {
          activities(
              order_by: {created_at: desc}
              where: {user_id: {_eq: $id}}
          ) {"#.to_string() + QUERY_FIELDS + r#"
          }
        }
        "#;

        let mut vars = HashMap::new();
        vars.insert("id", user_id.to_string());

        let resp = graphql_req(query, vars).await?;

        println!("Activity response: {resp:#?}");

        let data = &resp["data"]["activities"];

        println!("Activity data: {data:#?}");

        let activities: Vec<Activity> = data
            .as_array()
            .unwrap()
            .iter()
            .map(|activity_data| {
                Activity::new(activity_data.clone())
            })
            .collect();

        Ok(activities)
    }
}
