use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) enum PrivacySetting {
    Public,
    FollowersOnly,
    Private,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) enum AccountStatus {
    Created,
    Activated,
    Banned,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) enum MaybeInit<T> {
    Uninitialised,
    Initialised(T),
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