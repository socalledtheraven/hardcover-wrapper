use crate::book::Rating;
use crate::util::{Identifiers, Link, PrivacySetting, ReadingStatus};
use reqwest::header::{HeaderMap, AUTHORIZATION, CONTENT_TYPE, USER_AGENT};
use serde_json::Value;
use std::collections::HashMap;
use time::format_description::well_known::Iso8601;
use time::{Date, OffsetDateTime, PlainDateTime};

pub struct HardcoverClient {
    http: reqwest::Client,
    api_key: String,
}

impl HardcoverClient {
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            http: reqwest::Client::new(),
            api_key: api_key.into(),
        }
    }

    fn create_headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            format!("Bearer {}", self.api_key).parse().unwrap(),
        );
        headers.insert(USER_AGENT, "hardcover-api-wrapper".parse().unwrap());
        headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());

        headers
    }

    pub async fn graphql_req(
        &self,
        query: String,
        variables: HashMap<&str, String>,
    ) -> Result<Value, reqwest::Error> {
        let headers = Self::create_headers(self);

        let payload = serde_json::json!({
            "query": query,
            "variables": variables,
        });

        let request: Value = self
            .http
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
}

pub trait GraphQLResponse {
    fn get_u64(&self, key: &str) -> Option<u64>;
    fn get_f64(&self, key: &str) -> Option<f64>;
    fn get_str(&self, key: &str) -> Option<String>;
    fn get_date(&self, key: &str) -> Option<Date>;
    fn get_plaindt(&self, key: &str) -> Option<PlainDateTime>;
    fn get_offsetdt(&self, key: &str) -> Option<OffsetDateTime>;
    fn get_bool(&self, key: &str) -> Option<bool>;
    fn get_privacysetting(&self, key: &str) -> PrivacySetting;
    fn get_readingstatus(&self, key: &str) -> ReadingStatus;
    fn get_str_vec(&self, key: &str) -> Vec<String>;
    fn get_opt_str_vec(&self, key: &str) -> Option<Vec<String>>;
    fn get_u64_vec(&self, key: &str) -> Vec<u64>;
    fn get_link_vec(&self, key: &str) -> Vec<Link>;
    fn get_rating_vec(&self, key: &str) -> Vec<Rating>;
    fn get_identifiers(&self, key: &str) -> Identifiers;
}

impl GraphQLResponse for Value {
    fn get_u64(&self, key: &str) -> Option<u64> {
        self.get(key).and_then(|v| v.as_u64())
    }

    fn get_f64(&self, key: &str) -> Option<f64> {
        self.get(key).and_then(|v| v.as_f64())
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

    fn get_plaindt(&self, key: &str) -> Option<PlainDateTime> {
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

    fn get_bool(&self, key: &str) -> Option<bool> {
        self[key].as_bool()
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

    fn get_readingstatus(&self, key: &str) -> ReadingStatus {
        self[key]
            .as_u64()
            .map(|v| match v {
                1 => ReadingStatus::WantToRead,
                2 => ReadingStatus::CurrentlyReading,
                3 => ReadingStatus::Read,
                4 => ReadingStatus::Paused,
                5 => ReadingStatus::DidNotFinish,
                6 => ReadingStatus::Ignored,
                _ => panic!("Unknown reading status"),
            })
            .expect("Reading status is missing")
    }

    fn get_str_vec(&self, key: &str) -> Vec<String> {
        self.get_opt_str_vec(key).expect("String vector is missing")
    }

    fn get_opt_str_vec(&self, key: &str) -> Option<Vec<String>> {
        self.get(key).and_then(|v| v.as_array()).map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                .collect()
        })
    }

    fn get_u64_vec(&self, key: &str) -> Vec<u64> {
        self.get(key)
            .and_then(|v| v.as_array())
            .map(|arr| arr.iter().filter_map(|v| v.as_u64()).collect())
            .expect("u64 vector is missing")
    }

    fn get_link_vec(&self, key: &str) -> Vec<Link> {
        self.get(key)
            .and_then(|v| v.as_array())
            .unwrap_or(&vec![])
            .iter()
            .map(|link| Link {
                url: link.get_str("url").unwrap(),
                title: link.get_str("title").unwrap(),
            })
            .collect()
    }

    fn get_rating_vec(&self, key: &str) -> Vec<Rating> {
        self.get(key)
            .and_then(|v| v.as_array())
            .unwrap_or(&vec![])
            .iter()
            .map(|rating| Rating {
                count: rating.get_u64("count").unwrap(),
                rating: rating.get_f64("rating").unwrap(),
            })
            .collect()
    }

    fn get_identifiers(&self, key: &str) -> Identifiers {
        let data = &self[key];

        Identifiers {
            audible: { data.get_opt_str_vec("audible") },
            goodreads: { data.get_opt_str_vec("goodreads") },
            openlibrary: { data.get_opt_str_vec("openlibrary") },
        }
    }
}
