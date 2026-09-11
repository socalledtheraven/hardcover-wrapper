#[cfg(test)]
mod tests {
    use hardcover_wrapper::models::activity::ActivityType;
    use hardcover_wrapper::models::contribution;
    use hardcover_wrapper::models::contribution::{ContributableType, ContributionRole};
    use hardcover_wrapper::utils::enums::Gender;
    use hardcover_wrapper::*;
    use serde_json::Value;
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
    #[ignore = "Test on real API"]
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
    #[ignore = "Test on real API"]
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
    #[ignore = "Test on real API"]
    async fn test_author() {
        initialise();
        sleep(Duration::from_millis(500)).await;

        let client = CLIENT.get().unwrap();

        let author = Author::from_id(132049, client).await.unwrap();
        println!("Author: {:#?}", author);
        assert_eq!(author.born_year, Some(1892))
    }

    #[tokio::test]
    #[ignore = "Test on real API"]
    async fn test_book() {
        initialise();
        sleep(Duration::from_millis(500)).await;

        let client = CLIENT.get().unwrap();

        let book = Book::from_id(484946, client).await.unwrap();
        println!("Book: {:#?}", book);
        assert_eq!(book.title, Some("The Bright Sword".to_string()))
    }

    #[tokio::test]
    #[ignore = "Test on real API"]
    async fn test_character() {
        initialise();
        sleep(Duration::from_millis(500)).await;

        let client = CLIENT.get().unwrap();

        let char = Character::from_id(2135, client).await.unwrap();

        println!("Character: {:#?}", char);
        assert_eq!(char.name, "Arlen Weston")
    }

    #[tokio::test]
    #[ignore = "Test on real API"]
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
    #[ignore = "Test on real API"]
    async fn test_edition() {
        initialise();
        sleep(Duration::from_millis(500)).await;

        let client = CLIENT.get().unwrap();

        let edition = Edition::from_id(31529525, client).await.unwrap();

        println!("Edition: {:#?}", edition);
        assert_eq!(edition.pages, Some(288))
    }

    #[tokio::test]
    #[ignore = "Test on real API"]
    async fn test_goal() {
        initialise();
        sleep(Duration::from_millis(500)).await;

        let client = CLIENT.get().unwrap();

        let goal = Goal::from_id(16, client).await.unwrap();

        println!("Goal: {:#?}", goal);
        assert_eq!(goal.description, Some("Read 10 books in 2022".to_string()))
    }

    #[tokio::test]
    #[ignore = "Test on real API"]
    async fn test_like() {
        initialise();
        sleep(Duration::from_millis(500)).await;

        let client = CLIENT.get().unwrap();

        let like = Like::from_id(1, client).await.unwrap();

        println!("Like: {:#?}", like);
        assert_eq!(like.likeable_type, "Activity")
    }

    #[tokio::test]
    #[ignore = "Test on real API"]
    async fn test_list() {
        initialise();
        sleep(Duration::from_millis(500)).await;

        let client = CLIENT.get().unwrap();

        let list = List::from_id(11325, client).await.unwrap();

        println!("List: {:#?}", list);
        assert_eq!(list.slug, Some("owned".to_string()))
    }

    #[tokio::test]
    #[ignore = "Test on real API"]
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
    #[ignore = "Test on real API"]
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
    #[ignore = "Test on real API"]
    async fn test_publisher() {
        initialise();
        sleep(Duration::from_millis(500)).await;

        let client = CLIENT.get().unwrap();

        let publisher = Publisher::from_id(8, client).await.unwrap();

        println!("Publisher: {:#?}", publisher);
        assert_eq!(publisher.name, Some("Penguin Viking".to_string()))
    }

    #[tokio::test]
    #[ignore = "Test on real API"]
    async fn test_reading_journal() {
        initialise();
        sleep(Duration::from_millis(500)).await;

        let client = CLIENT.get().unwrap();

        let reading_journal = ReadingJournal::from_id(15497756, client).await.unwrap();

        println!("Reading Journal: {:#?}", reading_journal);
        assert_eq!(reading_journal.book_id, Some(427374))
    }

