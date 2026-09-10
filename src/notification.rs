use crate::date_parsing;
use serde::Deserialize;
use crate::base_hardcover_item::BaseHardcoverItem;
use crate::HardcoverClient;
use serde_json::Value;
use time::OffsetDateTime;

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

#[derive(Debug, Clone, Deserialize)]
pub struct Notification {
    #[serde(deserialize_with = "date_parsing::offset_datetime")]
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
    async fn from_id(id: u64, client: &HardcoverClient) -> Result<Self, reqwest::Error> {
        let query = r#"
        query GetNotification($id: Int!) {
          notifications_by_pk(id: $id) {"#
            .to_string()
            + QUERY_FIELDS
            + r#"
          }
        }
        "#;

        let data = Self::from_data(query, id, client).await?;

        Ok(Self::new(data["notifications_by_pk"].clone()))
    }

    fn new(data: Value) -> Self {
        serde_json::from_value(data).unwrap()
    }
}
