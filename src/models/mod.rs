//! Domain models representing entities returned by the Hardcover GraphQL API.
//!
//! Each submodule corresponds to an entity type (such as [`book::Book`], [`author::Author`], [`user::User`]),
//! and key types are re-exported at this module's root for convenience.

pub mod activity;
pub mod author;
pub mod book;
pub mod book_series;
pub mod character;
pub mod contribution;
pub mod country;
pub mod edition;
pub mod genre;
pub mod goal;
pub mod image;
pub mod language;
pub mod like;
pub mod list;
pub mod notification;
pub mod platform;
pub mod prompt;
pub mod publisher;
pub mod reading_format;
pub mod reading_journal;
pub mod series;
pub mod tag;
pub mod user;
pub mod user_book;
pub mod vibe;

pub use activity::*;
pub use author::*;
pub use book::*;
pub use book_series::*;
pub use character::*;
pub use contribution::*;
pub use country::*;
pub use edition::{Edition, EditionFormat, ReadingFormat as EditionReadingFormat};
pub use genre::*;
pub use goal::*;
pub use image::*;
pub use language::*;
pub use like::*;
pub use list::*;
pub use notification::*;
pub use platform::*;
pub use prompt::*;
pub use publisher::*;
pub use reading_format::*;
pub use reading_journal::*;
pub use series::*;
pub use tag::*;
pub use user::*;
pub use user_book::*;
pub use vibe::*;