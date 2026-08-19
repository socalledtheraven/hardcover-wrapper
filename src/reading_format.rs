use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct ReadingFormat {
    id: u64,
    // todo!
    format: String,
}