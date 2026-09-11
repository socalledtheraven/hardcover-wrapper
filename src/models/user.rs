//! User model representing Hardcover user accounts and profiles.

use crate::utils::base_hardcover_item::BaseHardcoverItem;
use crate::utils::date_parsing;
use crate::utils::enums::{AccountStatus, PrivacySetting};
use crate::HardcoverClient;
use serde::Deserialize;
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

/// Represents a registered user account and profile on Hardcover.
#[derive(Debug, Clone, Deserialize)]
pub struct User {
    /// Account access tier / level.
    pub access_level: Option<u64>,
    /// Privacy setting for the user's general account profile.
    #[serde(rename = "account_privacy_setting_id")]
    pub account_privacy_settings_id: PrivacySetting,
    /// Privacy setting for the user's activity feed.
    pub activity_privacy_settings_id: PrivacySetting,
    /// Whether the user has site administrator privileges.
    pub admin: bool,
    /// User profile biography / description.
    pub bio: Option<String>,
    /// Date of birth.
    #[serde(deserialize_with = "date_parsing::date_optional")]
    pub birthdate: Option<Date>,
    /// Number of books in the user's library.
    pub books_count: u64,
    /// Timestamp when account confirmation email was sent.
    #[serde(deserialize_with = "date_parsing::plain_datetime_optional")]
    pub confirmation_sent_at: Option<PlainDateTime>,
    /// Timestamp when account was confirmed.
    #[serde(deserialize_with = "date_parsing::plain_datetime_optional")]
    pub confirmed_at: Option<PlainDateTime>,
    /// Account creation timestamp.
    #[serde(deserialize_with = "date_parsing::offset_datetime_optional")]
    pub created_at: Option<OffsetDateTime>,
    /// Timestamp of current sign-in session.
    #[serde(deserialize_with = "date_parsing::plain_datetime_optional")]
    pub current_sign_in_at: Option<PlainDateTime>,
    /// Email address of the user.
    pub email: Option<String>,
    /// Timestamp when email was verified.
    #[serde(deserialize_with = "date_parsing::offset_datetime_optional")]
    pub email_verified: Option<OffsetDateTime>,
    /// User flair or profile badge text.
    pub flair: Option<String>,
    /// Number of users this user is following.
    pub followed_users_count: u64,
    /// Number of followers this user has.
    pub followers_count: u64,
    /// Unique identifier for the user.
    pub id: u64,
    /// Avatar image ID.
    pub image_id: u64,
    /// Timestamp of most recent activity.
    #[serde(deserialize_with = "date_parsing::plain_datetime_optional")]
    pub last_activity_at: Option<PlainDateTime>,
    /// Timestamp of previous sign-in session.
    #[serde(deserialize_with = "date_parsing::plain_datetime_optional")]
    pub last_sign_in_at: Option<PlainDateTime>,
    /// Librarian and moderation roles granted to the user.
    pub librarian_roles: Vec<String>,
    /// External web link from user profile.
    pub link: Option<String>,
    /// Geographic location of the user.
    pub location: Option<String>,
    /// Timestamp when account was locked, if applicable.
    #[serde(deserialize_with = "date_parsing::plain_datetime_optional")]
    pub locked_at: Option<PlainDateTime>,
    /// Membership plan description.
    pub membership: Option<String>,
    /// Expiration timestamp for the active membership.
    #[serde(deserialize_with = "date_parsing::plain_datetime_optional")]
    pub membership_ends_at: Option<PlainDateTime>,
    /// Display name of the user.
    pub name: Option<String>,
    /// GraphQL object type name.
    pub object_type: Option<String>,
    /// Whether user onboarding is completed.
    pub onboarded: bool,
    /// Connected payment system ID.
    pub payment_system_id: Option<u64>,
    /// Whether the user has Hardcover Pro membership.
    pub pro: bool,
    /// Personal pronoun (e.g. "she", "he", "they").
    pub pronoun_personal: String,
    /// Possessive pronoun (e.g. "her", "his", "their").
    pub pronoun_possessive: String,
    /// ID of the user who referred this account.
    pub referrer_id: Option<u64>,
    /// Referral URL used during registration.
    pub referrer_url: Option<String>,
    /// Timestamp for remember-me session creation.
    #[serde(deserialize_with = "date_parsing::plain_datetime_optional")]
    pub remember_created_at: Option<PlainDateTime>,
    /// Timestamp when password reset email was dispatched.
    #[serde(deserialize_with = "date_parsing::plain_datetime_optional")]
    pub reset_password_sent_at: Option<PlainDateTime>,
    /// Total sign-in count.
    pub sign_in_count: Option<u64>,
    /// Account activation status.
    #[serde(rename = "status_id")]
    pub status: AccountStatus,
    /// User timezone string.
    pub timezone: Option<String>,
    /// Pending unconfirmed email address.
    pub unconfirmed_email: Option<String>,
    /// Last update timestamp.
    #[serde(deserialize_with = "date_parsing::offset_datetime")]
    pub updated_at: OffsetDateTime,
    /// Unique username handle.
    pub username: String,
}

impl BaseHardcoverItem for User {
    async fn from_id(id: u64, client: &HardcoverClient) -> Result<Self, reqwest::Error> {
        let query = r#"
        query GetUser($user: Int!) {
          users_from_pk(id: $user) {"#
            .to_string()
            + QUERY_FIELDS
            + r#"
          }
        }
        "#;

        let variables = serde_json::json!({ "id": id });
        let data = Self::from_data(query, variables, client).await?;

        Ok(Self::from_value(data["users_from_pk"].clone()))
    }

    fn from_value(data: Value) -> Self {
        serde_json::from_value(data).unwrap()
    }
}

impl User {
    /// Fetches a user profile by their case-insensitive username handle.
    ///
    /// # Arguments
    ///
    /// * `username` - The unique username handle of the user.
    /// * `client` - The API client to execute the query.
    pub async fn from_username(
        username: &str,
        client: &HardcoverClient,
    ) -> Result<Self, reqwest::Error> {
        let query = r#"
        query GetUser($username: citext!) {
          users(where: {username: {_eq: $username}}, limit: 1) {"#
            .to_string()
            + QUERY_FIELDS
            + r#"
          }
        }
        "#;

        let variables = serde_json::json!({ "username": username });
        let data = Self::from_data(query, variables, client).await?;

        Ok(Self::from_value(data["users"][0].clone()))
    }
}
