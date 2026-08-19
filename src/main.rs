pub mod graphql;
pub mod user;
pub mod activity;
pub mod author;
pub mod enums;
pub mod genre;
pub mod tagging;
pub mod user_flag;
pub mod user_book;
pub mod prompt;
pub mod like;
pub mod book;
pub mod import;
pub mod list;
pub mod follow;
pub mod goal;
pub mod image;
pub mod link;
pub mod notification;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let me = user::User::from_username("prophecyreviews").await?;

    println!("{me:#?}");

    Ok(())
}
