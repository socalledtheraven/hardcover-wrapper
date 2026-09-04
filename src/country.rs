use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use time::PlainDateTime;
use crate::graphql::{get_plaindatetime_from_resp, get_str_from_resp, get_u64_from_resp, graphql_req};

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
        query GetAuthor($country: Int!) {
          countries(where: {id: {_eq: $country}}, limit: 1) {"#.to_string() + QUERY_FIELDS + r#"
          }
        }
        "#;

        Self::from_data(query, country_id).await
    }

    async fn from_data<T: ToString>(query: String, user_data: T) -> Result<Self, reqwest::Error> {
        let mut vars = HashMap::new();
        vars.insert("country", user_data.to_string());

        Self::new(query, vars).await
    }

    async fn new(query: String, vars: HashMap<&str, String>) -> Result<Self, reqwest::Error>  {
        let resp = graphql_req(query, vars).await?;

        let data = &resp["data"]["countries"][0];

        Ok(Country{
            code2: {
                get_str_from_resp(data, "code2")
            },
            code3: {
                get_str_from_resp(data, "code3")
            },
            created_at: {
                get_plaindatetime_from_resp(data, "created_at").unwrap()
            },
            id: {
                get_u64_from_resp(data, "id").unwrap()
            },
            intermediate_region: {
                get_str_from_resp(data, "intermediate_region")
            },
            intermediate_region_code: {
                get_str_from_resp(data, "intermediate_region_code")
            },
            iso_3166: {
                get_str_from_resp(data, "iso_3166")
            },
            name: {
                get_str_from_resp(data, "name")
            },
            phone_code: {
                get_str_from_resp(data, "phone_code")
            },
            region: {
                get_str_from_resp(data, "region")
            },
            region_code: {
                get_str_from_resp(data, "region_code")
            },
            sub_region: {
                get_str_from_resp(data, "sub_region")
            },
            sub_region_code: {
                get_str_from_resp(data, "sub_region_code")
            },
            updated_at: {
                get_plaindatetime_from_resp(data, "updated_at").unwrap()
            },
        })
    }
}