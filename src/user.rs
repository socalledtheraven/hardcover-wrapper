use serde_json::Value;
use time::{Date, OffsetDateTime, PlainDateTime};
use crate::base_hardcover_item::BaseHardcoverItem;
use crate::util::PrivacySetting;
use crate::genre::Genre;
use crate::graphql::{GraphQLResponse};
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

#[derive(Debug, Clone)]
pub(crate) struct User {
    access_level: Option<u64>,
    account_privacy_settings_id: PrivacySetting,
    activity_privacy_settings_id: PrivacySetting,
    admin: bool,
    bio: Option<String>,
    birthdate: Option<Date>,
    books_count: u64,
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
impl BaseHardcoverItem for User {
    async fn from_id(id: u64) -> Result<Self, reqwest::Error> {
        let query = r#"
        query GetUser($user: Int!) {
          users(where: {id: {_eq: $user}}, limit: 1) {"#.to_string() + QUERY_FIELDS + r#"
          }
        }
        "#;

        let data = Self::from_data(query, id).await?;

        Ok(Self::new(data["users"][0].clone()))
    }

    fn new(data: Value) -> Self {
        User {
            access_level: {
                data.get_u64("access_level")
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
                data.get_privacysetting("activity_privacy_settings_id")
            },
            admin: {
                // should always be present
                data.get_bool("admin").unwrap()
            },
            bio: {
                data.get_str("bio")
            },
            birthdate: {
                data.get_date("birthdate")
            },
            books_count: {
                data.get_u64("books_count").unwrap()
            },
            confirmation_sent_at: {
                data.get_plaindt("confirmation_sent_at")
            },
            confirmed_at: {
                data.get_plaindt("confirmed_at")
            },
            created_at: {
                data.get_offsetdt("created_at")
            },
            current_sign_in_at: {
                data.get_plaindt("current_sign_in_at")
            },
            email: {
                data.get_str("email")
            },
            email_verified: {
                data.get_offsetdt("email_verified")
            },
            flair: {
                data.get_str("flair")
            },
            followed_users_count: {
                data.get_u64("followed_users_count").unwrap()
            },
            followers_count: {
                data.get_u64("followers_count").unwrap()
            },
            id: data.get_u64("id").unwrap(),
            image_id: data.get_u64("image_id").unwrap(),
            last_activity_at: data.get_plaindt("last_activity_at"),
            last_sign_in_at: data.get_plaindt("last_sign_in_at"),
            librarian_roles: {
                data.get_str_vec("librarian_roles")
            },
            link: {
                data.get_str("link")
            },
            location: {
                data.get_str("location")
            },
            locked_at: {
                data.get_plaindt("locked_at")
            },
            membership: {
                data.get_str("membership")
            },
            membership_ends_at: {
                data.get_plaindt("membership_ends_at")
            },
            name: {
                data.get_str("name")
            },
            object_type: {
                data.get_str("object_type")
            },
            onboarded: {
                data.get_bool("onboarded").unwrap()
            },
            payment_system_id: {
                data.get_u64("payment_system_id")
            },
            pro: {
                data.get_bool("pro").unwrap()
            },
            pronoun_personal: {
                data.get_str("pronoun_personal").unwrap()
            },
            pronoun_possessive: {
                data.get_str("pronoun_possessive").unwrap()
            },
            referrer_id: {
                data.get_u64("referrer_id")
            },
            referrer_url: {
                data.get_str("referrer_url")
            },
            remember_created_at: {
                data.get_plaindt("remember_created_at")
            },
            reset_password_sent_at: {
                data.get_plaindt("reset_password_sent_at")
            },
            sign_in_count: {
                data.get_u64("sign_in_count")
            },
            status_id: {
                // we default to activated if the status is unknown, this will never happen, but better to fail open
                data.get_u64("status_id")
                    .map(|v| match v {
                        1 => AccountStatus::Created,
                        2 => AccountStatus::Activated,
                        3 => AccountStatus::Banned,
                        _ => AccountStatus::Activated,
                    })
                    .unwrap_or(AccountStatus::Activated)
            },
            timezone: {
                data.get_str("timezone")
            },
            unconfirmed_email: {
                data.get_str("unconfirmed_email")
            },
            updated_at: {
                data.get_offsetdt("updated_at").unwrap()
            },
            username: {
                data.get_str("username").unwrap()
            },
        }
    }
}

impl User {
    pub(crate) async fn from_username(username: &str) -> Result<Self, reqwest::Error> {
        let query = r#"
        query GetUser($id: citext!) {
          users(where: {username: {_eq: $id}}, limit: 1) {"#.to_string() + QUERY_FIELDS + r#"
          }
        }
        "#;

        let data = Self::from_data(query, username).await?;

        Ok(Self::new(data["users"][0].clone()))
    }
}


#[derive(Debug, Clone)]
struct BlockedUser {}


#[derive(Debug, Clone)]
pub(crate) enum AccountStatus {
    Created,
    Activated,
    Banned,
}