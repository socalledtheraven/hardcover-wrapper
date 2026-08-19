use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use time::OffsetDateTime;
use crate::book::Book;
use crate::enums::{MaybeInit, PrivacySetting};
use crate::graphql::graphql_req;
use crate::like::Like;
use crate::user::User;

const QUERY_FIELDS: &str = r#"
book_id
created_at
data
event
id
likes_count
object_type
original_book_id
privacy_setting
privacy_setting_id
uid
user_id"#;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct Activity {
    book: MaybeInit<Option<Book>>,
    book_id: Option<u64>,
    created_at: Option<OffsetDateTime>,
    // todo!
    data: Value,
    event: ActivityType,
    followers: MaybeInit<Vec<User>>,
    id: u64,
    likes: MaybeInit<Vec<Like>>,
    likes_count: u64,
    object_type: String,
    original_book_id: Option<u64>,
    // todo!
    privacy_setting: String,
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

        let data = &resp["data"]["users"];

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
            created_at: None,
            data: Default::default(),
            event: ActivityType::UserBookActivity,
            followers: MaybeInit::Uninitialised,
            id: 0,
            likes: MaybeInit::Uninitialised,
            likes_count: 0,
            object_type: "".to_string(),
            original_book_id: None,
            privacy_setting: "".to_string(),
            privacy_setting_id: PrivacySetting::Public,
            uid: "".to_string(),
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