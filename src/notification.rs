use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use crate::user::User;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct NotificationDelivery {
    created_at: OffsetDateTime,
    description: String,
    id: u32,
    link: Option<String>,
    link_text: Option<String>,
    notification_deliveries: Vec<NotificationDelivery>,
    notification_type_id: u32,
    notifierUser: User,
    notifier_user_id: u32,
    priority: Option<u32>,
    title: String,
    uid: String,
}