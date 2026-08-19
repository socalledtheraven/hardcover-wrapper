use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct ReadingFormat {
    id: u32,
    // todo!
    format: String,
}