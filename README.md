# hardcover-wrapper

A strongly typed Rust client library for the [Hardcover](https://hardcover.app/) GraphQL API.

## Features

- **Typed Data Models:** Rich Rust structs and enums matching Hardcover's schema (Books, Editions, Authors, Users, Reading Journals, Series, and more).
- **Asynchronous API:** Built with Tokio and Reqwest for async network requests.
- **Convenient Re-exports:** Top-level access to common models and utility clients.

## Usage

### Basic Example

```rust
use hardcover_wrapper::{HardcoverClient, Book, BaseHardcoverItem};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = std::env::var("API_KEY").expect("API_KEY must be set");
    let client = HardcoverClient::new(api_key);

    let lotr = Book::from_id(377938, &client).await?;
    println!("Lord of the Rings: {:#?}", lotr);
    println!("Editions: {}", lotr.editions_count);

    Ok(())
}
```

## Documentation

Generate and view documentation locally:

```bash
cargo doc --open
```

## License

Licensed under the MIT License.