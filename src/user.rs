use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use time::{Date, OffsetDateTime, PlainDateTime};
use crate::{AccountStatus, BlockedUser, Follow, Genre, Goal, Image, Import, Link, List, MaybeInit, NotificationDelivery, PrivacySetting, Prompt, PromptAnswer, Tagging, UserBook, UserFlag};
use crate::activity::Activity;
use crate::graphql::{graphql_req, get_bool_from_resp, get_date_from_resp, get_offsetdatetime_from_resp, get_plaindatetime_from_resp, get_str_from_resp, get_u64_from_resp, get_privacysetting_from_resp};


const QUERY_FIELDS: &str = r"
            access_level
            account_privacy_setting_id
            activity_privacy_settings_id
            admin
            bio
            birthdate
            books_count
            cached_cover
            cached_genres
            cached_image
            confirmation_sent_at
            confirmed_at
            created_at
            current_sign_in_at
            email
            email_verified
            flair
            followed_users_count
            followers_count
            id
            image_id
            last_activity_at
            last_sign_in_at
            librarian_roles
            link
            location
            locked_at
            membership
            membership_ends_at
            name
            object_type
            onboarded
            payment_system_id
            pro
            pronoun_personal
            pronoun_possessive
            referrer_id
            referrer_url
            remember_created_at
            reset_password_sent_at
            sign_in_count
            status_id
            timezone
            unconfirmed_email
            updated_at
            username";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct User {
    access_level: Option<u64>,
    account_privacy_settings_id: PrivacySetting,
    // todo!
    activities: MaybeInit<Vec<Activity>>,
    activity_privacy_settings_id: PrivacySetting,
    admin: bool,
    bio: Option<String>,
    birthdate: Option<Date>,
    // todo!
    blocked_users: MaybeInit<Vec<BlockedUser>>,
    books_count: u64,
    cached_cover: Image,
    cached_genres: Vec<Genre>,
    cached_image: Image,
    collection_imports: MaybeInit<Vec<Import>>,
    confirmation_sent_at: Option<PlainDateTime>,
    confirmed_at: Option<PlainDateTime>,
    created_at: Option<OffsetDateTime>,
    current_sign_in_at: Option<PlainDateTime>,
    email: Option<String>,
    email_verified: Option<OffsetDateTime>,
    flair: Option<String>,
    // todo!
    followed_by_users: MaybeInit<Vec<User>>,
    // todo!
    followed_lists: MaybeInit<Vec<List>>,
    // todo!
    followed_prompts: MaybeInit<Vec<Prompt>>,
    // todo!
    followed_users: MaybeInit<Vec<User>>,
    followed_users_count: u64,
    followers_count: u64,
    // todo!
    follows: MaybeInit<Vec<Follow>>,
    // todo!
    goals: MaybeInit<Vec<Goal>>,
    id: u64,
    // todo!
    images: MaybeInit<Vec<Image>>,
    image_id: u64,
    last_activity_at: Option<PlainDateTime>,
    last_sign_in_at: Option<PlainDateTime>,
    librarian_roles: Vec<String>,
    link: Option<String>,
    // todo!
    links: MaybeInit<Option<Vec<Option<Link>>>>,
    // todo!
    lists: MaybeInit<Vec<List>>,
    location: Option<String>,
    locked_at: Option<PlainDateTime>,
    membership: Option<String>,
    membership_ends_at: Option<PlainDateTime>,
    name: Option<String>,
    // todo!
    notification_deliveries: MaybeInit<Vec<NotificationDelivery>>,
    object_type: Option<String>,
    onboarded: bool,
    payment_system_id: Option<u64>,
    pro: bool,
    // todo!
    prompt_answers: MaybeInit<Vec<PromptAnswer>>,
    // todo!
    prompts: MaybeInit<Vec<Prompt>>,
    pronoun_personal: String,
    pronoun_possessive: String,
    referrer_id: Option<u64>,
    referrer_url: Option<String>,
    // todo!
    referred_users: MaybeInit<Vec<UserBook>>,
    remember_created_at: Option<PlainDateTime>,
    // todo!
    reported_user_flags: MaybeInit<Vec<UserFlag>>,
    reset_password_sent_at: Option<PlainDateTime>,
    sign_in_count: Option<u64>,
    status_id: AccountStatus,
    // todo!
    taggings: MaybeInit<Vec<Tagging>>,
    timezone: Option<String>,
    unconfirmed_email: Option<String>,
    updated_at: OffsetDateTime,
    // todo!
    user_books: MaybeInit<Vec<UserBook>>,
    // todo!
    user_flags: MaybeInit<Vec<UserFlag>>,
    username: String,
}
impl User {
    pub(crate) async fn from_username(username: &str) -> Result<Self, reqwest::Error> {
        let query = r#"
        query GetUser($user: citext!) {
          users(where: {username: {_eq: $user}}, limit: 1) {"#.to_string() + QUERY_FIELDS + r#"
          }
        }
        "#;

        let mut vars = HashMap::new();
        vars.insert("user", username.to_string());

        Self::new(query, vars).await
    }

