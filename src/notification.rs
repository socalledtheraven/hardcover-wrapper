use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use crate::graphql::{graphql_req, GraphQLResponse};

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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct Notification {
    created_at: OffsetDateTime,
    description: String,
    id: u64,
    link: Option<String>,
    link_text: Option<String>,
    notification_type_id: u64,
    notifier_user_id: u64,
    priority: Option<u64>,
    title: String,
    uid: String,
}

impl Notification {
    pub(crate) async fn from_notification_id(notification_id: u64) -> Result<Self, reqwest::Error> {
        let query = r#"
        query GetAuthor($notification: Int!) {
          notifications_by_pk(id: $notification) {"#.to_string() + QUERY_FIELDS + r#"
          }
        }
        "#;

        Self::from_data(query, notification_id).await
    }

    async fn from_data<T: ToString>(query: String, user_data: T) -> Result<Self, reqwest::Error> {
        let mut vars = HashMap::new();
        vars.insert("notification", user_data.to_string());

        Self::new(query, vars).await
    }

    async fn new(query: String, vars: HashMap<&str, String>) -> Result<Self, reqwest::Error> {
        let resp = graphql_req(query, vars).await?;

        let data = &resp["data"]["notifications_by_pk"];

        Ok(Notification{
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
        })
    }
}