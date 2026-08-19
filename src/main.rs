use std::collections::HashMap;
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE, USER_AGENT, HeaderMap};

use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;

use serde_json;
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: i32,
    username: String,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let query = r#"
        query GetCurrentUser {
          me {
            id
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

    Ok(request)
}

fn parse_resp<T: DeserializeOwned>(req: Value) -> T {
    let x = &*serde_json::to_string(
        &req["data"]["me"][0]
    ).expect("failed to return Value to string");

    serde_json::from_str(x).expect("failed to parse to type")
}