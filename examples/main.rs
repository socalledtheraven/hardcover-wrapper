use hardcover_wrapper::*;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let api_key = std::env::var("API_KEY").unwrap();
    let client = HardcoverClient::new(api_key);

    let vibe = Vibe::from_id(6, &client).await.unwrap();

    println!("Vibe: {:#?}", vibe);
    assert_eq!(vibe.title, "Graphic Novels");

    Ok(())
}
