use serde::{Deserialize, Serialize};
use serde_json::Value;
use time::OffsetDateTime;
use crate::enums::PrivacySetting;
use crate::user::User;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct Prompt {
    answers_count: u64,
    books_count: u64,
    created_at: Option<OffsetDateTime>	,
    description: String,
    featured: bool,
    // todo!
    followed_prompts: Vec<Value>,
    followers: Vec<User>,
    id: u64,
    privacy_setting: PrivacySetting,
    privacy_setting_id: u64,
    prompt_answers: Vec<PromptAnswer>,
    prompt_books: Vec<PromptBook>,
    question: String,
    slug: String,
    user: User,
    user_id: u64,
    users_count: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct PromptAnswer {}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct PromptBook {}