    #[tokio::test]
    #[ignore = "Test on real API"]
    async fn test_series() {
        initialise();
        sleep(Duration::from_millis(500)).await;

        let client = CLIENT.get().unwrap();

        let series = Series::from_id(147942, client).await.unwrap();

        println!("Series: {:#?}", series);
        assert_eq!(series.books_count, 8)
    }

    #[tokio::test]
    #[ignore = "Test on real API"]
    async fn test_tag() {
        initialise();
        sleep(Duration::from_millis(500)).await;

        let client = CLIENT.get().unwrap();

        let tag = Tag::from_id(12, client).await.unwrap();

        println!("Tag: {:#?}", tag);
        assert_eq!(tag.tag, "General")
    }

    #[tokio::test]
    #[ignore = "Test on real API"]
    async fn test_user_book() {
        initialise();
        sleep(Duration::from_millis(500)).await;

        let client = CLIENT.get().unwrap();

        let user_book = UserBook::from_id(452432, client).await.unwrap();

        println!("User Book: {:#?}", user_book);
        assert_eq!(user_book.edition_id, Some(29963190))
    }

    #[tokio::test]
    #[ignore = "Test on real API"]
    async fn test_vibe() {
        initialise();
        sleep(Duration::from_millis(500)).await;

        let client = CLIENT.get().unwrap();

        let vibe = Vibe::from_id(6, client).await.unwrap();

        println!("Vibe: {:#?}", vibe);
        assert_eq!(vibe.title, "Graphic Novels")
    }

    #[test]
    fn test_parse_activity_53892() {
        let raw = include_str!("fixtures/activity_53892.json");
        let json: Value = serde_json::from_str(raw).expect("valid fixture JSON");
        let activity = Activity::from_value(json["data"]["activities_by_pk"].clone());

        assert_eq!(activity.id, 53892);
        assert_eq!(activity.likes_count, 0);
        assert_eq!(activity.event, ActivityType::UserBookActivity);
    }

    #[test]
    fn test_parse_author_132049() {
        let raw = include_str!("fixtures/author_132049.json");
        let json: Value = serde_json::from_str(raw).expect("valid fixture JSON");
        let author = Author::from_value(json["data"]["authors_by_pk"].clone());

        assert_eq!(author.id, 132049);
        assert_eq!(author.books_count, 277);
        assert_eq!(author.gender, Some(Gender::Male));
        assert_eq!(author.death_year, Some(1973));
    }

    #[test]
    fn test_parse_book_484946() {
        let raw = include_str!("fixtures/book_484946.json");
        let json: Value = serde_json::from_str(raw).expect("valid fixture JSON");
        let book = Book::from_value(json["data"]["books_by_pk"].clone());

        assert_eq!(book.id, 484946);
        assert_eq!(book.title.as_deref(), Some("The Bright Sword"));
        assert_eq!(book.pages, Some(683));
    }

    #[test]
    fn test_parse_book_53892() {
        let raw = include_str!("fixtures/book_53892.json");
        let json: Value = serde_json::from_str(raw).expect("valid fixture JSON");
        let book = Book::from_value(json["data"]["books_by_pk"].clone());

        assert_eq!(book.id, 53892);
        assert_eq!(book.title.as_deref(), Some("Agatha Christie: The Finished Portrait"));
        assert_eq!(book.release_year, Some(2006));
    }

    #[test]
    fn test_parse_character_2135() {
        let raw = include_str!("fixtures/character_2135.json");
        let json: Value = serde_json::from_str(raw).expect("valid fixture JSON");
        let char = Character::from_value(json["data"]["characters_by_pk"].clone());

        assert_eq!(char.id, 2135);
        assert_eq!(char.books_count, 2);
        assert_eq!(char.gender, None);
        assert_eq!(char.name, "Arlen Weston");
    }

