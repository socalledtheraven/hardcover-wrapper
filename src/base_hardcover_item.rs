use std::collections::HashMap;
use serde_json::Value;
use crate::graphql::graphql_req;

pub(crate) trait BaseHardcoverItem: Sized {
    async fn from_id(id: u64) -> Result<Self, reqwest::Error>;

    async fn from_data<T: ToString>(query: String, user_data: T) -> Result<Value, reqwest::Error> {
        let mut vars = HashMap::new();
        vars.insert("id", user_data.to_string());

        let resp = graphql_req(query, vars).await?;

        println!("Resp: {:#?}", resp);

        Ok(resp["data"].clone())
    }

    fn new(data: Value) -> Self;
}