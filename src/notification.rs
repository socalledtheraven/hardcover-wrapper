use serde_json::Value;
use time::OffsetDateTime;
use crate::base_hardcover_item::BaseHardcoverItem;
use crate::graphql::{GraphQLResponse};

const QUERY_FIELDS: &str = r#"
created_at
description
id
link
link_text
notification_type_id
notifier_user_id
priority
title
uid
"#;

#[derive(Debug, Clone)]
pub struct Notification {
    pub created_at: OffsetDateTime,
    pub description: String,
    pub id: u64,
    pub link: Option<String>,
    pub link_text: Option<String>,
    pub notification_type_id: u64,
    pub notifier_user_id: u64,
    pub priority: Option<u64>,
    pub title: String,
    pub uid: String,
}

impl BaseHardcoverItem for Notification {
    async fn from_id(id: u64) -> Result<Self, reqwest::Error> {
        let query = r#"
        query GetNotification($id: Int!) {
          notifications_by_pk(id: $id) {"#.to_string() + QUERY_FIELDS + r#"
          }
        }
        "#;

        let data = Self::from_data(query, id).await?;

        Ok(Self::new(data["notifications_by_pk"].clone()))
    }

    fn new(data: Value) -> Self {
        Notification {
            created_at: {
                data.get_offsetdt("created_at").unwrap()
            },
            description: {
                data.get_str("description").unwrap()
            },
            id: {
                data.get_u64("id").unwrap()
            },
            link: {
                data.get_str("link")
            },
            link_text: {
                data.get_str("link_text")
            },
            notification_type_id: {
                data.get_u64("notification_type_id").unwrap()
            },
            notifier_user_id: {
                data.get_u64("notifier_user_id").unwrap()
            },
            priority: {
                data.get_u64("priority")
            },
            title: {
                data.get_str("title").unwrap()
            },
            uid: {
                data.get_str("uid").unwrap()
            },
        }
    }
}