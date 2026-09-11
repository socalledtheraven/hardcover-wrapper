//! Contribution model linking authors to books or editions with specific contributor roles.

use crate::utils::base_hardcover_item::BaseHardcoverItem;
use crate::utils::date_parsing;
use crate::HardcoverClient;
use serde::Deserialize;
use serde_json::Value;
use time::PlainDateTime;

const QUERY_FIELDS: &str = r#"
author_id
contributable_id
contributable_type
contribution
contributor_role_id
contributor_specialization_id
created_at
id
updated_at
"#;

/// Target entity type to which a contribution applies.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum ContributableType {
    /// Contribution is at the book level.
    Book,
    /// Contribution is at the specific edition level.
    Edition,
}

/// Specific role of a contributor on a work or edition.
#[derive(Debug, Clone, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum ContributionRole {
    Author,
    Illustrator,
    Translator,
    Editor,
    Narrator,
    Foreword,
    Afterword,
    CoverArtist,
}

/// Represents an author's contribution to a book or edition.
#[derive(Debug, Clone, Deserialize)]
pub struct Contribution {
    /// ID of the contributing author.
    pub author_id: u64,
    /// ID of the contributable entity (Book or Edition ID).
    pub contributable_id: u64,
    /// Type of the contributable entity.
    pub contributable_type: ContributableType,
    /// Named role played by the contributor.
    pub contribution: Option<ContributionRole>,
    /// Numeric contributor role ID.
    pub contributor_role_id: Option<u64>,
    /// Numeric contributor specialization ID.
    pub contributor_specialization_id: Option<u64>,
    /// Creation timestamp.
    #[serde(deserialize_with = "date_parsing::plain_datetime")]
    pub created_at: PlainDateTime,
    /// Unique identifier for this contribution record.
    pub id: u64,
    /// Last update timestamp.
    #[serde(deserialize_with = "date_parsing::plain_datetime")]
    pub updated_at: PlainDateTime,
}

impl BaseHardcoverItem for Contribution {
    async fn from_id(id: u64, client: &HardcoverClient) -> Result<Self, reqwest::Error> {
        let query = r#"
        query GetContribution($id: bigint!) {
          contributions_by_pk(id: $id) {"#
            .to_string()
            + QUERY_FIELDS
            + r#"
          }
        }
        "#;

        let variables = serde_json::json!({ "id": id });
        let data = Self::from_data(query, variables, client).await?;

        Ok(Self::from_value(data["contributions_by_pk"].clone()))
    }

    fn from_value(data: Value) -> Self {
        serde_json::from_value(data).unwrap()
    }
}
