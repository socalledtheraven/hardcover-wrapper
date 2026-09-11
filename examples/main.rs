use hardcover_rs::*;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let api_key = std::env::var("API_KEY").unwrap();
    let client = HardcoverClient::new(api_key);

    let _x = client.graphql_req(
        r#"
        query GetAllBook {
            books_by_pk(id: $id) {
                id
                title
                description
            }
        }
        "#.to_string(),
        serde_json::json!({ "id": 1 }),
    ).await?;

    Ok(())
}
