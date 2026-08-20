use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use time::OffsetDateTime;
use crate::book::Book;
use crate::enums::{MaybeInit, PrivacySetting};
use crate::goal::Goal;
use crate::graphql::{get_offsetdatetime_from_resp, graphql_req};
use crate::like::Like;
use crate::list::List;
use crate::prompt::Prompt;
use crate::user::User;
use crate::user_book::UserBook;

const QUERY_FIELDS: &str = r#"
book_id
created_at
data
event
id
likes_count
object_type
original_book_id
privacy_setting_id
uid
user_id"#;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct Activity {
    book: MaybeInit<Option<Book>>,
    book_id: Option<u64>,
    created_at: Option<OffsetDateTime>,
    // todo!
    data: MaybeInit<ActivityData>,
    event: ActivityType,
    followers: MaybeInit<Vec<User>>,
    id: u64,
    likes: MaybeInit<Vec<Like>>,
    likes_count: u64,
    object_type: String,
    original_book_id: Option<u64>,
    // todo!
    privacy_setting: MaybeInit<PrivacySetting2>,
    privacy_setting_id: PrivacySetting,
    uid: String,
    user: MaybeInit<User>,
    user_id: u64,
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
            book: MaybeInit::Uninitialised,
            book_id: data["book_id"].as_u64(),
            created_at: {
                get_offsetdatetime_from_resp(data, "created_at")
            },
            data: {
                MaybeInit::Uninitialised
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
            followers: MaybeInit::Uninitialised,
            id: data["id"].as_u64().expect("REASON"),
            likes: MaybeInit::Uninitialised,
            likes_count: data["likes_count"].as_u64().expect("REASON"),
            object_type: data["object_type"].to_string(),
            original_book_id: {
                data.get("original_book_id").and_then(|v| v.as_u64())
            },
            privacy_setting: MaybeInit::Uninitialised,
            privacy_setting_id: PrivacySetting::Public,
            uid: {
                data["uid"].as_str().expect("REASON").to_string()
            },
            user: MaybeInit::Uninitialised,
            user_id,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
enum ActivityType {
    UserBookActivity,
    GoalActivity,
    PromptActivity,
    ListActivity
}

#[derive(Debug, Serialize, Deserialize, Clone)]
enum ActivityData {
    UserBookActivityData(UserBook),
    GoalActivityData(Goal),
    PromptActivityData(Prompt),
    ListActivityData(List),
}