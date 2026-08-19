use serde::{Deserialize, Serialize};
use serde_json::Value;
use time::OffsetDateTime;
use crate::enums::PrivacySetting;
use crate::user::User;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct Prompt {
    answers_count: u32,
    books_count: u32,
    created_at: Option<OffsetDateTime>	,
    description: String,
    featured: bool,
    // todo!
    followed_prompts: Vec<Value>,
    followers: Vec<User>,
    id: u32,
    privacy_setting: PrivacySetting,
    privacy_setting_id: u32,
    prompt_answers: Vec<PromptAnswer>,
    prompt_books: Vec<PromptBook>,
    question: String,
    slug: String,
    user: User,
    user_id: u32,
    users_count: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct PromptAnswer {}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct PromptBook {}