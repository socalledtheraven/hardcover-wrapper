use serde_json::Value;
use time::PlainDateTime;
use crate::base_hardcover_item::BaseHardcoverItem;
use crate::graphql::{GraphQLResponse};

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

#[derive(Debug, Clone)]
enum ContributableType {
    Book,
    Edition,
}

#[derive(Debug, Clone)]
enum ContributionRole {
    Author,
    Illustrator,
    Translator,
    Editor,
    Narrator,
    Foreword,
    Afterword,
    CoverArtist
}

#[derive(Debug, Clone)]
pub(crate) struct Contribution {
    author_id: u64,
    contributable_id: u64,
    contributable_type: ContributableType,
    contribution: Option<ContributionRole>,
    contributor_role_id: Option<u64>,
    contributor_specialization_id: Option<u64>,
    created_at: PlainDateTime,
    id: u64,
    updated_at: PlainDateTime,
}

impl BaseHardcoverItem for Contribution {
    async fn from_id(id: u64) -> Result<Self, reqwest::Error> {
        let query = r#"
        query GetContribution($id: bigint!) {
          contributions_by_pk(id: $id) {"#.to_string() + QUERY_FIELDS + r#"
          }
        }
        "#;

        let data = Self::from_data(query, id).await?;

        Ok(Self::new(data["contributions_by_pk"].clone()))
    }

    fn new(data: Value) -> Self {
        Contribution {
            author_id: {
                data.get_u64("author_id").unwrap()
            },
            contributable_id: {
                data.get_u64("contributable_id").unwrap()
            },
            contributable_type: {
                match data["contributable_type"].as_str().unwrap() {
                    "Book" => ContributableType::Book,
                    "Edition" => ContributableType::Edition,
                    _ => panic!("Unexpected contributable type"),
                }
            },
            contribution: {
                match data["contribution"].as_str() {
                    Some("Author") => Some(ContributionRole::Author),
                    Some("Illustrator") => Some(ContributionRole::Illustrator),
                    Some("Translator") => Some(ContributionRole::Translator),
                    Some("Editor") => Some(ContributionRole::Editor),
                    Some("Narrator") => Some(ContributionRole::Narrator),
                    Some("Foreword") => Some(ContributionRole::Foreword),
                    Some("Afterword") => Some(ContributionRole::Afterword),
                    Some("CoverArtist") => Some(ContributionRole::CoverArtist),
                    _ => None,
                }
            },
            contributor_role_id: {
                data.get_u64("contributor_role_id")
            },
            contributor_specialization_id: {
                data.get_u64("contributor_specialization_id")
            },
            created_at: {
                data.get_plaindt("created_at").unwrap()
            },
            id: {
                data.get_u64("id").unwrap()
            },
            updated_at: {
                data.get_plaindt("updated_at").unwrap()
            },
        }
    }
}