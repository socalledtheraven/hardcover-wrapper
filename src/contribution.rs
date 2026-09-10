use crate::date_parsing;
use serde::Deserialize;
use crate::base_hardcover_item::BaseHardcoverItem;
use crate::client::GraphQLResponse;
use crate::HardcoverClient;
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

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum ContributableType {
    Book,
    Edition,
}

#[derive(Debug, Clone, Deserialize)]
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

#[derive(Debug, Clone, Deserialize)]
pub struct Contribution {
    pub author_id: u64,
    pub contributable_id: u64,
    pub contributable_type: ContributableType,
    pub contribution: Option<ContributionRole>,
    pub contributor_role_id: Option<u64>,
    pub contributor_specialization_id: Option<u64>,
    #[serde(deserialize_with = "date_parsing::plain_datetime")]
    pub created_at: PlainDateTime,
    pub id: u64,
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

        let data = Self::from_data(query, id, client).await?;

        Ok(Self::new(data["contributions_by_pk"].clone()))
    }

    fn new(data: Value) -> Self {
        serde_json::from_value(data).unwrap()
    }
}