    #[test]
    fn test_parse_contribution_1() {
        let raw = include_str!("fixtures/contribution_1.json");
        let json: Value = serde_json::from_str(raw).expect("valid fixture JSON");
        let contribution = Contribution::from_value(json["data"]["contributions_by_pk"].clone());

        assert_eq!(contribution.id, 1);
        assert_eq!(contribution.contributable_type, ContributableType::Book);
        assert_eq!(contribution.author_id, 154428);
        assert_eq!(contribution.contribution, Some(ContributionRole::Author));
    }

    #[test]
    fn test_parse_edition_31529525() {
        let raw = include_str!("fixtures/edition_31529525.json");
        let json: Value = serde_json::from_str(raw).expect("valid fixture JSON");
        let edition = Edition::from_value(json["data"]["editions_by_pk"].clone());

        assert_eq!(edition.id, 31529525);
        assert_eq!(edition.book_id, 1481438);
        assert_eq!(
            edition.title.as_deref(),
            Some("The Paradox Of Wealth And Poverty: Mapping The Ethical Dilemmas Of Global Development")
        );
        assert_eq!(edition.pages, Some(288));
        assert_eq!(edition.isbn_10.as_deref(), Some("0813316421"));
        assert_eq!(edition.isbn_13.as_deref(), Some("9780813316420"));
    }

    #[test]
    fn test_parse_goal_16() {
        let raw = include_str!("fixtures/goal_16.json");
        let json: Value = serde_json::from_str(raw).expect("valid fixture JSON");
        let goal = Goal::from_value(json["data"]["goals_by_pk"].clone());

        assert_eq!(goal.id, 16);
        assert_eq!(goal.user_id, 65);
        assert_eq!(goal.goal, 10);
        assert_eq!(goal.metric, "book");
        assert_eq!(goal.description.as_deref(), Some("Read 10 books in 2022"));
        assert_eq!(goal.archived, false);
    }

    #[test]
    fn test_parse_like_1() {
        let raw = include_str!("fixtures/like_1.json");
        let json: Value = serde_json::from_str(raw).expect("valid fixture JSON");
        let like = Like::from_value(json["data"]["likes_by_pk"].clone());

        assert_eq!(like.id, 1);
        assert_eq!(like.user_id, 3);
        assert_eq!(like.likeable_id, 1);
        assert_eq!(like.likeable_type, "Activity");
    }

    #[test]
    fn test_parse_list_11325() {
        let raw = include_str!("fixtures/list_11325.json");
        let json: Value = serde_json::from_str(raw).expect("valid fixture JSON");
        let list = List::from_value(json["data"]["lists_by_pk"].clone());

        assert_eq!(list.id, 11325);
        assert_eq!(list.user_id, 2173);
        assert_eq!(list.name, "Owned");
        assert_eq!(list.slug.as_deref(), Some("owned"));
        assert_eq!(list.books_count, 0);
        assert_eq!(list.public, true);
    }

    #[test]
    fn test_parse_notification_440096() {
        let raw = include_str!("fixtures/notification_440096.json");
        let json: Value = serde_json::from_str(raw).expect("valid fixture JSON");
        let notification = Notification::from_value(json["data"]["notifications_by_pk"].clone());

        assert_eq!(notification.id, 440096);
        assert_eq!(notification.notifier_user_id, 52626);
        assert_eq!(notification.notification_type_id, 9);
        assert_eq!(
            notification.link.as_deref(),
            Some("/books/of-monsters-and-mainframes")
        );
        assert_eq!(
            notification.title,
            "We completed your feedback about 'Of Monsters and Mainframes'"
        );
    }

