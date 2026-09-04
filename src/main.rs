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
pub mod contribution;
pub mod character;
pub mod mappings;
pub mod series;
pub mod edition;
pub mod country;
pub mod language;
pub mod publisher;
pub mod reading_format;
pub mod book_series;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    // todo: turn these into test cases

    // let me = user::User::from_username("prophecyreviews").await?;
    // println!("{me:#?}");

    // let activities = activity::Activity::of_user(me.get_id()).await?;
    // let activity = activities.get(0).unwrap();
    // let d = &activity.created_at;
    // println!("{d:#?}");

    // let author = author::Author::from_author_id(132049).await?;
    // println!("{}", author.born_year.unwrap());

    // let book = book::Book::from_book_id(484946).await?;
    // println!("{:?}", book.updated_at);

    Ok(())
}
