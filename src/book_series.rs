use std::collections::HashMap;
use time::PlainDateTime;
use crate::graphql::{get_bool_from_resp, get_plaindatetime_from_resp, get_str_from_resp, get_u64_from_resp, graphql_req};

const QUERY_FIELDS: &str = r#"
book_id
compilation
created_at
details
featured
id
position
series_id
updated_at
"#;

pub(crate) struct BookSeries {
    book_id: u64,
    compilation: bool,
    created_at: PlainDateTime,
    details: String,
    featured: bool,
    id: u64,
    position: f64,
    series_id: u64,
    updated_at: PlainDateTime
}

impl BookSeries {
    pub(crate) async fn from_book_series_id(book_series_id: u64) -> Result<Self, reqwest::Error> {
        let query = r#"
        query GetBookSeries($series: Int!) {
          books(where: {id: {_eq: $series}}, limit: 1) {"#.to_string() + QUERY_FIELDS + r#"
          }
        }
        "#;

        Self::from_data(query, book_series_id).await
    }

    async fn from_data<T: ToString>(query: String, user_data: T) -> Result<Self, reqwest::Error> {
        let mut vars = HashMap::new();
        vars.insert("series", user_data.to_string());

        Self::new(query, vars).await
    }

    async fn new(query: String, vars: HashMap<&str, String>) -> Result<Self, reqwest::Error>  {
        let resp = graphql_req(query, vars).await?;

        let data = &resp["data"]["book_series"][0];

        Ok(BookSeries{
            book_id: {
                get_u64_from_resp(data, "book_id").unwrap()
            },
            compilation: {
                get_bool_from_resp(data, "compilation")
            },
            created_at: {
                get_plaindatetime_from_resp(data, "created_at").unwrap()
            },
            details: {
                get_str_from_resp(data, "details").unwrap()
            },
            featured: {
                get_bool_from_resp(data, "featured")
            },
            id: {
                get_u64_from_resp(data, "id").unwrap()
            },
            position: {
                data["position"].as_f64().unwrap()
            },
            series_id: {
                get_u64_from_resp(data, "series_id").unwrap()
            },
            updated_at: {
                get_plaindatetime_from_resp(data, "updated_at").unwrap()
            },
        })
    }
}