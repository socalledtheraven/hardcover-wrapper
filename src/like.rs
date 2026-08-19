use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use crate::activity::Activity;
use crate::list::List;
use crate::user::User;
use crate::user_book::UserBook;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct Like {
    activity: Option<Activity>,
    created_at: Option<OffsetDateTime>,
    followers: Vec<User>,
    id: u32,
    likeable_id: u32,
    likeable_type: String,
    list: Option<List>,
    user: User,
    user_book: Option<UserBook>,
    user_id: u32,
}