use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct ReadingFormat {
    pub id: u64,
    // todo!
    pub format: String,
}
