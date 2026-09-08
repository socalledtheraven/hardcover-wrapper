use std::collections::HashMap;
use serde_json::Value;
use crate::client::graphql_req;

pub trait BaseHardcoverItem: Sized {
    #[allow(async_fn_in_trait)]
    async fn from_data<T: ToString>(query: String, user_data: T) -> Result<Value, reqwest::Error> {
        let mut vars = HashMap::new();
        vars.insert("id", user_data.to_string());

        let resp = graphql_req(query, vars).await?;

        println!("Resp: {:#?}", resp);

        Ok(resp["data"].clone())
    }

    #[allow(async_fn_in_trait)]
    async fn from_id(id: u64) -> Result<Self, reqwest::Error>;

    fn new(data: Value) -> Self;
}