    #[test]
    fn test_parse_prompt_122() {
        let raw = include_str!("fixtures/prompt_122.json");
        let json: Value = serde_json::from_str(raw).expect("valid fixture JSON");
        let prompt = Prompt::from_value(json["data"]["prompts_by_pk"].clone());

        assert_eq!(prompt.id, 122);
        assert_eq!(prompt.user_id, 1);
        assert_eq!(
            prompt.question,
            "What biographical stories of growth inspired you?"
        );
        assert_eq!(
            prompt.slug,
            "what-biographical-stories-of-growth-inspired-you"
        );
        assert_eq!(prompt.books_count, 9);
    }

    #[test]
    fn test_parse_publisher_8() {
        let raw = include_str!("fixtures/publisher_8.json");
        let json: Value = serde_json::from_str(raw).expect("valid fixture JSON");
        let publisher = Publisher::from_value(json["data"]["publishers_by_pk"].clone());

        assert_eq!(publisher.id, 8);
        assert_eq!(publisher.name.as_deref(), Some("Penguin Viking"));
        assert_eq!(publisher.slug, "penguin-viking");
        assert_eq!(publisher.editions_count, 14);
        assert_eq!(publisher.state, "active");
    }

    #[test]
    fn test_parse_reading_journal_15497756() {
        let raw = include_str!("fixtures/reading_journal_15497756.json");
        let json: Value = serde_json::from_str(raw).expect("valid fixture JSON");
        let reading_journal =
            ReadingJournal::from_value(json["data"]["reading_journals_by_pk"].clone());

        assert_eq!(reading_journal.id, 15497756);
        assert_eq!(reading_journal.user_id, Some(31792));
        assert_eq!(reading_journal.book_id, Some(427374));
        assert_eq!(reading_journal.edition_id, Some(7441206));
        assert_eq!(reading_journal.event.as_deref(), Some("rated"));
    }

    #[test]
    fn test_parse_series_147942() {
        let raw = include_str!("fixtures/series_147942.json");
        let json: Value = serde_json::from_str(raw).expect("valid fixture JSON");
        let series = Series::from_value(json["data"]["series_by_pk"].clone());

        assert_eq!(series.id, 147942);
        assert_eq!(series.author_id, Some(678724));
        assert_eq!(series.name, "Savage Awakening");
        assert_eq!(series.slug, "savage-awakening");
        assert_eq!(series.books_count, 8);
        assert_eq!(series.primary_books_count, Some(8));
    }

    #[test]
    fn test_parse_tag_12() {
        let raw = include_str!("fixtures/tag_12.json");
        let json: Value = serde_json::from_str(raw).expect("valid fixture JSON");
        let tag = Tag::from_value(json["data"]["tags_by_pk"].clone());

        assert_eq!(tag.id, 12);
        assert_eq!(tag.tag, "General");
        assert_eq!(tag.slug, "general");
        assert_eq!(tag.count, 27602);
        assert_eq!(tag.tag_category_id, 1);
    }

    #[test]
    fn test_parse_user_book_452432() {
        let raw = include_str!("fixtures/user_book_452432.json");
        let json: Value = serde_json::from_str(raw).expect("valid fixture JSON");
        let user_book = UserBook::from_value(json["data"]["user_books_by_pk"].clone());

        assert_eq!(user_book.id, 452432);
        assert_eq!(user_book.user_id, 2032);
        assert_eq!(user_book.book_id, 242651);
        assert_eq!(user_book.edition_id, Some(29963190));
        assert_eq!(user_book.imported, Some(true));
        assert_eq!(user_book.owned, false);
    }

    #[test]
    fn test_parse_vibe_6() {
        let raw = include_str!("fixtures/vibe_6.json");
        let json: Value = serde_json::from_str(raw).expect("valid fixture JSON");
        let vibe = Vibe::from_value(json["data"]["vibes_by_pk"].clone());

        assert_eq!(vibe.id, 6);
        assert_eq!(vibe.user_id, 15471);
        assert_eq!(vibe.title, "Graphic Novels");
        assert_eq!(vibe.slug, "graphic-novels");
        assert_eq!(vibe.description.as_deref(), Some("Testing out vibes."));
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
