use hardcover_wrapper::*;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let api_key = std::env::var("API_KEY").unwrap();
    let client = HardcoverClient::new(api_key);

    let char = Character::from_id(2135, &client).await.unwrap();

    println!("Character: {:#?}", char);
    assert_eq!(char.name, "Arlen Weston");

    Ok(())
}
