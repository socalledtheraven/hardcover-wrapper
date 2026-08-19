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