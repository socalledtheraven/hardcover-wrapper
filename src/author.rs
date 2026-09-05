use serde_json::Value;
use time::Date;
use crate::base_hardcover_item::BaseHardcoverItem;
use crate::enums::{Gender, RecordState};
use crate::graphql::GraphQLResponse;
use crate::image::Image;

const QUERY_FIELDS: &str = r#"
alias_id
alternate_names
bio
books_count
born_date
born_year
cached_image
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
pub(crate) struct Author {
    pub(crate) alias_id: Option<u64>,
    pub(crate) alternate_names: Vec<String>,
    pub(crate) bio: Option<String>,
    pub(crate) books_count: u64,
    pub(crate) born_date: Option<Date>,
    pub(crate) born_year: Option<u64>,
    pub(crate) cached_image: Image,
    pub(crate) canonical_id: Option<u64>,
    pub(crate) death_date: Option<Date>,
    pub(crate) death_year: Option<u64>,
    pub(crate) gender_id: Option<Gender>,
    pub(crate) id: u64,
    pub(crate) identifiers: Value,
    pub(crate) image_id: Option<u64>,
    pub(crate) is_bipoc: Option<bool>,
    pub(crate) is_lgbtq: Option<bool>,
    pub(crate) links: Value,
    pub(crate) location: Option<String>,
    pub(crate) locked:	bool,
    pub(crate) name: String,
    pub(crate) name_personal: Option<String>,
    pub(crate) object_type: String,
    pub(crate) slug: Option<String>,
    pub(crate) state: RecordState,
    pub(crate) title: Option<String>,
    pub(crate) user_id: Option<u64>,
    pub(crate) users_count: u64,
}

impl BaseHardcoverItem for Author {
    // this is extra complicated because of duplicates and stuff
    // pub(crate) async fn from_name(name: &str) -> Result<Self, reqwest::Error> {
    //     let query = r#"
    //     query GetAuthor($author: String!) {
    //       authors(where: {name: {_eq: $author}}, limit: 1) {"#.to_string() + QUERY_FIELDS + r#"
    //       }
    //     }
    //     "#;
    //
    //     Self::from_data(query, name).await
    // }

    async fn from_id(id: u64) -> Result<Self, reqwest::Error> {
        let query = r#"
        query GetAuthor($id: Int!) {
          authors_by_pk(id: $id) {"#.to_string() + QUERY_FIELDS + r#"
          }
        }
        "#;

        let data = Self::from_data(query, id).await?;

        Ok(Self::new(data["authors_by_pk"].clone()))
    }

    fn new(data: Value) -> Self {
        Author {
            alias_id: {
                data["alias_id"].as_u64()
            },
            alternate_names: {
                data.get_str_vec("alternate_names")
            },
            bio: {
                data["bio"].as_str().map(|s| s.to_string())
            },
            books_count: {
                data["books_count"].as_u64().unwrap_or(0)
            },
            born_date: {
                data.get_date("born_date")
            },
            born_year: {
                data["born_year"].as_u64()
            },
            cached_image: {
                Image::new(data["cached_image"].clone())
            },
            canonical_id: {
                data["canonical_id"].as_u64()
            },
            death_date: {
                data.get_date("death_date")
            },
            death_year: {
                data["death_year"].as_u64()
            },
            gender_id: {
                match data["gender_id"].as_u64() {
                    Some(id) => match id {
                        1 => Some(Gender::Female),
                        2 => Some(Gender::Male),
                        3 => Some(Gender::Nonbinary),
                        _ => None,
                    },
                    None => None,
                }
            },
            id: {
                data["id"].as_u64().unwrap_or(0)
            },
            identifiers: {
                // todo
                data["identifiers"].clone()
            },
            image_id: {
                data["image_id"].as_u64()
            },
            is_bipoc: {
                data["is_bipoc"].as_bool()
            },
            is_lgbtq: {
                data["is_lgbtq"].as_bool()
            },
            links: {
                data["links"].clone()
            },
            location: {
                data["location"].as_str().map(|s| s.to_string())
            },
            locked: {
                data["locked"].as_bool().unwrap_or(false)
            },
            name: {
                data["name"].as_str().unwrap_or_default().to_string()
            },
            name_personal: {
                data["name_personal"].as_str().map(|s| s.to_string())
            },
            object_type: {
                // todo!
                data["object_type"].as_str().unwrap_or_default().to_string()
            },
            slug: {
                data["slug"].as_str().map(|s| s.to_string())
            },
            state: {
                match data["state"].as_str() {
                    Some("active") => RecordState::Active,
                    Some("duplicate") => RecordState::Duplicate,
                    _ => RecordState::Active,
                }
            },
            title: {
                data["title"].as_str().map(|s| s.to_string()).filter(|s| !s.is_empty())
            },
            user_id: {
                data["user_id"].as_u64()
            },
            users_count: {
                data["users_count"].as_u64().unwrap_or(0)
            },
        }
    }
}