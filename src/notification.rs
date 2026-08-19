use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use crate::user::User;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct NotificationDelivery {
    created_at: OffsetDateTime,
    description: String,
    id: u64,
    link: Option<String>,
    link_text: Option<String>,
    notification_deliveries: Vec<NotificationDelivery>,
    notification_type_id: u64,
    notifierUser: User,
    notifier_user_id: u64,
    priority: Option<u64>,
    title: String,
    uid: String,
}