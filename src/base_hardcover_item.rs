use crate::HardcoverClient;
use serde_json::Value;

pub trait BaseHardcoverItem: Sized {
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

    #[allow(async_fn_in_trait)]
    async fn from_id(id: u64, client: &HardcoverClient) -> Result<Self, reqwest::Error>;

    fn from_value(data: Value) -> Self;
}
