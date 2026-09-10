use hardcover_wrapper::*;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let api_key = std::env::var("API_KEY").unwrap();
    let client = HardcoverClient::new(api_key);

    let author = Author::from_id(132049, &client).await.unwrap();
    println!("Author: {:#?}", author);
    assert_eq!(author.born_year, Some(1892));

    Ok(())
}
