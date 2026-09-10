use crate::date_parsing;
use serde::Deserialize;
use crate::base_hardcover_item::BaseHardcoverItem;
use crate::util::{Gender, Identifiers, Link, RecordState};
use crate::HardcoverClient;
use serde_json::Value;
use time::Date;

const QUERY_FIELDS: &str = r#"
alias_id
alternate_names
bio
books_count
born_date
born_year
canonical_id
death_date
death_year
gender_id
id
identifiers
image_id
is_bipoc
is_lgbtq
links
location
locked
name
name_personal
object_type
slug
state
title
user_id
users_count
"#;

#[derive(Debug, Clone, Deserialize)]
pub struct Author {
    pub alias_id: Option<u64>,
    pub alternate_names: Vec<String>,
    pub bio: Option<String>,
    pub books_count: u64,

    #[serde(deserialize_with = "date_parsing::date_optional")]
    pub born_date: Option<Date>,
    pub born_year: Option<u64>,
    pub canonical_id: Option<u64>,

    #[serde(deserialize_with = "date_parsing::date_optional")]
    pub death_date: Option<Date>,
    pub death_year: Option<u64>,
    #[serde(rename = "gender_id")]
    pub gender: Option<Gender>,
    pub id: u64,
    pub identifiers: Identifiers,
    pub image_id: Option<u64>,
    pub is_bipoc: Option<bool>,
    pub is_lgbtq: Option<bool>,
    pub links: Vec<Link>,
    pub location: Option<String>,
    pub locked: bool,
    pub name: String,
    pub name_personal: Option<String>,
    pub object_type: String,
    pub slug: Option<String>,
    pub state: RecordState,
    pub title: Option<String>,
    pub user_id: Option<u64>,
    pub users_count: u64,
}

impl BaseHardcoverItem for Author {
    // this is extra complicated because of duplicates and stuff
    // pub async fn from_name(name: &str) -> Result<Self, reqwest::Error> {
    //     let query = r#"
    //     query GetAuthor($author: String!) {
    //       authors(where: {name: {_eq: $author}}, limit: 1) {"#.to_string() + QUERY_FIELDS + r#"
    //       }
    //     }
    //     "#;
    //
    //     Self::from_data(query, name).await
    // }

    async fn from_id(id: u64, client: &HardcoverClient) -> Result<Self, reqwest::Error> {
        let query = r#"
        query GetAuthor($id: Int!) {
          authors_by_pk(id: $id) {"#
            .to_string()
            + QUERY_FIELDS
            + r#"
          }
        }
        "#;

        let data = Self::from_data(query, id, client).await?;

        Ok(Self::new(data["authors_by_pk"].clone()))
    }

    fn new(data: Value) -> Self {
        serde_json::from_value(data).unwrap()
    }
}
