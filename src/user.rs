use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use time::{Date, OffsetDateTime, PlainDateTime};
use crate::enums::PrivacySetting;
use crate::genre::Genre;
use crate::graphql::{graphql_req, get_bool_from_resp, get_date_from_resp, get_offsetdatetime_from_resp, get_plaindatetime_from_resp, get_str_from_resp, get_u64_from_resp, get_privacysetting_from_resp, get_str_vec};
use crate::image::Image;

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
    activity_privacy_settings_id: PrivacySetting,
    admin: bool,
    bio: Option<String>,
    birthdate: Option<Date>,
    books_count: u64,
    cached_cover: Image,
    cached_genres: Vec<Genre>,
    cached_image: Image,
    confirmation_sent_at: Option<PlainDateTime>,
    confirmed_at: Option<PlainDateTime>,
    created_at: Option<OffsetDateTime>,
    current_sign_in_at: Option<PlainDateTime>,
    email: Option<String>,
    email_verified: Option<OffsetDateTime>,
    flair: Option<String>,
    followed_users_count: u64,
    followers_count: u64,
    id: u64,
    image_id: u64,
    last_activity_at: Option<PlainDateTime>,
    last_sign_in_at: Option<PlainDateTime>,
    librarian_roles: Vec<String>,
    link: Option<String>,
    location: Option<String>,
    locked_at: Option<PlainDateTime>,
    membership: Option<String>,
    membership_ends_at: Option<PlainDateTime>,
    name: Option<String>,
    object_type: Option<String>,
    onboarded: bool,
    payment_system_id: Option<u64>,
    pro: bool,
    pronoun_personal: String,
    pronoun_possessive: String,
    referrer_id: Option<u64>,
    referrer_url: Option<String>,
    remember_created_at: Option<PlainDateTime>,
    reset_password_sent_at: Option<PlainDateTime>,
    sign_in_count: Option<u64>,
    status_id: AccountStatus,
    timezone: Option<String>,
    unconfirmed_email: Option<String>,
    updated_at: OffsetDateTime,
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

        Self::from_data(query, username).await
    }

    pub(crate) async fn from_user_id(user_id: u64) -> Result<Self, reqwest::Error> {
        let query = r#"
        query GetUser($user: Int!) {
          users(where: {id: {_eq: $user}}, limit: 1) {"#.to_string() + QUERY_FIELDS + r#"
          }
        }
        "#;

        Self::from_data(query, user_id).await
    }

    async fn from_data<T: ToString>(query: String, user_data: T) -> Result<Self, reqwest::Error> {
        let mut vars = HashMap::new();
        vars.insert("user", user_data.to_string());

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
            followed_users_count: {
                get_u64_from_resp(data, "followed_users_count").unwrap()
            },
            followers_count: {
                get_u64_from_resp(data, "followers_count").unwrap()
            },
            id: get_u64_from_resp(data, "id").unwrap(),
            image_id: get_u64_from_resp(data, "image_id").unwrap(),
            last_activity_at: get_plaindatetime_from_resp(data, "last_activity_at"),
            last_sign_in_at: get_plaindatetime_from_resp(data, "last_sign_in_at"),
            librarian_roles: {
                get_str_vec(data["librarian_roles"].as_array())
            },
            link: {
                get_str_from_resp(data, "link")
            },
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
            remember_created_at: {
                get_plaindatetime_from_resp(data, "remember_created_at")
            },
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
            timezone: {
                get_str_from_resp(data, "timezone")
            },
            unconfirmed_email: {
                get_str_from_resp(data, "unconfirmed_email")
            },
            updated_at: {
                get_offsetdatetime_from_resp(data, "updated_at").unwrap()
            },
            username: {
                get_str_from_resp(data, "username").unwrap()
            },
        })
    }
    
    pub(crate) fn get_id(&self) -> u64 {
        self.id
    }
}


#[derive(Debug, Serialize, Deserialize, Clone)]
struct BlockedUser {}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) enum AccountStatus {
    Created,
    Activated,
    Banned,
}