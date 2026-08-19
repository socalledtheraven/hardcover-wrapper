use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct Prompt {}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct PromptAnswer {}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct PromptSummary {}