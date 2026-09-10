use crate::date_parsing;
use serde::Deserialize;
use crate::base_hardcover_item::BaseHardcoverItem;
use crate::util::{AccountStatus, PrivacySetting};
use crate::HardcoverClient;
use serde_json::Value;
use time::{Date, OffsetDateTime, PlainDateTime};

const QUERY_FIELDS: &str = r"
            access_level
            account_privacy_setting_id
            activity_privacy_settings_id
            admin
            bio
            birthdate
            books_count
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

#[derive(Debug, Clone, Deserialize)]
pub struct User {
    pub access_level: Option<u64>,
    #[serde(rename = "account_privacy_setting_id")]
    pub account_privacy_settings_id: PrivacySetting,
    pub activity_privacy_settings_id: PrivacySetting,
    pub admin: bool,
    pub bio: Option<String>,
    #[serde(deserialize_with = "date_parsing::date_optional")]
    pub birthdate: Option<Date>,
    pub books_count: u64,
    #[serde(deserialize_with = "date_parsing::plain_datetime_optional")]
    pub confirmation_sent_at: Option<PlainDateTime>,
    #[serde(deserialize_with = "date_parsing::plain_datetime_optional")]
    pub confirmed_at: Option<PlainDateTime>,
    #[serde(deserialize_with = "date_parsing::offset_datetime_optional")]
    pub created_at: Option<OffsetDateTime>,
    #[serde(deserialize_with = "date_parsing::plain_datetime_optional")]
    pub current_sign_in_at: Option<PlainDateTime>,
    pub email: Option<String>,
    #[serde(deserialize_with = "date_parsing::offset_datetime_optional")]
    pub email_verified: Option<OffsetDateTime>,
    pub flair: Option<String>,
    pub followed_users_count: u64,
    pub followers_count: u64,
    pub id: u64,
    pub image_id: u64,
    #[serde(deserialize_with = "date_parsing::plain_datetime_optional")]
    pub last_activity_at: Option<PlainDateTime>,
    #[serde(deserialize_with = "date_parsing::plain_datetime_optional")]
    pub last_sign_in_at: Option<PlainDateTime>,
    pub librarian_roles: Vec<String>,
    pub link: Option<String>,
    pub location: Option<String>,
    #[serde(deserialize_with = "date_parsing::plain_datetime_optional")]
    pub locked_at: Option<PlainDateTime>,
    pub membership: Option<String>,
    #[serde(deserialize_with = "date_parsing::plain_datetime_optional")]
    pub membership_ends_at: Option<PlainDateTime>,
    pub name: Option<String>,
    pub object_type: Option<String>,
    pub onboarded: bool,
    pub payment_system_id: Option<u64>,
    pub pro: bool,
    pub pronoun_personal: String,
    pub pronoun_possessive: String,
    pub referrer_id: Option<u64>,
    pub referrer_url: Option<String>,
    #[serde(deserialize_with = "date_parsing::plain_datetime_optional")]
    pub remember_created_at: Option<PlainDateTime>,
    #[serde(deserialize_with = "date_parsing::plain_datetime_optional")]
    pub reset_password_sent_at: Option<PlainDateTime>,
    pub sign_in_count: Option<u64>,
    pub status_id: AccountStatus,
    pub timezone: Option<String>,
    pub unconfirmed_email: Option<String>,
    #[serde(deserialize_with = "date_parsing::offset_datetime")]
    pub updated_at: OffsetDateTime,
    pub username: String,
}
impl BaseHardcoverItem for User {
    async fn from_id(id: u64, client: &HardcoverClient) -> Result<Self, reqwest::Error> {
        let query = r#"
        query GetUser($user: Int!) {
          users(where: {id: {_eq: $user}}, limit: 1) {"#
            .to_string()
            + QUERY_FIELDS
            + r#"
          }
        }
        "#;

        let data = Self::from_data(query, id, client).await?;

        Ok(Self::new(data["users"][0].clone()))
    }

    fn new(data: Value) -> Self {
        serde_json::from_value(data).unwrap()
    }
}

impl User {
    pub async fn from_username(
        username: &str,
        client: &HardcoverClient,
    ) -> Result<Self, reqwest::Error> {
        let query = r#"
        query GetUser($id: citext!) {
          users(where: {username: {_eq: $id}}, limit: 1) {"#
            .to_string()
            + QUERY_FIELDS
            + r#"
          }
        }
        "#;

        let data = Self::from_data(query, username, client).await?;

        Ok(Self::new(data["users"][0].clone()))
    }
}
