use hardcover_wrapper::*;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    set_api_key(env!("API_KEY"));

    Ok(())
}
