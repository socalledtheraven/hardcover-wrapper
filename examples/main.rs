use time::macros::datetime;
use hardcover_wrapper::*;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let api_key = std::env::var("API_KEY").unwrap();
    let client = HardcoverClient::new(api_key);

    let activity = Activity::from_id(53892, &client).await.unwrap();
    println!("Activity: {:#?}", activity);
    assert_eq!(
        activity.created_at,
        Some(datetime!(2023-10-27 22:25:36.829681 +00:00:00))
    );

    Ok(())
}
