use hardcover_wrapper::*;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let api_key = std::env::var("API_KEY").unwrap();
    let client = HardcoverClient::new(api_key);

    let book = Book::from_id(484946, &client).await.unwrap();
    println!("Book: {:#?}", book);
    assert_eq!(book.title, Some("The Bright Sword".to_string()));
    
    Ok(())
}
