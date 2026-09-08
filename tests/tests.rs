#[cfg(test)]
mod tests {
    use time::macros::datetime;
    use tokio::time::{sleep, Duration};
    use hardcover_wrapper::*;

    async fn test_setup() {
        sleep(Duration::from_secs(2)).await;
        set_api_key(env!("API_KEY"));
    }

    #[tokio::test]
    async fn test_me() {
        test_setup().await;

        let me = User::from_username("prophecyreviews").await.unwrap();
        assert_eq!(
            me.id, 52626
        );
    }

    #[tokio::test]
    async fn test_activity() {
        test_setup().await;

        let activity = Activity::from_id(53892).await.unwrap();
        println!("{0:#?}", activity.created_at);
        assert_eq!(
            activity.created_at,
            Some(datetime!(2023-10-27 22:25:36.829681 +00:00:00))
        );
    }

    #[tokio::test]
    async fn test_author() {
        test_setup().await;
        
        let author = Author::from_id(132049).await.unwrap();
        assert_eq!(
            author.born_year,
            Some(1892)
        )
    }

    #[tokio::test]
    async fn test_book() {
        test_setup().await;

        let book = Book::from_id(484946).await.unwrap();
        assert_eq!(
            book.title,
            Some("The Bright Sword".to_string())
        )
    }

    #[tokio::test]
    async fn test_character() {
        test_setup().await;

        let char = Character::from_id(2135).await.unwrap();

        assert_eq!(
            char.name,
            "Arlen Weston"
        )
    }

    #[tokio::test]
    async fn test_contribution() {
        test_setup().await;
        
        let contribution = Contribution::from_id(1).await.unwrap();

        assert_eq!(
            contribution.contributable_type,
            contribution::ContributableType::Book
        )
    }

    #[tokio::test]
    async fn test_edition() {
        test_setup().await;
        
        let edition = Edition::from_id(31529525).await.unwrap();

        assert_eq!(
            edition.pages,
            Some(288)
        )
    }

    #[tokio::test]
    async fn test_goal() {
        test_setup().await;

        let goal = Goal::from_id(16).await.unwrap();

        assert_eq!(
            goal.description,
            Some("Read 10 books in 2022".to_string())
        )
    }

    #[tokio::test]
    async fn test_like() {
        test_setup().await;

        let like = Like::from_id(1).await.unwrap();

        assert_eq!(
            like.likeable_type,
            "Activity"
        )
    }

    #[tokio::test]
    async fn test_list() {
        test_setup().await;

        let list = List::from_id(11325).await.unwrap();

        assert_eq!(
            list.slug,
            Some("owned".to_string())
        )
    }

    #[tokio::test]
    async fn test_notification() {
        test_setup().await;

        let notification = Notification::from_id(440096).await.unwrap();

        assert_eq!(
            notification.link,
            Some("/books/of-monsters-and-mainframes".to_string())
        )
    }

    #[tokio::test]
    async fn test_prompt() {
        test_setup().await;

        let prompt = Prompt::from_id(122).await.unwrap();

        assert_eq!(
            prompt.question,
            "What biographical stories of growth inspired you?"
        )
    }

    #[tokio::test]
    async fn test_publisher() {
        test_setup().await;

        let publisher = Publisher::from_id(8).await.unwrap();

        assert_eq!(
            publisher.name,
            Some("Penguin Viking".to_string())
        )
    }

    #[tokio::test]
    async fn test_reading_journal() {
        test_setup().await;

        let reading_journal = ReadingJournal::from_id(15497756).await.unwrap();

        assert_eq!(
            reading_journal.book_id,
            Some(427374)
        )
    }

    #[tokio::test]
    async fn test_series() {
        test_setup().await;

        let series = Series::from_id(147942).await.unwrap();

        assert_eq!(
            series.books_count,
            8
        )
    }

    #[tokio::test]
    async fn test_tag() {
        test_setup().await;

        let tag = Tag::from_id(12).await.unwrap();

        assert_eq!(
            tag.tag,
            "General"
        )
    }

    #[tokio::test]
    async fn test_user_book() {
        test_setup().await;

        let user_book = UserBook::from_id(452432).await.unwrap();

        assert_eq!(
            user_book.edition_id,
            Some(29963190)
        )
    }

    #[tokio::test]
    async fn test_vibe() {
        test_setup().await;

        let vibe = Vibe::from_id(6).await.unwrap();

        assert_eq!(
            vibe.title,
            "Graphic Novels"
        )
    }

    // #[tokio::test]
    // async fn test_custom_graphql() {
    //     test_setup().await;
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
    //     // ).await.unwrap();
    //     //
    //     // println!("{resp:#?}");
    // }
}