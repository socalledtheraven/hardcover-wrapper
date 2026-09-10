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
pub enum Gender {
    Male,
    Female,
    Nonbinary,
}

#[derive(Debug, Clone, Deserialize)]
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
pub enum RecordState3 {
    Pending,
    Linking,
    Linked,
    Normalized,
    Error,
    Duplicate,
}

#[derive(Debug, Clone, Deserialize)]
pub enum ReadingStatus {
    WantToRead,
    CurrentlyReading,
    Read,
    Paused,
    DidNotFinish,
    Ignored,
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
