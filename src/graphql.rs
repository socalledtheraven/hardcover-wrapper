use std::collections::HashMap;
use reqwest::header::{HeaderMap, AUTHORIZATION, USER_AGENT, CONTENT_TYPE};
use serde_json::Value;
use time::{Date, OffsetDateTime, PlainDateTime};
use time::format_description::well_known::Iso8601;
use crate::PrivacySetting;

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

pub(crate) fn get_u64_from_resp(data: &Value, key: &str) -> Option<u64> {
    data.get(key)
        .and_then(|v| v.as_u64())
}

pub(crate) fn get_str_from_resp(data: &Value, key: &str) -> Option<String> {
    data.get(key)
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
}

pub(crate) fn get_date_from_resp(data: &Value, key: &str) -> Option<Date> {
    data.get(key)
        .and_then(|v| v.as_str())
        .and_then(|s| Date::parse(s, &Iso8601::DATE).ok())
        .and_then(|d| Some(d))
}

pub(crate) fn get_plaindatetime_from_resp(data: &Value, key: &str) -> Option<PlainDateTime> {
    data.get(key)
        .and_then(|v| v.as_str())
        .and_then(|s| PlainDateTime::parse(s, &Iso8601::DATE_TIME).ok())
        .and_then(|d| Some(d))
}

pub(crate) fn get_offsetdatetime_from_resp(data: &Value, key: &str) -> Option<OffsetDateTime> {
    data.get(key)
        .and_then(|v| v.as_str())
        .and_then(|s| OffsetDateTime::parse(s, &Iso8601::DATE_TIME).ok())
        .and_then(|d| Some(d))
}

pub(crate) fn get_bool_from_resp(data: &Value, key: &str) -> bool {
    data[key]
        .as_bool()
        .unwrap()
}

pub(crate) fn get_privacysetting_from_resp(data: &Value, key: &str) -> PrivacySetting {
    // in either case of it being unknown, we default to private,
    // this will never happen, but better to fail closed
    data[key]
        .as_u64()
        .map(|v| match v {
            1 => PrivacySetting::Public,
            2 => PrivacySetting::FollowersOnly,
            3 => PrivacySetting::Private,
            _ => PrivacySetting::Private,
        })
        .unwrap_or(PrivacySetting::Private)
}