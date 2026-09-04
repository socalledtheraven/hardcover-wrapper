use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use time::PlainDateTime;
use crate::graphql::{graphql_req, GraphQLResponse};

const QUERY_FIELDS: &str = r#"
code2
code3
created_at
id
intermediate_region
intermediate_region_code
iso_3166
name
phone_code
region
region_code
sub_region
sub_region_code
updated_at
"#;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct Country {
    code2: Option<String>,
    code3: Option<String>,
    created_at: PlainDateTime,
    id: u64,
    intermediate_region: Option<String>,
    intermediate_region_code: Option<String>,
    iso_3166: Option<String>,
    name: Option<String>,
    phone_code: Option<String>,
    region: Option<String>,
    region_code: Option<String>,
    sub_region: Option<String>,
    sub_region_code: Option<String>,
    updated_at: PlainDateTime,
}

impl Country {
    pub(crate) async fn from_country_id(country_id: u64) -> Result<Self, reqwest::Error> {
        let query = r#"
        query GetAuthor($id: Int!) {
          countries(where: {id: {_eq: $id}}, limit: 1) {"#.to_string() + QUERY_FIELDS + r#"
          }
        }
        "#;

        Self::from_data(query, country_id).await
    }

    async fn from_data<T: ToString>(query: String, user_data: T) -> Result<Self, reqwest::Error> {
        let mut vars = HashMap::new();
        vars.insert("id", user_data.to_string());

        Self::new(query, vars).await
    }

    async fn new(query: String, vars: HashMap<&str, String>) -> Result<Self, reqwest::Error>  {
        let resp = graphql_req(query, vars).await?;

        let data = &resp["data"]["countries"][0];

        Ok(Country{
            code2: {
                data.get_str("code2")
            },
            code3: {
                data.get_str("code3")
            },
            created_at: {
                data.get_plaindt("created_at").unwrap()
            },
            id: {
                data.get_u64("id").unwrap()
            },
            intermediate_region: {
                data.get_str("intermediate_region")
            },
            intermediate_region_code: {
                data.get_str("intermediate_region_code")
            },
            iso_3166: {
                data.get_str("iso_3166")
            },
            name: {
                data.get_str("name")
            },
            phone_code: {
                data.get_str("phone_code")
            },
            region: {
                data.get_str("region")
            },
            region_code: {
                data.get_str("region_code")
            },
            sub_region: {
                data.get_str("sub_region")
            },
            sub_region_code: {
                data.get_str("sub_region_code")
            },
            updated_at: {
                data.get_plaindt("updated_at").unwrap()
            },
        })
    }
}