//! Custom Serde deserializers for parsing date and time strings returned by the Hardcover API.

use serde::{Deserialize, Deserializer};
use time::format_description::well_known::Rfc3339;
use time::macros::format_description;
use time::{Date, OffsetDateTime, PlainDateTime};

const DATE_FORMAT: &[time::format_description::FormatItem<'_>] =
    format_description!("[year]-[month]-[day]");

const PLAIN_DATE_TIME_FORMAT: &[time::format_description::FormatItem<'_>] =
    format_description!("[year]-[month]-[day]T[hour]:[minute]:[second].[subsecond]");

/// Deserializes a required date formatted as `YYYY-MM-DD`.
pub fn date<'de, D>(deserializer: D) -> Result<Date, D::Error>
where
    D: Deserializer<'de>,
{
    let value = String::deserialize(deserializer)?;

    Date::parse(&value, DATE_FORMAT).map_err(serde::de::Error::custom)
}

/// Deserializes an optional date formatted as `YYYY-MM-DD`.
pub fn date_optional<'de, D>(deserializer: D) -> Result<Option<Date>, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Option::<String>::deserialize(deserializer)?;

    match value {
        Some(value) => Date::parse(&value, DATE_FORMAT)
            .map(Some)
            .map_err(serde::de::Error::custom),
        None => Ok(None),
    }
}

/// Deserializes a required date-time without timezone offset (`YYYY-MM-DDTHH:MM:SS.sss`).
pub fn plain_datetime<'de, D>(deserializer: D) -> Result<PlainDateTime, D::Error>
where
    D: Deserializer<'de>,
{
    let value = String::deserialize(deserializer)?;

    PlainDateTime::parse(&value, PLAIN_DATE_TIME_FORMAT).map_err(serde::de::Error::custom)
}

/// Deserializes an optional date-time without timezone offset (`YYYY-MM-DDTHH:MM:SS.sss`).
pub fn plain_datetime_optional<'de, D>(deserializer: D) -> Result<Option<PlainDateTime>, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Option::<String>::deserialize(deserializer)?;

    match value {
        Some(value) => PlainDateTime::parse(&value, PLAIN_DATE_TIME_FORMAT)
            .map(Some)
            .map_err(serde::de::Error::custom),
        None => Ok(None),
    }
}

/// Deserializes a required RFC 3339 formatted offset date-time.
pub fn offset_datetime<'de, D>(deserializer: D) -> Result<OffsetDateTime, D::Error>
where
    D: Deserializer<'de>,
{
    let value = String::deserialize(deserializer)?;

    OffsetDateTime::parse(&value, &Rfc3339).map_err(serde::de::Error::custom)
}

/// Deserializes an optional RFC 3339 formatted offset date-time.
pub fn offset_datetime_optional<'de, D>(deserializer: D) -> Result<Option<OffsetDateTime>, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Option::<String>::deserialize(deserializer)?;

    match value {
        Some(value) => OffsetDateTime::parse(&value, &Rfc3339)
            .map(Some)
            .map_err(serde::de::Error::custom),
        None => Ok(None),
    }
}
