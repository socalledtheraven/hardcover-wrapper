use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(from = "u64")]
pub enum PrivacySetting {
    Public,
    FollowersOnly,
    Private,
}

impl From<u64> for PrivacySetting {
    fn from(value: u64) -> Self {
        match value {
            1 => PrivacySetting::Public,
            2 => PrivacySetting::FollowersOnly,
            3 => PrivacySetting::Private,
            _ => panic!("Unknown book_status_id: {value}"),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(from = "u64")]
pub enum Gender {
    Male,
    Female,
    Nonbinary,
}

impl From<u64> for Gender {
    fn from(value: u64) -> Self {
        match value {
            1 => Gender::Female,
            2 => Gender::Male,
            3 => Gender::Nonbinary,
            _ => panic!("Unknown gender id: {}", value),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecordState {
    Active,
    Duplicate,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecordState2 {
    Pending,
    Processing,
    Normalized,
    Processed,
    Error,
    Duplicate,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecordState3 {
    Pending,
    Linking,
    Linked,
    Normalized,
    Error,
    Duplicate,
}

#[derive(Debug, Clone, Deserialize)]
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

#[derive(Clone, Debug, Deserialize)]
pub struct Link {
    pub url: String,
    pub title: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Identifiers {
    pub audible: Option<Vec<String>>,
    pub goodreads: Option<Vec<String>>,
    pub openlibrary: Option<Vec<String>>,
}

#[derive(Debug, Clone, Deserialize)]
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


#[derive(Clone, Debug, Deserialize)]
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

#[derive(Clone, Debug, Deserialize)]
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

