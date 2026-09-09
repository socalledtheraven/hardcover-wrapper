#[cfg(test)]
mod tests {
    use time::macros::datetime;
    use hardcover_wrapper::*;

    static CLIENT: std::sync::OnceLock<HardcoverClient> = std::sync::OnceLock::new();

    fn initialise() {
        CLIENT.get_or_init(|| {
            let api_key = std::env::var("HARDCOVER_API_KEY").unwrap();
            HardcoverClient::new(api_key)
        });
    }

    #[tokio::test]
    async fn test_me() {
        initialise();

        let client = CLIENT.get().unwrap();

        let me = User::from_username("prophecyreviews", client).await.unwrap();
        assert_eq!(
            me.id, 52626
        );
    }

    #[tokio::test]
    async fn test_activity() {
        initialise();

        let client = CLIENT.get().unwrap();

        let activity = Activity::from_id(53892, client).await.unwrap();
        println!("{0:#?}", activity.created_at);
        assert_eq!(
            activity.created_at,
            Some(datetime!(2023-10-27 22:25:36.829681 +00:00:00))
        );
    }

    #[tokio::test]
    async fn test_author() {
        initialise();

        let client = CLIENT.get().unwrap();

        let author = Author::from_id(132049, client).await.unwrap();
        assert_eq!(
            author.born_year,
            Some(1892)
        )
    }

    #[tokio::test]
    async fn test_book() {
        initialise();

        let client = CLIENT.get().unwrap();

        let book = Book::from_id(484946, client).await.unwrap();
        assert_eq!(
            book.title,
            Some("The Bright Sword".to_string())
        )
    }

    #[tokio::test]
    async fn test_character() {
        initialise();

        let client = CLIENT.get().unwrap();

        let char = Character::from_id(2135, client).await.unwrap();

        assert_eq!(
            char.name,
            "Arlen Weston"
        )
    }

    #[tokio::test]
    async fn test_contribution() {
        initialise();

        let client = CLIENT.get().unwrap();

        let contribution = Contribution::from_id(1, client).await.unwrap();

        assert_eq!(
            contribution.contributable_type,
            contribution::ContributableType::Book
        )
    }

    #[tokio::test]
    async fn test_edition() {
        initialise();

        let client = CLIENT.get().unwrap();

        let edition = Edition::from_id(31529525, client).await.unwrap();

        assert_eq!(
            edition.pages,
            Some(288)
        )
    }

    #[tokio::test]
    async fn test_goal() {
        initialise();

        let client = CLIENT.get().unwrap();

        let goal = Goal::from_id(16, client).await.unwrap();

        assert_eq!(
            goal.description,
            Some("Read 10 books in 2022".to_string())
        )
    }

    #[tokio::test]
    async fn test_like() {
        initialise();

        let client = CLIENT.get().unwrap();

        let like = Like::from_id(1, client).await.unwrap();

        assert_eq!(
            like.likeable_type,
            "Activity"
        )
    }

    #[tokio::test]
    async fn test_list() {
        initialise();

        let client = CLIENT.get().unwrap();

        let list = List::from_id(11325, client).await.unwrap();

        assert_eq!(
            list.slug,
            Some("owned".to_string())
        )
    }

    #[tokio::test]
    async fn test_notification() {
        initialise();

        let client = CLIENT.get().unwrap();

        let notification = Notification::from_id(440096, client).await.unwrap();

        assert_eq!(
            notification.link,
            Some("/books/of-monsters-and-mainframes".to_string())
        )
    }

    #[tokio::test]
    async fn test_prompt() {
        initialise();

        let client = CLIENT.get().unwrap();

        let prompt = Prompt::from_id(122, client).await.unwrap();

        assert_eq!(
            prompt.question,
            "What biographical stories of growth inspired you?"
        )
    }

    #[tokio::test]
    async fn test_publisher() {
        initialise();

        let client = CLIENT.get().unwrap();

        let publisher = Publisher::from_id(8, client).await.unwrap();

        assert_eq!(
            publisher.name,
            Some("Penguin Viking".to_string())
        )
    }

    #[tokio::test]
    async fn test_reading_journal() {
        initialise();

        let client = CLIENT.get().unwrap();

        let reading_journal = ReadingJournal::from_id(15497756, client).await.unwrap();

        assert_eq!(
            reading_journal.book_id,
            Some(427374)
        )
    }

    #[tokio::test]
    async fn test_series() {
        initialise();

        let client = CLIENT.get().unwrap();

        let series = Series::from_id(147942, client).await.unwrap();

        assert_eq!(
            series.books_count,
            8
        )
    }

    #[tokio::test]
    async fn test_tag() {
        initialise();

        let client = CLIENT.get().unwrap();

        let tag = Tag::from_id(12, client).await.unwrap();

        assert_eq!(
            tag.tag,
            "General"
        )
    }

    #[tokio::test]
    async fn test_user_book() {
        initialise();

        let client = CLIENT.get().unwrap();

        let user_book = UserBook::from_id(452432, client).await.unwrap();

        assert_eq!(
            user_book.edition_id,
            Some(29963190)
        )
    }

    #[tokio::test]
    async fn test_vibe() {
        initialise();

        let client = CLIENT.get().unwrap();

        let vibe = Vibe::from_id(6, client).await.unwrap();

        assert_eq!(
            vibe.title,
            "Graphic Novels"
        )
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