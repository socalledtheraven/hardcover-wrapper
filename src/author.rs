use crate::base_hardcover_item::BaseHardcoverItem;
use crate::client::GraphQLResponse;
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

#[derive(Debug, Clone)]
pub struct Author {
    pub alias_id: Option<u64>,
    pub alternate_names: Vec<String>,
    pub bio: Option<String>,
    pub books_count: u64,
    pub born_date: Option<Date>,
    pub born_year: Option<u64>,
    pub canonical_id: Option<u64>,
    pub death_date: Option<Date>,
    pub death_year: Option<u64>,
    pub gender_id: Option<Gender>,
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
        Author {
            alias_id: { data.get_u64("alias_id") },
            alternate_names: { data.get_str_vec("alternate_names") },
            bio: { data.get_str("bio") },
            books_count: { data.get_u64("books_count").unwrap() },
            born_date: { data.get_date("born_date") },
            born_year: { data.get_u64("born_year") },
            canonical_id: { data.get_u64("canonical_id") },
            death_date: { data.get_date("death_date") },
            death_year: { data.get_u64("death_year") },
            gender_id: {
                match data["gender_id"].as_u64() {
                    Some(id) => match id {
                        1 => Some(Gender::Female),
                        2 => Some(Gender::Male),
                        3 => Some(Gender::Nonbinary),
                        _ => panic!("Unknown gender id: {}", id),
                    },
                    None => None,
                }
            },
            id: { data.get_u64("id").unwrap() },
            identifiers: { data.get_identifiers("identifiers") },
            image_id: { data.get_u64("image_id") },
            is_bipoc: { data.get_bool("is_bipoc") },
            is_lgbtq: { data.get_bool("is_lgbtq") },
            links: { data.get_link_vec("links") },
            location: { data.get_str("location") },
            locked: { data.get_bool("locked").unwrap() },
            name: { data.get_str("name").unwrap() },
            name_personal: { data.get_str("name_personal") },
            object_type: { data.get_str("object_type").unwrap() },
            slug: { data.get_str("slug") },
            state: {
                match data["state"].as_str() {
                    Some("active") => RecordState::Active,
                    Some("duplicate") => RecordState::Duplicate,
                    _ => panic!("Unknown record state: {:?}", data["state"].as_str()),
                }
            },
            title: { data.get_str("title").filter(|s| !s.is_empty()) },
            user_id: { data.get_u64("user_id") },
            users_count: { data.get_u64("users_count").unwrap() },
        }
    }
}
