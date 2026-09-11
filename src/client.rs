use reqwest::header::{HeaderMap, AUTHORIZATION, CONTENT_TYPE, USER_AGENT};
use serde_json::Value;

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
        variables: Value,
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
