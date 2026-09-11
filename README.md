# hardcover-wrapper
A Rust API wrapper for the Hardcover API, providing fully typed data. Currently in alpha, not ready for production use yet. Of particular note, there's no error handling whatsoever. If the API gives an error or malformed data, it will panic and crash. If you find any bugs with the library itself, ex. problems with the data model, please open an issue. Thanks!

# Usage
## Basic Usage
```rust
let api_key = std::env::var("API_KEY").unwrap();
let client = HardcoverClient::new(api_key);

let lotr = Book::from_id(377938, &client).await?;
println!("Lord of the Rings: {:#?}", lotr);
println!(lotr.editions_count);
```