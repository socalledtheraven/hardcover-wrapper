//! Notification model representing user alerts and activity notices.

use crate::utils::base_hardcover_item::BaseHardcoverItem;
use crate::utils::date_parsing;
use crate::HardcoverClient;
use serde::Deserialize;
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

/// Represents a notification delivered to a user.
#[derive(Debug, Clone, Deserialize)]
pub struct Notification {
    /// Timestamp when the notification was created.
    #[serde(deserialize_with = "date_parsing::offset_datetime")]
    pub created_at: OffsetDateTime,
    /// Body text description of the notification.
    pub description: String,
    /// Unique identifier for the notification.
    pub id: u64,
    /// Link target URL associated with the notification.
    pub link: Option<String>,
    /// Display text for the link.
    pub link_text: Option<String>,
    /// Notification category/type ID.
    pub notification_type_id: u64,
    /// User ID of the sender/notifier.
    pub notifier_user_id: u64,
    /// Notification display priority.
    pub priority: Option<u64>,
    /// Title header of the notification.
    pub title: String,
    /// Unique string identifier (UID).
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

        let variables = serde_json::json!({ "id": id });
        let data = Self::from_data(query, variables, client).await?;

        Ok(Self::from_value(data["notifications_by_pk"].clone()))
    }

    fn from_value(data: Value) -> Self {
        serde_json::from_value(data).unwrap()
    }
}