    pub(crate) async fn from_user_id(user_id: u64) -> Result<Self, reqwest::Error> {
        let query = r#"
        query GetUser($user: Int!) {
          users(where: {id: {_eq: $user}}, limit: 1) {"#.to_string() + QUERY_FIELDS + r#"
          }
        }
        "#;

        let mut vars = HashMap::new();
        vars.insert("user", user_id.to_string());

        Self::new(query, vars).await
    }

    async fn new(query: String, vars: HashMap<&str, String>) -> Result<Self, reqwest::Error>  {
        let resp = graphql_req(query, vars).await?;

        let data = &resp["data"]["users"][0];

        Ok(User {
            access_level: {
                get_u64_from_resp(data, "access_level")
            },
            account_privacy_settings_id: {
                // in either case of it being unknown, we default to private
                // this will never happen, but better to fail closed
                data["account_privacy_setting_id"]
                    .as_u64()
                    .map(|v| match v {
                        1 => PrivacySetting::Public,
                        2 => PrivacySetting::FollowersOnly,
                        3 => PrivacySetting::Private,
                        _ => PrivacySetting::Private,
                    })
                    .unwrap_or(PrivacySetting::Private)
            },
            activities: { MaybeInit::Uninitialised },
            activity_privacy_settings_id: {
                get_privacysetting_from_resp(data, "activity_privacy_settings_id")
            },
            admin: {
                // should always be present
                get_bool_from_resp(data, "admin")
            },
            bio: {
                get_str_from_resp(data, "bio")
            },
            birthdate: {
                get_date_from_resp(data, "birthdate")
            },
            blocked_users: { MaybeInit::Uninitialised },
            books_count: {
                get_u64_from_resp(data, "books_count").unwrap()
            },
            cached_cover: {
                Image::new(data["cached_cover"].clone())
            },
            cached_genres: {
                let x = data["cached_genres"].as_array().unwrap();
                x.iter().filter_map(|v| {
                    let count = v["count"].as_u64()?;
                    let tag = v["tag"].as_str()?.to_string();
                    let tag_slug = v["tag_slug"].as_str()?.to_string();
                    Some(Genre { count, tag, tag_slug })
                }).collect()
            },
            cached_image: { Image::new(data["cached_image"].clone()) },
            collection_imports: { MaybeInit::Uninitialised },
            confirmation_sent_at: {
                get_plaindatetime_from_resp(data, "confirmation_sent_at")
            },
            confirmed_at: {
                get_plaindatetime_from_resp(data, "confirmed_at")
            },
            created_at: {
                get_offsetdatetime_from_resp(data, "created_at")
            },
            current_sign_in_at: {
                get_plaindatetime_from_resp(data, "current_sign_in_at")
            },
            email: {
                get_str_from_resp(data, "email")
            },
            email_verified: {
                get_offsetdatetime_from_resp(data, "email_verified")
            },
            flair: {
                get_str_from_resp(data, "flair")
            },
            followed_by_users: { MaybeInit::Uninitialised },
            followed_lists: { MaybeInit::Uninitialised },
            followed_prompts: { MaybeInit::Uninitialised },
            followed_users: { MaybeInit::Uninitialised },
            followed_users_count: {
                get_u64_from_resp(data, "followed_users_count").unwrap()
            },
            followers_count: {
                get_u64_from_resp(data, "followers_count").unwrap()
            },
            follows: { MaybeInit::Uninitialised },
            goals: { MaybeInit::Uninitialised },
            id: get_u64_from_resp(data, "id").unwrap(),
            images: { MaybeInit::Uninitialised },
            image_id: get_u64_from_resp(data, "image_id").unwrap(),
            last_activity_at: get_plaindatetime_from_resp(data, "last_activity_at"),
            last_sign_in_at: get_plaindatetime_from_resp(data, "last_sign_in_at"),
            librarian_roles: {
                let x = data["librarian_roles"].as_array().unwrap();
                x.iter().filter_map(|v| {
                    v.as_str().map(|s| s.to_string())
                }).collect()
            },
            link: {
                get_str_from_resp(data, "link")
            },
            links: { MaybeInit::Uninitialised },
            lists: { MaybeInit::Uninitialised },
            location: {
                get_str_from_resp(data, "location")
            },
            locked_at: {
                get_plaindatetime_from_resp(data, "locked_at")
            },
            membership: {
                get_str_from_resp(data, "membership")
            },
            membership_ends_at: {
                get_plaindatetime_from_resp(data, "membership_ends_at")
            },
            name: {
                get_str_from_resp(data, "name")
            },
            notification_deliveries: { MaybeInit::Uninitialised },
            object_type: {
                get_str_from_resp(data, "object_type")
            },
            onboarded: {
                get_bool_from_resp(data, "onboarded")
            },
            payment_system_id: {
                get_u64_from_resp(data, "payment_system_id")
            },
            pro: {
                get_bool_from_resp(data, "pro")
            },
            prompt_answers: { MaybeInit::Uninitialised },
            prompts: { MaybeInit::Uninitialised },
            pronoun_personal: {
                get_str_from_resp(data, "pronoun_personal").unwrap()
            },
            pronoun_possessive: {
                get_str_from_resp(data, "pronoun_possessive").unwrap()
            },
            referrer_id: {
                get_u64_from_resp(data, "referrer_id")
            },
            referrer_url: {
                get_str_from_resp(data, "referrer_url")
            },
            referred_users: { MaybeInit::Uninitialised },
            remember_created_at: {
                get_plaindatetime_from_resp(data, "remember_created_at")
            },
            reported_user_flags: { MaybeInit::Uninitialised },
            reset_password_sent_at: {
                get_plaindatetime_from_resp(data, "reset_password_sent_at")
            },
            sign_in_count: {
                get_u64_from_resp(data, "sign_in_count")
            },
            status_id: {
                // we default to activated if the status is unknown, this will never happen, but better to fail open
                data["status_id"]
                    .as_u64()
                    .map(|v| match v {
                        1 => AccountStatus::Created,
                        2 => AccountStatus::Activated,
                        3 => AccountStatus::Banned,
                        _ => AccountStatus::Activated,
                    })
                    .unwrap_or(AccountStatus::Activated)
            },
            taggings: { MaybeInit::Uninitialised },
            timezone: {
                get_str_from_resp(data, "timezone")
            },
            unconfirmed_email: {
                get_str_from_resp(data, "unconfirmed_email")
            },
            updated_at: {
                get_offsetdatetime_from_resp(data, "updated_at").unwrap()
            },
            user_books: { MaybeInit::Uninitialised },
            user_flags: { MaybeInit::Uninitialised },
            username: {
                get_str_from_resp(data, "username").unwrap()
            },
        })
    }

