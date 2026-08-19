use std::collections::HashMap;
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE, USER_AGENT, HeaderMap};

use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;

use serde_json;
use serde_json::Value;

use chrono;

#[derive(Debug, Serialize, Deserialize)]
struct User {
    access_level: Option<i32>,
    #[serde(rename = "account_privacy_setting_id")]
    // todo!
    account_privacy_settings_id: i32,
    // todo!
    activities: Vec<Activity>,
    // todo!
    activity_privacy_settings_id: i32,
    admin: bool,
    bio: Option<String>,
    birthdate: Option<chrono::NaiveDate>,
    // todo!
    blocked_users: Vec<BlockedUser>,
    books_count: i32,
    // todo!
    cached_cover: Value,
    // todo!
    cached_genres: Value,
    // todo!
    cached_image: Value,
    // todo!
    collection_imports: Vec<Import>,
    confirmation_sent_at: Option<chrono::NaiveDateTime>,
    confirmed_at: Option<chrono::NaiveDateTime>,
    // todo!
    created_at: Option<chrono::DateTime<chrono::Utc>>,
    current_sign_in_at: Option<chrono::NaiveDateTime>,
    email: Option<String>,
    // todo!
    email_verified: Option<chrono::DateTime<chrono::Utc>>,
    flair: Option<String>,
    // todo!
    followed_by_users: Vec<User>,
    // todo!
    followed_lists: Vec<List>,
    // todo!
    followed_prompts: Vec<Prompt>,
    // todo!
    followed_users: Vec<User>,
    followed_users_count: i32,
    followers_count: i32,
    // todo!
    follows: Vec<Follow>,
    // todo!
    goals: Vec<Goal>,
    id: i32,
    // todo!
    images: Vec<Image>,
    image_id: i32,
    last_activity_at: Option<chrono::NaiveDateTime>,
    last_sign_in_at: Option<chrono::NaiveDateTime>,
    // todo!
    librarian_roles: Option<Value>,
    link: Option<String>,
    // todo!
    links: Option<Vec<Option<Link>>>,
    // todo!
    lists: Vec<List>,
    location: Option<String>,
    locked_at: Option<chrono::NaiveDateTime>,
    membership: Option<String>,
    membership_ends_at: Option<chrono::NaiveDateTime>,
    name: Option<String>,
    // todo!
    notification_deliveries: Vec<NotificationDelivery>,
    object_type: Option<String>,
    onboarded: bool,
    payment_system_id: Option<i32>,
    pro: bool,
    // todo!
    prompt_answers: Vec<PromptAnswer>,
    // todo!
    prompts: Vec<Prompt>,
    pronoun_personal: String,
    pronoun_possessive: String,
    referrer_id: Option<i32>,
    referrer_url: Option<String>,
    // todo!
    #[serde(rename = "referrered_users")]
    referred_users: Vec<UserBook>,
    remember_created_at: Option<chrono::NaiveDateTime>,
    // todo!
    reported_user_flags: Vec<UserFlag>,
    reset_password_sent_at: Option<chrono::NaiveDateTime>,
    sign_in_count: Option<i32>,
    // todo!
    status_id: i32,
    // todo!
    taggings: Vec<Tagging>,
    // todo!
    timezone: Option<String>,
    unconfirmed_email: Option<String>,
    // todo!
    updated_at: chrono::DateTime<chrono::Utc>,
    // todo!
    user_books: Vec<UserBook>,
    // todo!
    user_flags: Vec<UserFlag>,
    username: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Tagging {}

#[derive(Debug, Serialize, Deserialize)]
struct UserFlag {}

#[derive(Debug, Serialize, Deserialize)]
struct UserBook {}

#[derive(Debug, Serialize, Deserialize)]
struct PromptAnswer {}

#[derive(Debug, Serialize, Deserialize)]
struct NotificationDelivery {}

#[derive(Debug, Serialize, Deserialize)]
struct Link {}

#[derive(Debug, Serialize, Deserialize)]
struct Image {}

#[derive(Debug, Serialize, Deserialize)]
struct Goal {}

#[derive(Debug, Serialize, Deserialize)]
struct Follow {}

#[derive(Debug, Serialize, Deserialize)]
struct Prompt {}

#[derive(Debug, Serialize, Deserialize)]
struct List {}

#[derive(Debug, Serialize, Deserialize)]
struct Import {}

#[derive(Debug, Serialize, Deserialize)]
struct BlockedUser {}

#[derive(Debug, Serialize, Deserialize)]
struct Activity {}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let query = r#"
        query GetCurrentUser {
          me {
            access_level
            account_privacy_setting_id
            activity_privacy_settings_id
            admin
            bio
            birthdate
            books_count
            cached_cover
            cached_genres
            cached_image
            confirmation_sent_at
            confirmed_at
            created_at
            current_sign_in_at
            email
            email_verified
            flair
            followed_users_count
            followers_count
            id
            image_id
            last_activity_at
            last_sign_in_at
            librarian_roles
            link
            location
            locked_at
            membership
            membership_ends_at
            name
            object_type
            onboarded
            payment_system_id
            pro
            pronoun_personal
            pronoun_possessive
            referrer_id
            referrer_url
            remember_created_at
            reset_password_sent_at
            sign_in_count
            status_id
            timezone
            unconfirmed_email
            updated_at
            username
          }
        }
    "#;

    let req = graphql_req(query, HashMap::new()).await?;
    let me: User = parse_resp(req);

    println!("{me:#?}");
    Ok(())
}

fn create_headers(api_key: &str) -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        format!("Bearer {}", api_key).parse().unwrap()
    );
    headers.insert(
        USER_AGENT,
        "hardcover-api-wrapper".parse().unwrap()
    );
    headers.insert(
        CONTENT_TYPE,
        "application/json".parse().unwrap()
    );

    headers
}

async fn graphql_req(query: &str, variables: HashMap<String, Value>) -> Result<Value, reqwest::Error> {
    let headers = create_headers(env!("API_KEY"));

    let payload = serde_json::json!({
        "query": query,
        "variables": variables,
    });

    let request: Value = reqwest::Client::new()
        .post("https://api.hardcover.app/v1/graphql")
        .headers(headers)
        .json(&payload)
        .send()
        .await?
        .json()
        .await?;

    // todo! add error handling for internal request errors

    println!("request: {request:#?}");

    Ok(request)
}

fn parse_resp<T: DeserializeOwned>(req: Value) -> T {
    let x = &*serde_json::to_string(
        &req["data"]["me"][0]
    ).expect("failed to return Value to string");

    println!("x: {x}");

    serde_json::from_str(x).expect("failed to parse to type")
}