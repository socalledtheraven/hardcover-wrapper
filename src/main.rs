pub mod graphql;
pub mod user;
pub mod activity;
pub mod author;

use serde::{Deserialize, Serialize};

use serde_json;
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, Clone)]
enum PrivacySetting {
    Public,
    FollowersOnly,
    Private,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
enum AccountStatus {
    Created,
    Activated,
    Banned,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
enum MaybeInit<T> {
    Uninitialised,
    Initialised(T),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Genre {
    count: u64,
    tag: String,
    tag_slug: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Tagging {}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct UserFlag {}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct UserBook {}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct PromptAnswer {}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct NotificationDelivery {}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Link {}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Image {
    color: String,
    color_name: String,
    height: u64,
    id: u64,
    url: String,
    width: u64,
}

impl Image {
    fn new(resp: Value) -> Self {
        Image {
            color: resp["color"].as_str().unwrap().to_string(),
            color_name: resp["color_name"].as_str().unwrap().to_string(),
            height: resp["height"].as_u64().unwrap(),
            id: resp["id"].as_u64().unwrap(),
            url: resp["url"].as_str().unwrap().to_string(),
            width: resp["width"].as_u64().unwrap(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Goal {}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Follow {}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Prompt {}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct List {}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Import {}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct BlockedUser {}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Book {}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Like {}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let me = user::User::from_username("prophecyreviews").await?;

    println!("{me:#?}");

    me.activities().await?;

    Ok(())
}
