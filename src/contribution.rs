use serde::{Deserialize, Serialize};
use time::PlainDateTime;
use crate::author::Author;
use crate::book::Book;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct Contribution {
    author: Option<Author>,
    author_id: u64,
    book: Option<Book>,
    contributable_id: u64,
    contributable_type: ContributableType,
    contribution: Option<ContributionRole>,
    created_at: PlainDateTime,
    id: u64,
    updated_at: PlainDateTime,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
enum ContributableType {
    Book,
    Edition,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
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