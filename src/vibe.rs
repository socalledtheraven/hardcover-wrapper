use reqwest::Error;
use serde_json::Value;
use time::PlainDateTime;
use crate::base_hardcover_item::BaseHardcoverItem;
use crate::util::PrivacySetting;
use crate::graphql::GraphQLResponse;

const QUERY_FIELDS: &str = r#"
books_generated_at
cached_book_ids
created_at
description
featured
featured_at
id
likes_count
object_type
privacy_setting_id
result_type
slug
title
updated_at
user_id
vibe_type
"#;

#[derive(Clone, Debug)]
pub(crate) enum RecommendationType {
    Book
}

#[derive(Clone, Debug)]
pub(crate) enum VibeType {
    Custom,
    Recommendation,
    Dynamic,
    TopPicks
}

#[derive(Debug, Clone)]
pub(crate) struct Vibe {
    pub(crate) books_generated_at: Option<PlainDateTime>,
    pub(crate) created_at: PlainDateTime,
    pub(crate) description: Option<String>,
    pub(crate) featured: bool,
    pub(crate) featured_at: Option<PlainDateTime>,
    pub(crate) id: u64,
    pub(crate) likes_count: u64,
    pub(crate) object_type: String,
    pub(crate) privacy_setting_id: PrivacySetting,
    pub(crate) result_type: RecommendationType,
    pub(crate) slug: String,
    pub(crate) title: String,
    pub(crate) updated_at: PlainDateTime,
    pub(crate) user_id: u64,
    pub(crate) vibe_type: VibeType,
}

impl BaseHardcoverItem for Vibe {
    async fn from_id(id: u64) -> Result<Self, Error> {
        let query = r#"
        query GetVibe($id: Int!) {
          vibes_by_pk(id: $id) {"#.to_string() + QUERY_FIELDS + r#"
          }
        }
        "#;

        let data = Self::from_data(query, id).await?;

        Ok(Self::new(data["vibes_by_pk"].clone()))
    }

    fn new(data: Value) -> Self {
        Vibe {
            books_generated_at: {
                data.get_plaindt("books_generated_at")
            },
            created_at: {
                data.get_plaindt("created_at").unwrap()
            },
            description: {
                data.get_str("description")
            },
            featured: {
                data.get_bool("locked").unwrap()
            },
            featured_at: {
                data.get_plaindt("featured_at")
            },
            id: {
                data.get_u64("id").unwrap()
            },
            likes_count: {
                data.get_u64("likes_count").unwrap()
            },
            object_type: {
                data.get_str("object_type").unwrap()
            },
            privacy_setting_id: {
                data.get_privacysetting("privacy_setting_id")
            },
            result_type: {
                // currently the only option
                match data.get_u64("result_type") {
                    Some(0) => {
                        RecommendationType::Book
                    }
                    _ => {
                        panic!("Unknown result_type: {:?}", data.get_u64("result_type"))
                    }
                }
            },
            slug: {
                data.get_str("slug").unwrap()
            },
            title: {
                data.get_str("title").unwrap()
            },
            updated_at: {
                data.get_plaindt("updated_at").unwrap()
            },
            user_id: {
                data.get_u64("user_id").unwrap()
            },
            vibe_type: {
                match data.get_u64("vibe_type") {
                    Some(0) => {
                        VibeType::Custom
                    }
                    Some(1) => {
                        VibeType::Recommendation
                    }
                    Some(2) => {
                        VibeType::Dynamic
                    }
                    Some(3) => {
                        VibeType::TopPicks
                    }
                    _ => {
                        panic!("Unknown vibe_type: {:?}", data.get_u64("vibe_type"))
                    }
                }
            },
        }
    }
}