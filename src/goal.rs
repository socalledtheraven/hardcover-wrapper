use serde::{Deserialize, Serialize};
use serde_json::Value;
use time::{Date, OffsetDateTime};
use crate::user::User;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct Goal {
    archived: bool,
    completed_at: Option<OffsetDateTime>,
    // todo!
    conditions: Value,
    description: Option<String>,
    end_date: Date,
    followers: Vec<User>,
    goal: u32,
    id: u32,
    metric: String,
    privacy_setting_id: Option<u32>,
    progress: f32,
    start_date: Date,
    state: String,
    user: User,
    user_id: u32,
}