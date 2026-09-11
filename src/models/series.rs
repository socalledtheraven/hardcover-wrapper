//! Series model representing book series, trilogies, and multi-part sagas.

use crate::utils::base_hardcover_item::BaseHardcoverItem;
use crate::utils::enums::Identifiers;
use crate::HardcoverClient;
use serde::Deserialize;
use serde_json::Value;

const QUERY_FIELDS: &str = r#"
author_id
books_count
canonical_id
description
id
identifiers
is_completed
locked
name
object_type
primary_books_count
slug
state
user_id
"#;

/// Represents a book series.
#[derive(Debug, Clone, Deserialize)]
pub struct Series {
    /// Author ID of the series creator.
    pub author_id: Option<u64>,
    /// Total number of books in the series (including novellas/spinoffs).
    pub books_count: u64,
    /// Canonical series ID if merged.
    pub canonical_id: Option<u64>,
    /// Synopsis or description of the series.
    pub description: Option<String>,
    /// Unique identifier for the series.
    pub id: u64,
    /// External service identifiers.
    pub identifiers: Identifiers,
    /// Whether the series has concluded.
    pub is_completed: Option<bool>,
    /// Whether this record is locked against community edits.
    pub locked: bool,
    /// Name / title of the series.
    pub name: String,
    /// GraphQL object type name.
    pub object_type: String,
    /// Number of main/primary books in the series.
    pub primary_books_count: Option<u64>,
    /// URL slug for the series page.
    pub slug: String,
    /// Record state.
    pub state: String,
    /// Contributor user ID who created the series entry.
    pub user_id: Option<u64>,
}

impl BaseHardcoverItem for Series {
    async fn from_id(id: u64, client: &HardcoverClient) -> Result<Self, reqwest::Error> {
        let query = r#"
        query GetSeries($id: Int!) {
          series_by_pk(id: $id) {"#
            .to_string()
            + QUERY_FIELDS
            + r#"
          }
        }
        "#;

        let variables = serde_json::json!({ "id": id });
        let data = Self::from_data(query, variables, client).await?;

        Ok(Self::from_value(data["series_by_pk"].clone()))
    }

    fn from_value(data: Value) -> Self {
        serde_json::from_value(data).unwrap()
    }
}
