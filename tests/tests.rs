#[cfg(test)]
mod tests {
    use hardcover_wrapper::*;

    #[tokio::test]
    async fn test_me() {
        client::set_api_key(env!("API_KEY"));

        let me = User::from_username("prophecyreviews").await.unwrap();
        println!("{me:#?}");
    }

    #[tokio::test]
    async fn test_activity() {
        client::set_api_key(env!("API_KEY"));

        let activity = Activity::from_id(53892).await.unwrap();
        println!("{0:#?}", activity.created_at);
    }

    #[tokio::test]
    async fn test_author() {
        client::set_api_key(env!("API_KEY"));
        
        let author = Author::from_id(132049).await.unwrap();
        println!("{}", author.born_year.unwrap());
    }

    #[tokio::test]
    async fn test_book() {
        client::set_api_key(env!("API_KEY"));

        let book = Book::from_id(484946).await.unwrap();
        println!("{:?}", book.updated_at);
    }

    #[tokio::test]
    async fn test_character() {
        client::set_api_key(env!("API_KEY"));

        let char = Character::from_id(2135).await.unwrap();
        println!("{char:#?}");
    }

    #[tokio::test]
    async fn test_contribution() {
        client::set_api_key(env!("API_KEY"));
        
        let contribution = Contribution::from_id(1).await.unwrap();
        println!("{contribution:#?}");
    }

    #[tokio::test]
    async fn test_edition() {
        client::set_api_key(env!("API_KEY"));
        
        let edition = Edition::from_id(31529525).await.unwrap();
        println!("{edition:#?}");
    }

    #[tokio::test]
    async fn test_goal() {
        client::set_api_key(env!("API_KEY"));

        let goal = Goal::from_id(16).await.unwrap();
        println!("{goal:#?}");
    }

    #[tokio::test]
    async fn test_like() {
        client::set_api_key(env!("API_KEY"));

        let like = Like::from_id(1).await.unwrap();
        println!("{like:#?}");
    }

    #[tokio::test]
    async fn test_list() {
        client::set_api_key(env!("API_KEY"));

        let list = List::from_id(11325).await.unwrap();
        println!("{list:#?}");
    }

    #[tokio::test]
    async fn test_notification() {
        client::set_api_key(env!("API_KEY"));

        let notification = Notification::from_id(440096).await.unwrap();
        println!("{notification:#?}");
    }

    #[tokio::test]
    async fn test_prompt() {
        client::set_api_key(env!("API_KEY"));

        let prompt = Prompt::from_id(122).await.unwrap();
        println!("{prompt:#?}");
    }

    #[tokio::test]
    async fn test_publisher() {
        client::set_api_key(env!("API_KEY"));

        let publisher = Publisher::from_id(8).await.unwrap();
        println!("{publisher:#?}");
    }

    #[tokio::test]
    async fn test_reading_journal() {
        client::set_api_key(env!("API_KEY"));

        let reading_journal = ReadingJournal::from_id(15497756).await.unwrap();
        println!("{reading_journal:#?}");
    }

    #[tokio::test]
    async fn test_series() {
        client::set_api_key(env!("API_KEY"));

        let series = Series::from_id(147942).await.unwrap();
        println!("{series:#?}");
    }

    #[tokio::test]
    async fn test_tag() {
        client::set_api_key(env!("API_KEY"));

        let tag = Tag::from_id(12).await.unwrap();
        println!("{tag:#?}");
    }

    #[tokio::test]
    async fn test_user_book() {
        client::set_api_key(env!("API_KEY"));

        let user_book = UserBook::from_id(452432).await.unwrap();
        println!("{user_book:#?}");
    }

    #[tokio::test]
    async fn test_vibe() {
        client::set_api_key(env!("API_KEY"));

        let vibe = Vibe::from_id(6).await.unwrap();
        println!("{vibe:#?}");
    }
}