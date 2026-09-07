
#[derive(Debug, Clone)]
pub(crate) enum PrivacySetting {
    Public,
    FollowersOnly,
    Private,
}

#[derive(Debug, Clone)]
pub(crate) enum Gender {
    Male,
    Female,
    Nonbinary
}

#[derive(Debug, Clone)]
pub(crate) enum RecordState {
    Active,
    Duplicate
}

#[derive(Debug, Clone)]
pub(crate) enum RecordState2 {
    Pending,
    Processing,
    Normalized,
    Processed,
    Error,
    Duplicate
}

#[derive(Debug, Clone)]
pub(crate) enum RecordState3 {
    Pending,
    Linking,
    Linked,
    Normalized,
    Error,
    Duplicate
}

#[derive(Debug, Clone)]
pub(crate) enum ReadingStatus { 
    WantToRead,
    CurrentlyReading,
    Read,
    Paused,
    DidNotFinish,
    Ignored,
}

#[derive(Clone, Debug)]
pub(crate) struct Link {
    pub(crate) url: String,
    pub(crate) title: String,
}

#[derive(Clone, Debug)]
pub(crate) struct Identifiers {
    pub(crate) audible: Option<Vec<String>>,
    pub(crate) goodreads: Option<Vec<String>>,
    pub(crate) openlibrary: Option<Vec<String>>,
}