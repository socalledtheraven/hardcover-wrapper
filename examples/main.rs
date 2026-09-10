use hardcover_wrapper::*;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let api_key = std::env::var("API_KEY").unwrap();
    let client = HardcoverClient::new(api_key);

    let contribution = Contribution::from_id(1, &client).await.unwrap();

    println!("Contribution: {:#?}", contribution);
    assert_eq!(
        contribution.contributable_type,
        contribution::ContributableType::Book
    );

    Ok(())
}
