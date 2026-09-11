//! # hardcover-rs
//!
//! A strongly typed Rust client library for the [Hardcover](https://hardcover.app/) API.
//!
//! `hardcover-rs` provides structured data models and an asynchronous client for querying
//! Hardcover's GraphQL API, mapping resources like Books, Authors, Editions, Users, Lists, and more.
//!
//! ## Quickstart
//!
//! ```no_run
//! use hardcover_rs::{HardcoverClient, Book, BaseHardcoverItem};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let api_key = std::env::var("API_KEY").expect("API_KEY must be set");
//!     let client = HardcoverClient::new(api_key);
//!
//!     // Fetch a book by its Hardcover ID
//!     let book = Book::from_id(484946, &client).await?;
//!     println!("Title: {:?}", book.title);
//!     println!("Editions count: {}", book.editions_count);
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Modules
//!
//! - [`models`]: Data models representing Hardcover domain entities (e.g. [`Book`], [`Author`], [`Edition`], [`User`]).
//! - [`utils`]: Core client ([`HardcoverClient`]), common traits ([`BaseHardcoverItem`]), enums, and deserialization helpers.

pub mod models;
pub mod utils;

pub use utils::base_hardcover_item::BaseHardcoverItem;
pub use utils::client::HardcoverClient;
pub use utils::enums::*;
pub use models::*;
