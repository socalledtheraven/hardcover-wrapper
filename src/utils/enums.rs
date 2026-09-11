//! Enumerations and shared supporting types used across Hardcover data models.

use serde::Deserialize;

/// Privacy levels for user content such as activities, lists, or goals.
#[derive(Debug, Clone, Deserialize, Eq, PartialEq)]
#[serde(from = "u64")]
pub enum PrivacySetting {
    /// Visible to everyone.
    Public,
    /// Visible only to the user's followers.
    FollowersOnly,
    /// Visible only to the user.
    Private,
}

impl From<u64> for PrivacySetting {
    fn from(value: u64) -> Self {
        match value {
            1 => PrivacySetting::Public,
            2 => PrivacySetting::FollowersOnly,
            3 => PrivacySetting::Private,
            _ => panic!("Unknown privacy_setting_id: {value}"),
        }
    }
}

/// Gender identification for authors and users.
#[derive(Debug, Clone, Deserialize, Eq, PartialEq)]
#[serde(from = "u64")]
pub enum Gender {
    Male,
    Female,
    Nonbinary,
}

impl From<u64> for Gender {
    fn from(value: u64) -> Self {
        match value {
            1 => Gender::Male,
            2 => Gender::Female,
            3 => Gender::Nonbinary,
            _ => panic!("Unknown gender id: {}", value),
        }
    }
}

/// General state of an entity record in the Hardcover database.
#[derive(Debug, Clone, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum RecordState {
    Active,
    Duplicate,
}

/// Processing status of a book record.
#[derive(Debug, Clone, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum BookRecordState {
    Pending,
    Processing,
    Normalized,
    Processed,
    Error,
    Duplicate,
}

/// Processing and linking status of an edition record.
#[derive(Debug, Clone, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum EditionRecordState {
    Pending,
    Linking,
    Linked,
    Normalized,
    Error,
    Duplicate,
}

/// Reading progress/status for a user's book entry.
#[derive(Debug, Clone, Deserialize, Eq, PartialEq)]
#[serde(from = "u64")]
pub enum ReadingStatus {
    WantToRead,
    CurrentlyReading,
    Read,
    Paused,
    DidNotFinish,
    Ignored,
}

impl From<u64> for ReadingStatus {
    fn from(value: u64) -> Self {
        match value {
            1 => ReadingStatus::WantToRead,
            2 => ReadingStatus::CurrentlyReading,
            3 => ReadingStatus::Read,
            4 => ReadingStatus::Paused,
            5 => ReadingStatus::DidNotFinish,
            6 => ReadingStatus::Ignored,
            _ => panic!("Unknown reading status id: {}", value),
        }
    }
}

/// An external hyperlink associated with an entity.
#[derive(Debug, Clone, Deserialize, Eq, PartialEq)]
pub struct Link {
    /// The target URL.
    pub url: String,
    /// Title or label of the link.
    pub title: String,
}

/// External service identifiers (e.g. Goodreads, Audible, OpenLibrary).
#[derive(Debug, Clone, Deserialize, Eq, PartialEq)]
pub struct Identifiers {
    /// Audible ASINs or identifiers.
    pub audible: Option<Vec<String>>,
    /// Goodreads author or book IDs.
    pub goodreads: Option<Vec<String>>,
    /// OpenLibrary IDs.
    pub openlibrary: Option<Vec<String>>,
}

/// Account status of a user profile.
#[derive(Debug, Clone, Deserialize, Eq, PartialEq)]
#[serde(from = "u64")]
pub enum AccountStatus {
    Created,
    Activated,
    Banned,
}

impl From<u64> for AccountStatus {
    fn from(value: u64) -> Self {
        match value {
            1 => AccountStatus::Created,
            2 => AccountStatus::Activated,
            3 => AccountStatus::Banned,
            _ => panic!("Unknown account status id: {}", value),
        }
    }
}

/// Type of item being recommended.
#[derive(Debug, Clone, Deserialize, Eq, PartialEq)]
#[serde(from = "u64")]
pub enum RecommendationType {
    Book,
}

impl From<u64> for RecommendationType {
    fn from(value: u64) -> Self {
        match value {
            0 => RecommendationType::Book,
            _ => panic!("Unknown recommendation type id: {}", value),
        }
    }
}

/// Category/nature of a vibe entry.
#[derive(Debug, Clone, Deserialize, Eq, PartialEq)]
#[serde(from = "u64")]
pub enum VibeType {
    Custom,
    Recommendation,
    Dynamic,
    TopPicks,
}

impl From<u64> for VibeType {
    fn from(value: u64) -> Self {
        match value {
            0 => VibeType::Custom,
            1 => VibeType::Recommendation,
            2 => VibeType::Dynamic,
            3 => VibeType::TopPicks,
            _ => panic!("Unknown vibe type id: {}", value),
        }
    }
}
