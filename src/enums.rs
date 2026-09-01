use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) enum PrivacySetting {
    Public,
    FollowersOnly,
    Private,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) enum Gender {
    Male,
    Female,
    Nonbinary
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) enum RecordState {
    Active,
    Duplicate
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) enum RecordState2 {
    Pending,
    Processing,
    Normalized,
    Processed,
    Error,
    Duplicate
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) enum RecordState3 {
    Pending,
    Linking,
    Linked,
    Normalized,
    Error,
    Duplicate
}