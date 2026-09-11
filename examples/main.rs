use hardcover_rs::{HardcoverClient, Book, BaseHardcoverItem};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = std::env::var("API_KEY").expect("API_KEY must be set");
    let client = HardcoverClient::new(api_key);

    let lotr = Book::from_id(377938, &client).await?;
    println!("Lord of the Rings: {:#?}", lotr);
    println!("Editions: {}", lotr.editions_count);

    Ok(())
}