use hardcover_wrapper::*;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    client::set_api_key(env!("API_KEY"));

    Ok(())
}
