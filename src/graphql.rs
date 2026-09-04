use std::collections::HashMap;
use reqwest::header::{HeaderMap, AUTHORIZATION, USER_AGENT, CONTENT_TYPE};
use serde_json::Value;
use time::{Date, OffsetDateTime, PlainDateTime};
use time::format_description::well_known::Iso8601;
use crate::enums::PrivacySetting;

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

pub(crate) async fn graphql_req(query: String, variables: HashMap<&str, String>) -> Result<Value, reqwest::Error> {
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

    Ok(request)
}

pub(crate) trait GraphQLResponse {
    fn get_u64(&self, key: &str) -> Option<u64>;
    fn get_f64(&self, key: &str) -> Option<f64>;
    fn get_str(&self, key: &str) -> Option<String>;
    fn get_date(&self, key: &str) -> Option<Date>;
    fn get_plaindatetime(&self, key: &str) -> Option<PlainDateTime>;
    fn get_offsetdt(&self, key: &str) -> Option<OffsetDateTime>;
    fn get_bool(&self, key: &str) -> bool;
    fn get_privacysetting(&self, key: &str) -> PrivacySetting;
    fn get_str_vec(&self, key: &str) -> Vec<String>;
}

impl GraphQLResponse for Value {
    fn get_u64(&self, key: &str) -> Option<u64> {
        self.get(key)
            .and_then(|v| v.as_u64())
    }

    fn get_f64(&self, key: &str) -> Option<f64> {
        self.get(key)
            .and_then(|v| v.as_f64())
    }

    fn get_str(&self, key: &str) -> Option<String> {
        self.get(key)
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
    }

    fn get_date(&self, key: &str) -> Option<Date> {
        self.get(key)
            .and_then(|v| v.as_str())
            .and_then(|s| Date::parse(s, &Iso8601::DATE).ok())
            .and_then(|d| Some(d))
    }

    fn get_plaindatetime(&self, key: &str) -> Option<PlainDateTime> {
        self.get(key)
            .and_then(|v| v.as_str())
            .and_then(|s| PlainDateTime::parse(s, &Iso8601::DATE_TIME).ok())
            .and_then(|d| Some(d))
    }

    fn get_offsetdt(&self, key: &str) -> Option<OffsetDateTime> {
        self.get(key)
            .and_then(|v| v.as_str())
            .and_then(|s| OffsetDateTime::parse(s, &Iso8601::DATE_TIME).ok())
            .and_then(|d| Some(d))
    }

    fn get_bool(&self, key: &str) -> bool {
        self[key]
            .as_bool()
            .unwrap()
    }

    fn get_privacysetting(&self, key: &str) -> PrivacySetting {
        // in either case of it being unknown, we panic
        self[key]
            .as_u64()
            .map(|v| match v {
                1 => PrivacySetting::Public,
                2 => PrivacySetting::FollowersOnly,
                3 => PrivacySetting::Private,
                _ => panic!("Unknown privacy setting"),
            })
            .expect("Privacy setting is missing")
    }

    fn get_str_vec(&self, key: &str) -> Vec<String> {
        self.get(key)
            .and_then(|v| v.as_array())
            .map(|arr| arr
                .iter()
                .filter_map(|v| v
                    .as_str()
                    .map(|s| s.to_string())
                ).collect()
            )
            .expect("String vector is missing")
    }
}