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
pub mod list;
pub mod goal;
pub mod image;
pub mod notification;
pub mod contribution;
pub mod character;
pub mod series;
pub mod edition;
pub mod country;
pub mod language;
pub mod publisher;
pub mod reading_format;
pub mod book_series;
pub mod platform;
pub mod reading_journal;

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

    // let char = character::Character::from_character_id(2135).await?;
    // print!("{char:#?}");

    // let contribution = contribution::Contribution::from_contribution_id(1).await?;
    // println!("{contribution:#?}");

    // let edition = edition::Edition::from_edition_id(31529525).await?;
    // println!("{edition:#?}");

    // let goal = goal::Goal::from_goal_id(16).await?;
    // println!("{goal:#?}");

    // let like = like::Like::from_like_id(1).await?;
    // println!("{like:#?}");

    // let list = list::List::from_list_id(11325).await?;
    // println!("{list:#?}");

    // let notification = notification::Notification::from_notification_id(440096).await?;
    // println!("{notification:#?}");

    // let prompt = prompt::Prompt::from_prompt_id(122).await?;
    // println!("{prompt:#?}");

    // let publisher = publisher::Publisher::from_publisher_id(8).await?;
    // println!("{publisher:#?}");

    let reading_journal = reading_journal::ReadingJournal::from_reading_journal_id(15497756).await?;
    println!("{reading_journal:#?}");

    Ok(())
}