    pub fn access_level(&self) -> Option<u64> {
        self.access_level
    }

    pub fn account_privacy_settings_id(&self) -> &PrivacySetting {
        &self.account_privacy_settings_id
    }

    pub async fn activities(&self) -> Result<Vec<Activity>, reqwest::Error> {
        Activity::of_user(self.id).await
    }

    pub fn activity_privacy_settings_id(&self) -> &PrivacySetting {
        &self.activity_privacy_settings_id
    }

    pub fn admin(&self) -> bool {
        self.admin
    }

    pub fn bio(&self) -> &Option<String> {
        &self.bio
    }

    pub fn birthdate(&self) -> Option<Date> {
        self.birthdate
    }

    pub fn blocked_users(&self) -> &MaybeInit<Vec<BlockedUser>> {
        &self.blocked_users
    }

    pub fn books_count(&self) -> u64 {
        self.books_count
    }

    pub fn cached_cover(&self) -> &Image {
        &self.cached_cover
    }

    pub fn cached_genres(&self) -> &Vec<Genre> {
        &self.cached_genres
    }

    pub fn cached_image(&self) -> &Image {
        &self.cached_image
    }

    pub fn collection_imports(&self) -> &MaybeInit<Vec<Import>> {
        &self.collection_imports
    }

    pub fn confirmation_sent_at(&self) -> Option<PlainDateTime> {
        self.confirmation_sent_at
    }

    pub fn confirmed_at(&self) -> Option<PlainDateTime> {
        self.confirmed_at
    }

    pub fn created_at(&self) -> Option<OffsetDateTime> {
        self.created_at
    }

    pub fn current_sign_in_at(&self) -> Option<PlainDateTime> {
        self.current_sign_in_at
    }

    pub fn email(&self) -> &Option<String> {
        &self.email
    }

    pub fn email_verified(&self) -> Option<OffsetDateTime> {
        self.email_verified
    }

