//! Common trait for fetching and deserializing Hardcover items.

use crate::HardcoverClient;
use serde_json::Value;

/// Trait implemented by Hardcover data models for GraphQL fetching and JSON conversion.
///
/// Types implementing `BaseHardcoverItem` can query their corresponding entity from the API by ID
/// and construct instances from raw JSON values.
pub trait BaseHardcoverItem: Sized {
    /// Executes a GraphQL query against the API with the given parameters and extracts the `data` field.
    ///
    /// # Arguments
    ///
    /// * `query` - The GraphQL query text.
    /// * `variables` - Serialized JSON variables for the query.
    /// * `client` - The API client to use for the network request.
    #[allow(async_fn_in_trait)]
    async fn from_data(
        query: String,
        variables: Value,
        client: &HardcoverClient,
    ) -> Result<Value, reqwest::Error> {
        let resp = client.graphql_req(query, variables).await?;

        println!("Resp: {:#?}", resp);

        Ok(resp["data"].clone())
    }

    /// Fetches an entity by its unique numeric ID using the Hardcover API.
    ///
    /// # Arguments
    ///
    /// * `id` - The unique primary key ID of the entity.
    /// * `client` - The API client to execute the query.
    #[allow(async_fn_in_trait)]
    async fn from_id(id: u64, client: &HardcoverClient) -> Result<Self, reqwest::Error>;

    /// Deserializes an entity from a `serde_json::Value`.
    ///
    /// # Arguments
    ///
    /// * `data` - A JSON value corresponding to the entity object.
    fn from_value(data: Value) -> Self;
}
