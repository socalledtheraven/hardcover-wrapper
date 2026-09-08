#[derive(Debug, Clone)]
pub enum PrivacySetting {
    Public,
    FollowersOnly,
    Private,
}

#[derive(Debug, Clone)]
pub enum Gender {
    Male,
    Female,
    Nonbinary
}

#[derive(Debug, Clone)]
pub enum RecordState {
    Active,
    Duplicate
}

#[derive(Debug, Clone)]
pub enum RecordState2 {
    Pending,
    Processing,
    Normalized,
    Processed,
    Error,
    Duplicate
}

#[derive(Debug, Clone)]
pub enum RecordState3 {
    Pending,
    Linking,
    Linked,
    Normalized,
    Error,
    Duplicate
}

#[derive(Debug, Clone)]
pub enum ReadingStatus {
    WantToRead,
    CurrentlyReading,
    Read,
    Paused,
    DidNotFinish,
    Ignored,
}

#[derive(Clone, Debug)]
pub struct Link {
    pub url: String,
    pub title: String,
}

#[derive(Clone, Debug)]
pub struct Identifiers {
    pub audible: Option<Vec<String>>,
    pub goodreads: Option<Vec<String>>,
    pub openlibrary: Option<Vec<String>>,
}