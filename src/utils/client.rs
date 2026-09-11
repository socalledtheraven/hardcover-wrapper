//! HTTP client for sending requests to the Hardcover GraphQL API.

use reqwest::header::{HeaderMap, AUTHORIZATION, CONTENT_TYPE, USER_AGENT};
use serde_json::Value;

/// An asynchronous client for interacting with the Hardcover GraphQL API.
///
/// `HardcoverClient` manages authentication and HTTP communication with the Hardcover API endpoint.
///
/// # Example
///
/// ```no_run
/// use hardcover_rs::HardcoverClient;
///
/// let client = HardcoverClient::new("your_api_key_here");
/// ```
pub struct HardcoverClient {
    http: reqwest::Client,
    api_key: String,
}

impl HardcoverClient {
    /// Creates a new `HardcoverClient` instance configured with the specified API key.
    ///
    /// # Arguments
    ///
    /// * `api_key` - The Hardcover API bearer token (e.g. from developer account settings).
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

    /// Sends a GraphQL query or mutation with variables to the Hardcover API.
    ///
    /// # Arguments
    ///
    /// * `query` - The GraphQL query string.
    /// * `variables` - A `serde_json::Value` containing variable bindings for the query.
    ///
    /// # Returns
    ///
    /// The parsed JSON response as a `serde_json::Value` on success, or a `reqwest::Error` on failure.
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