    pub fn flair(&self) -> &Option<String> {
        &self.flair
    }

    pub fn followed_by_users(&self) -> &MaybeInit<Vec<User>> {
        &self.followed_by_users
    }

    pub fn followed_lists(&self) -> &MaybeInit<Vec<List>> {
        &self.followed_lists
    }

    pub fn followed_prompts(&self) -> &MaybeInit<Vec<Prompt>> {
        &self.followed_prompts
    }

    pub fn followed_users(&self) -> &MaybeInit<Vec<User>> {
        &self.followed_users
    }

    pub fn followed_users_count(&self) -> u64 {
        self.followed_users_count
    }

    pub fn followers_count(&self) -> u64 {
        self.followers_count
    }

    pub fn follows(&self) -> &MaybeInit<Vec<Follow>> {
        &self.follows
    }

    pub fn goals(&self) -> &MaybeInit<Vec<Goal>> {
        &self.goals
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn images(&self) -> &MaybeInit<Vec<Image>> {
        &self.images
    }

    pub fn image_id(&self) -> u64 {
        self.image_id
    }

    pub fn last_activity_at(&self) -> Option<PlainDateTime> {
        self.last_activity_at
    }

    pub fn last_sign_in_at(&self) -> Option<PlainDateTime> {
        self.last_sign_in_at
    }

    pub fn librarian_roles(&self) -> &Vec<String> {
        &self.librarian_roles
    }

    pub fn link(&self) -> &Option<String> {
        &self.link
    }

    pub fn links(&self) -> &MaybeInit<Option<Vec<Option<Link>>>> {
        &self.links
    }

    pub fn lists(&self) -> &MaybeInit<Vec<List>> {
        &self.lists
    }

    pub fn location(&self) -> &Option<String> {
        &self.location
    }

    pub fn locked_at(&self) -> Option<PlainDateTime> {
        self.locked_at
    }

    pub fn membership(&self) -> &Option<String> {
        &self.membership
    }

    pub fn membership_ends_at(&self) -> Option<PlainDateTime> {
        self.membership_ends_at
    }

    pub fn name(&self) -> &Option<String> {
        &self.name
    }

    pub fn notification_deliveries(&self) -> &MaybeInit<Vec<NotificationDelivery>> {
        &self.notification_deliveries
    }

    pub fn object_type(&self) -> &Option<String> {
        &self.object_type
    }

    pub fn onboarded(&self) -> bool {
        self.onboarded
    }

    pub fn payment_system_id(&self) -> Option<u64> {
        self.payment_system_id
    }

    pub fn pro(&self) -> bool {
        self.pro
    }

    pub fn prompt_answers(&self) -> &MaybeInit<Vec<PromptAnswer>> {
        &self.prompt_answers
    }

    pub fn prompts(&self) -> &MaybeInit<Vec<Prompt>> {
        &self.prompts
    }

    pub fn pronoun_personal(&self) -> &str {
        &self.pronoun_personal
    }

    pub fn pronoun_possessive(&self) -> &str {
        &self.pronoun_possessive
    }

    pub fn referrer_id(&self) -> Option<u64> {
        self.referrer_id
    }

    pub fn referrer_url(&self) -> &Option<String> {
        &self.referrer_url
    }

    pub fn referred_users(&self) -> &MaybeInit<Vec<UserBook>> {
        &self.referred_users
    }

    pub fn remember_created_at(&self) -> Option<PlainDateTime> {
        self.remember_created_at
    }

    pub fn reported_user_flags(&self) -> &MaybeInit<Vec<UserFlag>> {
        &self.reported_user_flags
    }

    pub fn reset_password_sent_at(&self) -> Option<PlainDateTime> {
        self.reset_password_sent_at
    }

    pub fn sign_in_count(&self) -> Option<u64> {
        self.sign_in_count
    }

    pub fn status_id(&self) -> &AccountStatus {
        &self.status_id
    }

    pub fn taggings(&self) -> &MaybeInit<Vec<Tagging>> {
        &self.taggings
    }

    pub fn timezone(&self) -> &Option<String> {
        &self.timezone
    }

    pub fn unconfirmed_email(&self) -> &Option<String> {
        &self.unconfirmed_email
    }

    pub fn updated_at(&self) -> OffsetDateTime {
        self.updated_at
    }

    pub fn user_books(&self) -> &MaybeInit<Vec<UserBook>> {
        &self.user_books
    }

    pub fn user_flags(&self) -> &MaybeInit<Vec<UserFlag>> {
        &self.user_flags
    }

    pub fn username(&self) -> &str {
        &self.username
    }
}