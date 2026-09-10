use hardcover_wrapper::*;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let api_key = std::env::var("API_KEY").unwrap();
    let client = HardcoverClient::new(api_key);

    let me = User::from_username("prophecyreviews", &client).await?;
    println!("Me: {:#?}", me);
    assert_eq!(me.id, 52626);

    Ok(())
}
