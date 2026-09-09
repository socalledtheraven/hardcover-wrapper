#[cfg(test)]
mod tests {
    use hardcover_wrapper::*;
    use time::macros::datetime;
    use tokio::time::{sleep, Duration};

    static CLIENT: std::sync::OnceLock<HardcoverClient> = std::sync::OnceLock::new();

    fn initialise() {
        CLIENT.get_or_init(|| {
            let api_key = std::env::var("API_KEY").unwrap();
            HardcoverClient::new(api_key)
        });
    }

    #[tokio::test]
    async fn test_me() {
        initialise();
        sleep(Duration::from_millis(500)).await;

        let client = CLIENT.get().unwrap();

        let me = User::from_username("prophecyreviews", client)
            .await
            .unwrap();
        println!("Me: {:#?}", me);
        assert_eq!(me.id, 52626);
    }

    #[tokio::test]
    async fn test_activity() {
        initialise();
        sleep(Duration::from_millis(500)).await;

        let client = CLIENT.get().unwrap();

        let activity = Activity::from_id(53892, client).await.unwrap();
        println!("Activity: {:#?}", activity);
        assert_eq!(
            activity.created_at,
            Some(datetime!(2023-10-27 22:25:36.829681 +00:00:00))
        );
    }

    #[tokio::test]
    async fn test_author() {
        initialise();
        sleep(Duration::from_millis(500)).await;

        let client = CLIENT.get().unwrap();

        let author = Author::from_id(132049, client).await.unwrap();
        println!("Author: {:#?}", author);
        assert_eq!(author.born_year, Some(1892))
    }

    #[tokio::test]
    async fn test_book() {
        initialise();
        sleep(Duration::from_millis(500)).await;

        let client = CLIENT.get().unwrap();

        let book = Book::from_id(484946, client).await.unwrap();
        println!("Book: {:#?}", book);
        assert_eq!(book.title, Some("The Bright Sword".to_string()))
    }

    #[tokio::test]
    async fn test_character() {
        initialise();
        sleep(Duration::from_millis(500)).await;

        let client = CLIENT.get().unwrap();

        let char = Character::from_id(2135, client).await.unwrap();

        println!("Character: {:#?}", char);
        assert_eq!(char.name, "Arlen Weston")
    }

    #[tokio::test]
    async fn test_contribution() {
        initialise();
        sleep(Duration::from_millis(500)).await;

        let client = CLIENT.get().unwrap();

        let contribution = Contribution::from_id(1, client).await.unwrap();

        println!("Contribution: {:#?}", contribution);
        assert_eq!(
            contribution.contributable_type,
            contribution::ContributableType::Book
        )
    }

    #[tokio::test]
    async fn test_edition() {
        initialise();
        sleep(Duration::from_millis(500)).await;

        let client = CLIENT.get().unwrap();

        let edition = Edition::from_id(31529525, client).await.unwrap();

        println!("Edition: {:#?}", edition);
        assert_eq!(edition.pages, Some(288))
    }

    #[tokio::test]
    async fn test_goal() {
        initialise();
        sleep(Duration::from_millis(500)).await;

        let client = CLIENT.get().unwrap();

        let goal = Goal::from_id(16, client).await.unwrap();

        println!("Goal: {:#?}", goal);
        assert_eq!(goal.description, Some("Read 10 books in 2022".to_string()))
    }

    #[tokio::test]
    async fn test_like() {
        initialise();
        sleep(Duration::from_millis(500)).await;

        let client = CLIENT.get().unwrap();

        let like = Like::from_id(1, client).await.unwrap();

        println!("Like: {:#?}", like);
        assert_eq!(like.likeable_type, "Activity")
    }

    #[tokio::test]
    async fn test_list() {
        initialise();
        sleep(Duration::from_millis(500)).await;

        let client = CLIENT.get().unwrap();

        let list = List::from_id(11325, client).await.unwrap();

        println!("List: {:#?}", list);
        assert_eq!(list.slug, Some("owned".to_string()))
    }

    #[tokio::test]
    async fn test_notification() {
        initialise();
        sleep(Duration::from_millis(500)).await;

        let client = CLIENT.get().unwrap();

        let notification = Notification::from_id(440096, client).await.unwrap();

        println!("Notification: {:#?}", notification);
        assert_eq!(
            notification.link,
            Some("/books/of-monsters-and-mainframes".to_string())
        )
    }

    #[tokio::test]
    async fn test_prompt() {
        initialise();
        sleep(Duration::from_millis(500)).await;

        let client = CLIENT.get().unwrap();

        let prompt = Prompt::from_id(122, client).await.unwrap();

        println!("Prompt: {:#?}", prompt);
        assert_eq!(
            prompt.question,
            "What biographical stories of growth inspired you?"
        )
    }

    #[tokio::test]
    async fn test_publisher() {
        initialise();
        sleep(Duration::from_millis(500)).await;

        let client = CLIENT.get().unwrap();

        let publisher = Publisher::from_id(8, client).await.unwrap();

        println!("Publisher: {:#?}", publisher);
        assert_eq!(publisher.name, Some("Penguin Viking".to_string()))
    }

    #[tokio::test]
    async fn test_reading_journal() {
        initialise();
        sleep(Duration::from_millis(500)).await;

        let client = CLIENT.get().unwrap();

        let reading_journal = ReadingJournal::from_id(15497756, client).await.unwrap();

        println!("Reading Journal: {:#?}", reading_journal);
        assert_eq!(reading_journal.book_id, Some(427374))
    }

    #[tokio::test]
    async fn test_series() {
        initialise();
        sleep(Duration::from_millis(500)).await;

        let client = CLIENT.get().unwrap();

        let series = Series::from_id(147942, client).await.unwrap();

        println!("Series: {:#?}", series);
        assert_eq!(series.books_count, 8)
    }

    #[tokio::test]
    async fn test_tag() {
        initialise();
        sleep(Duration::from_millis(500)).await;

        let client = CLIENT.get().unwrap();

        let tag = Tag::from_id(12, client).await.unwrap();

        println!("Tag: {:#?}", tag);
        assert_eq!(tag.tag, "General")
    }

    #[tokio::test]
    async fn test_user_book() {
        initialise();
        sleep(Duration::from_millis(500)).await;

        let client = CLIENT.get().unwrap();

        let user_book = UserBook::from_id(452432, client).await.unwrap();

        println!("User Book: {:#?}", user_book);
        assert_eq!(user_book.edition_id, Some(29963190))
    }

    #[tokio::test]
    async fn test_vibe() {
        initialise();
        sleep(Duration::from_millis(500)).await;

        let client = CLIENT.get().unwrap();

        let vibe = Vibe::from_id(6, client).await.unwrap();

        println!("Vibe: {:#?}", vibe);
        assert_eq!(vibe.title, "Graphic Novels")
    }

    // #[tokio::test]
    // async fn test_custom_graphql() {
    //     initialise();
    //
    //     let query = r#"
    //         query GetUserByUsername($username: String!) {
    //             user(username: $username) {
    //                 id
    //                 username
    //                 name
    //             }
    //         }
    //     "#;
    //
    //     // let resp = client::graphql_req(
    //     //     query.to_string(),
    //     //
    //     // , client).await.unwrap();
    //     //
    //     // println!("{resp:#?}");
    // }
}
