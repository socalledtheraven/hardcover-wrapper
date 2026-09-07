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

#[derive(Clone, Debug)]
pub(crate) struct AuthorIdentifiers {
    pub(crate) audible: Option<Vec<String>>,
    pub(crate) goodreads: Option<Vec<String>>,
    pub(crate) openlibrary: Option<Vec<String>>,
}

#[derive(Clone, Debug)]
pub(crate) struct Link {
    pub(crate) url: String,
    pub(crate) title: String,
}

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
    pub(crate) identifiers: AuthorIdentifiers,
    pub(crate) image_id: Option<u64>,
    pub(crate) is_bipoc: Option<bool>,
    pub(crate) is_lgbtq: Option<bool>,
    pub(crate) links: Vec<Link>,
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
                data.get_u64("alias_id")
            },
            alternate_names: {
                data.get_str_vec("alternate_names")
            },
            bio: {
                data.get_str("bio")
            },
            books_count: {
                data.get_u64("books_count").unwrap()
            },
            born_date: {
                data.get_date("born_date")
            },
            born_year: {
                data.get_u64("born_year")
            },
            cached_image: {
                Image::new(data["cached_image"].clone())
            },
            canonical_id: {
                data.get_u64("canonical_id")
            },
            death_date: {
                data.get_date("death_date")
            },
            death_year: {
                data.get_u64("death_year")
            },
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
            id: {
                data.get_u64("id").unwrap()
            },
            identifiers: {
                let data = &data["identifiers"];

                AuthorIdentifiers {
                    audible: {
                        data.get_opt_str_vec("audible")
                    },
                    goodreads: {
                        data.get_opt_str_vec("goodreads")
                    },
                    openlibrary: {
                        data.get_opt_str_vec("openlibrary")
                    },
                }
            },
            image_id: {
                data.get_u64("image_id")
            },
            is_bipoc: {
                data["is_bipoc"].as_bool()
            },
            is_lgbtq: {
                data["is_lgbtq"].as_bool()
            },
            links: {
                data["links"].as_array()
                    .unwrap_or(&vec![])
                    .iter()
                    .map(|link| Link {
                        url: link.get_str("url").unwrap_or_default(),
                        title: link.get_str("title").unwrap_or_default(),
                    })
                    .collect()
            },
            location: {
                data.get_str("location")
            },
            locked: {
                data.get_bool("locked")
            },
            name: {
                data.get_str("name").unwrap()
            },
            name_personal: {
                data.get_str("name_personal")
            },
            object_type: {
                data.get_str("object_type").unwrap()
            },
            slug: {
                data.get_str("slug")
            },
            state: {
                match data["state"].as_str() {
                    Some("active") => RecordState::Active,
                    Some("duplicate") => RecordState::Duplicate,
                    _ => panic!("Unknown record state: {:?}", data["state"].as_str()),
                }
            },
            title: {
                data.get_str("title").filter(|s| !s.is_empty())
            },
            user_id: {
                data.get_u64("user_id")
            },
            users_count: {
                data.get_u64("users_count").unwrap()
            },
        }
    }
}