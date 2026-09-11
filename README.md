# hardcover-rs

A strongly typed Rust client library for the [Hardcover](https://hardcover.app/) GraphQL API.

## Features

- **Natural Interface**: Abstracts away querying, authentication, and response parsing.
- **Typed Data Models:** Rich Rust structs and enums matching Hardcover's schema (Books, Editions, Authors, Users, etc).
- **Asynchronous API:** Built with Tokio and Reqwest for async network requests.

## Usage

### Basic Example

```rust
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
```

## Documentation

Generate and view documentation locally:

```bash
cargo doc --open
```

## License

Licensed under LGPLv2.