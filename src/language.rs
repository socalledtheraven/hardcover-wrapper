use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct Language {
    code2: Option<String>,
    code3: Option<String>,
    id: u32,
    language: String,
}