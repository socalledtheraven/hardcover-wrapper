use crate::base_hardcover_item::BaseHardcoverItem;
use crate::client::GraphQLResponse;
use crate::util::PrivacySetting;
use crate::HardcoverClient;
use reqwest::Error;
use serde_json::Value;
use time::PlainDateTime;

const QUERY_FIELDS: &str = r#"
books_generated_at
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
pub enum RecommendationType {
    Book,
}

#[derive(Clone, Debug)]
pub enum VibeType {
    Custom,
    Recommendation,
    Dynamic,
    TopPicks,
}

#[derive(Debug, Clone)]
pub struct Vibe {
    pub books_generated_at: Option<PlainDateTime>,
    pub created_at: PlainDateTime,
    pub description: Option<String>,
    pub featured: bool,
    pub featured_at: Option<PlainDateTime>,
    pub id: u64,
    pub likes_count: u64,
    pub object_type: String,
    pub privacy_setting_id: PrivacySetting,
    pub result_type: RecommendationType,
    pub slug: String,
    pub title: String,
    pub updated_at: PlainDateTime,
    pub user_id: u64,
    pub vibe_type: VibeType,
}

impl BaseHardcoverItem for Vibe {
    async fn from_id(id: u64, client: &HardcoverClient) -> Result<Self, Error> {
        let query = r#"
        query GetVibe($id: Int!) {
          vibes_by_pk(id: $id) {"#
            .to_string()
            + QUERY_FIELDS
            + r#"
          }
        }
        "#;

        let data = Self::from_data(query, id, client).await?;

        Ok(Self::new(data["vibes_by_pk"].clone()))
    }

    fn new(data: Value) -> Self {
        Vibe {
            books_generated_at: { data.get_plaindt("books_generated_at") },
            created_at: { data.get_plaindt("created_at").unwrap() },
            description: { data.get_str("description") },
            featured: { data.get_bool("featured").unwrap() },
            featured_at: { data.get_plaindt("featured_at") },
            id: { data.get_u64("id").unwrap() },
            likes_count: { data.get_u64("likes_count").unwrap() },
            object_type: { data.get_str("object_type").unwrap() },
            privacy_setting_id: { data.get_privacysetting("privacy_setting_id") },
            result_type: {
                // currently the only option
                match data.get_u64("result_type") {
                    Some(0) => RecommendationType::Book,
                    _ => {
                        panic!("Unknown result_type: {:?}", data.get_u64("result_type"))
                    }
                }
            },
            slug: { data.get_str("slug").unwrap() },
            title: { data.get_str("title").unwrap() },
            updated_at: { data.get_plaindt("updated_at").unwrap() },
            user_id: { data.get_u64("user_id").unwrap() },
            vibe_type: {
                match data.get_u64("vibe_type") {
                    Some(0) => VibeType::Custom,
                    Some(1) => VibeType::Recommendation,
                    Some(2) => VibeType::Dynamic,
                    Some(3) => VibeType::TopPicks,
                    _ => {
                        panic!("Unknown vibe_type: {:?}", data.get_u64("vibe_type"))
                    }
                }
            },
        }
    }
}
