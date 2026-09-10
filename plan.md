## Alpha-release review: non-error-handling issues and improvements

Here are the main issues I’d address before an alpha release, excluding the known lack of proper error handling.

---


## 8. Tests are integration tests against the live API

The tests:

- Require `API_KEY` at compile time via `env!("API_KEY")`.
- Hit the real Hardcover API.
- Sleep for two seconds before every test.
- Depend on live data remaining unchanged.
- Depend on network availability.
- Are slow.
- Can be flaky.
- May fail under parallel test execution.
- May be unsuitable for CI.

### Specific issue

```rust
client::set_api_key(env!("API_KEY"));
```


`env!("API_KEY")` requires the environment variable at compile time, not runtime. This can surprise contributors and CI users.

Prefer:

```rust
std::env::var("API_KEY")
```


Then skip tests if it is missing.

### Better testing split

Use two categories:

1. Unit tests for parsing JSON into models.
2. Ignored/live integration tests for real API calls.

For live tests:

```rust
#[tokio::test]
#[ignore = "requires Hardcover API key and network access"]
async fn test_book_live() {
    // ...
}
```


---

## 9. Models are manually parsed from `serde_json::Value`

All model constructors accept `serde_json::Value` and manually extract fields. This works, but it gives up many of Rust/Serde’s benefits.

Current pattern:

```rust
data.get_u64("books_count").unwrap()
```


Better long-term direction:

```rust
#[derive(Debug, Clone, Deserialize)]
pub struct Author {
    pub books_count: u64,
    // ...
}
```


Then deserialize directly from the GraphQL response.

Benefits:

- Less boilerplate.
- Better schema validation.
- Easier tests.
- Easier nested types.
- More idiomatic Rust API bindings.
- Better error reporting later.

You may still need custom deserializers for enums, IDs, dates, and nullable fields, but overall it would reduce a lot of repetitive code.

---

## 10. Many enum types are incomplete or too strict

Examples:

```rust
pub enum BookCategory {
    Book,
    Novella,
    ShortStory,
    GraphicNovel,
    FanFiction,
    ResearchPaper,
    Poetry,
    Collection,
    WebNovel,
    LightNovel
}
```


```rust
pub enum ContributionRole {
    Author,
    Illustrator,
    Translator,
    Editor,
    Narrator,
    Foreword,
    Afterword,
    CoverArtist
}
```


Hardcoding current API values is useful, but APIs evolve. For alpha, consider whether unknown variants should be preserved.

### Possible design

```rust
pub enum ContributionRole {
    Author,
    Illustrator,
    Translator,
    Editor,
    Narrator,
    Foreword,
    Afterword,
    CoverArtist,
    Unknown(String),
}
```


For numeric enums:

```rust
pub enum BookCategory {
    Book,
    Novella,
    ShortStory,
    GraphicNovel,
    FanFiction,
    ResearchPaper,
    Poetry,
    Collection,
    WebNovel,
    LightNovel,
    Unknown(u64),
}
```


This prevents new Hardcover values from breaking users.

---

## 11. Some enum names expose database column details

Several fields have names like:

```rust
pub book_category_id: BookCategory,
pub book_status_id: BookStatus,
pub literary_type_id: Option<LiteraryType>,
pub reading_format_id: ReadingFormat,
pub status_id: AccountStatus,
```


The Rust type is no longer an ID; it is an enum.

More ergonomic names would be:

```rust
pub book_category: BookCategory,
pub book_status: BookStatus,
pub literary_type: Option<LiteraryType>,
pub reading_format: ReadingFormat,
pub status: AccountStatus,
```


You can still query `book_category_id` internally, but the public Rust API should ideally feel domain-oriented rather than database-oriented.

---

## 12. Some model fields expose API cache/internal fields

Many structs include fields like:

```rust
cached_image
cached_cover
cached_tags
cached_contributors
cached_featured_series
cached_header_image
```


Some are queried but not exposed. Others may be omitted from structs. Before alpha, decide whether these are intentionally excluded or whether the queried fields and struct fields should match.

If cache fields are not part of the supported public API, avoid querying them. If they are useful, expose them.

Currently there are many queried fields that are not represented in structs, for example in `Book`:

```rust
cached_contributors
cached_featured_series
cached_header_image
cached_image
cached_similar_book_ids
cached_similar_books_updated_at
cached_tags
```


This increases response size without benefiting users.

---

## 13. GraphQL queries are built with string concatenation

Example:

```rust
let query = r#"
query GetBook($id: Int!) {
  books_by_pk(id: $id) {"#.to_string() + QUERY_FIELDS + r#"
  }
}
"#;
```


This works, but it is brittle and hard to format. Consider using `format!`:

```rust
let query = format!(
    r#"
    query GetBook($id: Int!) {{
      books_by_pk(id: $id) {{
        {QUERY_FIELDS}
      }}
    }}
    "#
);
```


Or centralize query construction.

Potential benefits:

- Less awkward string composition.
- Easier to add tests that snapshot queries.
- Easier to validate field lists.

---

## 14. Inconsistent GraphQL ID scalar usage

Some queries use `Int!`:

```rust
query GetBook($id: Int!)
```


One uses `bigint!`:

```rust
query GetContribution($id: bigint!)
```


The Rust API uses `u64` everywhere. GraphQL `Int` is typically a signed 32-bit integer, while some Hardcover IDs may exceed that range. You already have an edition test using:

```rust
Edition::from_id(31529525)
```


That fits 32-bit, but if Hardcover uses `bigint` for many IDs, queries should consistently use the API’s correct scalar type.

This is worth auditing across every `from_id`.

---

## 15. `BaseHardcoverItem::from_data` is too generic but still too narrow

Current signature:

```rust
async fn from_data<T: ToString>(
    query: String,
    user_data: T,
) -> Result<Value, reqwest::Error>
```


Issues:

- It assumes the variable name is always `"id"`.
- It always converts the variable to a `String`.
- It cannot support multiple variables.
- It cannot support non-string/non-ID inputs cleanly.
- It couples all resource constructors to one GraphQL variable shape.

For example, `User::from_username` works because it names the GraphQL variable `$id` even though it is actually a username:

```rust
query GetUser($id: citext!) {
  users(where: {username: {_eq: $id}}, limit: 1) {
```


This is confusing.

### Better direction

Accept variables as JSON:

```rust
pub async fn graphql_req(
    query: impl Into<String>,
    variables: serde_json::Value,
) -> Result<Value, HardcoverError>
```


Usage:

```rust
let variables = serde_json::json!({ "id": id });
```


or:

```rust
let variables = serde_json::json!({ "username": username });
```


---

## 16. IDs are converted to strings in GraphQL variables

In `from_data`:

```rust
vars.insert("id", user_data.to_string());
```


So numeric IDs are sent as strings.

GraphQL may coerce some values, but it is not ideal. If the query says `$id: Int!` or `$id: bigint!`, the JSON variable should be numeric, not string.

Better:

```rust
serde_json::json!({
    "id": id
})
```


This is especially important for type correctness and avoiding weird GraphQL validation issues.

---

## 17. Public structs lack common derives

Many public structs and enums derive only:

```rust
#[derive(Debug, Clone, Deserialize)]
```


Some have `PartialEq`, but most do not.

For a data-wrapper library, users often expect:

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
```


where applicable.

For serializable API models, users may also want:

```rust
Serialize
Deserialize
```


Possible improvements:

- Add `PartialEq` to most model structs.
- Add `Eq` where no `f64` fields are present.
- Add `Serialize`/`Deserialize` behind a feature or by default.
- Add `Copy` for small enums if appropriate.
- Add `Hash` for enums if useful.

Example:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PrivacySetting {
    Public,
    FollowersOnly,
    Private,
}
```


---

## 18. Some fields probably should not be mandatory

Example from `User`:

```rust
pub image_id: u64,
pub pronoun_personal: String,
pub pronoun_possessive: String,
```


These are parsed with `unwrap()`:

```rust
image_id: data.get_u64("image_id").unwrap(),
pronoun_personal: data.get_str("pronoun_personal").unwrap(),
pronoun_possessive: data.get_str("pronoun_possessive").unwrap(),
```


For public user profiles, these fields may plausibly be missing or null. You may want to audit every non-optional field and verify against the API schema.

Likely candidates for `Option<T>`:

- `User::image_id`
- `User::pronoun_personal`
- `User::pronoun_possessive`
- Some timestamp fields
- Some count fields depending on API behavior
- Some relation IDs

---

## 19. Time type choices are inconsistent

The code uses both:

```rust
PlainDateTime
OffsetDateTime
Date
```


That can be correct, but the choice should reflect the API schema.

Examples:

```rust
pub created_at: PlainDateTime,
pub updated_at: Option<OffsetDateTime>,
```


In `Book`, `created_at` is `PlainDateTime`, while `updated_at` is `Option<OffsetDateTime>`. In other resources, `updated_at` is sometimes `PlainDateTime`.

If the API returns timestamps with timezone offsets, prefer `OffsetDateTime`. If it returns naive timestamps, prefer `PrimitiveDateTime`/`PlainDateTime`.

Before alpha, audit all timestamp fields and make the policy consistent.

---

## 20. `time::PlainDateTime` may be an unusual public type choice

`PlainDateTime` exists in newer `time` versions, but many Rust users are more familiar with:

- `time::OffsetDateTime`
- `time::PrimitiveDateTime`
- `chrono::DateTime`

You do not need to switch, but document the choice and ensure the crate’s MSRV supports it.

---

## 21. Some helper implementations allocate unnecessarily

Examples:

```rust
.map(|s| s.to_string())
```


This is fine, but could use:

```rust
.map(str::to_owned)
```


Not critical.

More importantly:

```rust
.unwrap_or(&vec![])
```


appears in `get_link_vec` and `get_rating_vec`.

That creates a temporary vector reference. It works because it is only used immediately, but it is awkward and easy to avoid.

Better pattern:

```rust
self.get(key)
    .and_then(Value::as_array)
    .into_iter()
    .flatten()
    .map(...)
    .collect()
```


or:

```rust
let Some(arr) = self.get(key).and_then(Value::as_array) else {
    return Vec::new();
};
```


---

## 22. `GraphQLResponse` silently drops invalid array items

For example:

```rust
.filter_map(|v| v.as_str().map(|s| s.to_string()))
```


and:

```rust
.filter_map(|v| v.as_u64())
```


This means malformed or unexpected entries are silently omitted.

That can lead to confusing data loss. Even before full error handling, decide whether the library should:

- Silently skip invalid entries.
- Return an error.
- Preserve raw values.
- Panic.
- Treat the whole field as absent.

For alpha, document the behavior.

---

## 23. `GraphQLResponse` is implemented for all `serde_json::Value`

This trait is public through `client`, and the implementation applies globally to `Value`.

That is convenient internally, but it leaks parsing helpers into your public API. It also makes the trait appear more general-purpose than it is.

Consider moving it into a private module or making it `pub(crate)`.

---

## 24. Inconsistent enum fallback behavior

Some unknown values panic:

```rust
_ => panic!("Unknown record state")
```


Some default:

```rust
_ => RecordState2::Error
```


Some return `None`:

```rust
_ => None
```


Some “fail closed”:

```rust
_ => PrivacySetting::Private
```


Some “fail open”:

```rust
_ => AccountStatus::Activated
```


Even ignoring error handling, this inconsistency affects semantics.

Before alpha, choose a policy:

- Unknown enum values become `Unknown(...)`.
- Unknown enum values produce parse errors.
- Unknown optional enum values become `None`.
- Security/privacy-sensitive values default conservatively.

Whatever you choose, apply it consistently and document exceptions.

---

## 25. Naming inconsistency: `get_plaindt`

The method name:

```rust
fn get_plaindt(&self, key: &str) -> Option<PlainDateTime>;
```


is abbreviated and harder to read. Prefer:

```rust
fn get_plain_datetime(&self, key: &str) -> Option<PlainDateTime>;
```


Since this is still alpha, now is a good time to rename it.

---

## 26. Some type names are ambiguous or conflicting

There is a module:

```rust
reading_format.rs
```


and also in `edition.rs`:

```rust
pub enum ReadingFormat {
    Physical,
    Audio,
    Both,
    Ebook,
}
```


This can confuse users because `hardcover_wrapper::ReadingFormat` re-exports the struct from `reading_format.rs`, while `edition::ReadingFormat` is a separate enum.

Potential rename:

```rust
pub enum EditionReadingFormat {
    Physical,
    Audio,
    Both,
    Ebook,
}
```


Similarly, names like `List` can conflict mentally with collection types, though that one is probably acceptable because it matches the domain.

---

## 27. `BookRecordState` and `EditionRecordState` are poor public names

In `util.rs`:

```rust
pub enum RecordState2 { ... }
pub enum RecordState3 { ... }
```


These names look temporary and should not ship as public API.

Rename them based on domain:

```rust
pub enum BookRecordState { ... }
pub enum EditionRecordState { ... }
```


or:

```rust
pub enum ProcessingRecordState { ... }
pub enum LinkingRecordState { ... }
```


Alpha users will notice `BookRecordState`/`EditionRecordState` immediately.

---

## 28. Some structs query fields they do not use

As noted above, many `QUERY_FIELDS` include fields not represented in the struct. This happens in multiple files.

This has downsides:

- Larger payloads.
- Slower API responses.
- Confusing maintenance.
- Harder to see what the struct actually supports.
- Higher chance of breakage if unused fields change.

Before alpha, either:

1. Remove unused query fields, or
2. Add corresponding struct fields.

---

## 29. Some API methods use list queries where primary-key queries may be better

`User::from_id` uses:

```rust
users(where: {id: {_eq: $user}}, limit: 1)
```


Other types use:

```rust
books_by_pk(id: $id)
```


If Hardcover exposes `users_by_pk`, that would be more direct. If not, the current approach is fine, but the indexing behavior is currently fragile:

```rust
data["users"][0]
```


Same with `from_username`.

For ergonomics, consider returning an `Option<User>` from lookup-style methods, especially username searches.

---

## 30. `from_id` cannot distinguish “not found” from malformed data

This is technically related to error handling, but also public API design.

Current API:

```rust
async fn from_id(id: u64) -> Result<Self, reqwest::Error>;
```


A lookup by ID can naturally have three outcomes:

1. Network/request failure.
2. Found item.
3. No item found.

The current signature cannot represent “not found” except by failing later during parsing.

Better options:

```rust
async fn from_id(id: u64) -> Result<Option<Self>, HardcoverError>;
```


or:

```rust
async fn get_by_id(id: u64) -> Result<Self, HardcoverError>;
```


where `HardcoverError::NotFound` exists later.

For alpha, decide the intended semantics now.

---

## 31. `BaseHardcoverItem::new` is an odd public constructor

The trait exposes:

```rust
fn new(data: Value) -> Self;
```


This means users can construct model objects from arbitrary JSON values, but the method name `new` sounds like a normal constructor.

Better names:

```rust
fn from_graphql_value(data: Value) -> Self;
```


or make it internal:

```rust
pub(crate) fn from_value(...)
```


If this remains public, users may depend on it.

---

## 32. The `BaseHardcoverItem` trait uses `async fn in trait`

You currently suppress:

```rust
#[allow(async_fn_in_trait)]
```


This is okay if you accept the limitation, but public `async fn` in traits has constraints around `Send` bounds and dyn compatibility.

If the trait is intended as public API, consider using one of these approaches:

- Avoid exposing the trait.
- Use inherent async methods on each type.
- Use `async-trait` if object safety or dynamic dispatch matters.
- Return `impl Future + Send` manually if necessary.

For an alpha, the biggest question is whether `BaseHardcoverItem` is meant for users or only internal implementation reuse.

---

## 33. No feature flags

Current dependencies:

```toml
tokio = { version = "1.53.1", features = ["full"] }
reqwest = { version = "0.13.4", features = ["json"] }
serde_json = "1.0.132"
time = { version = "0.3.55", features = ["serde", "parsing", "macros"] }
```


`tokio` with `features = ["full"]` is heavy for a library. Libraries usually avoid forcing a large runtime feature set on downstream users.

Consider:

```toml
tokio = { version = "1.53.1", features = ["macros", "rt-multi-thread"], optional = true }
```


But actually, your library code may not need a direct `tokio` dependency at all unless tests/examples use it. Async functions using `reqwest` do not require your library to depend directly on Tokio unless you use Tokio APIs in the library itself.

You can move Tokio to dev-dependencies:

```toml
[dev-dependencies]
tokio = { version = "1.53.1", features = ["macros", "rt-multi-thread", "time"] }
```


If examples need Tokio, dev-dependencies are usually enough for examples/tests.

---

## 34. `time` dependency enables `macros` in normal dependencies

Your library itself does not appear to need `time` macros, but tests do:

```rust
use time::macros::datetime;
```


Move `macros` to dev-only if possible, or keep it if you want to expose macro-dependent examples. For library dependencies, smaller feature sets are better.

---

## 35. Examples use compile-time environment variables

The attached example uses:

```rust
client::set_api_key(env!("API_KEY"));
```


For a published example, prefer runtime env lookup:

```rust
let api_key = std::env::var("API_KEY")
    .expect("API_KEY environment variable must be set");

client::set_api_key(&api_key);
```


This makes `cargo build --examples` less surprising, depending on how the example is compiled/run.

---

## 36. Missing crate-level documentation

There is no crate-level documentation in `lib.rs`.

Add something like:

```rust
//! Rust wrapper for the Hardcover GraphQL API.
//!
//! # Example
//!
//! ```no_run
//! use hardcover_wrapper::{client, Book, BaseHardcoverItem};
//!
//! # async fn example() -> Result<(), reqwest::Error> {
//! client::set_api_key("YOUR_API_KEY");
//! let book = Book::from_id(484946).await?;
//! # Ok(())
//! # }
```

This improves docs.rs output substantially.

---

## 37. Missing per-type documentation

Public models and fields have no doc comments. Since this library is mostly public data models, docs are valuable.

For alpha, you do not need perfect docs everywhere, but at least document:

- Authentication
- `Book::from_id`
- `Author::from_id`
- `User::from_username`
- Public enums
- Any fields that map to Hardcover database IDs

---

## 38. No docs.rs / crates.io readiness check

Before alpha, run:
```
bash
cargo package
cargo publish --dry-run
cargo doc --no-deps
```
Check that:

- The package includes the expected files.
- Docs build cleanly.
- Examples do not accidentally require unavailable environment variables at compile time.
- The README renders correctly.
- License metadata is present.

---

## 39. Potential privacy concern: user fields include sensitive/private fields

`User` queries fields like:
```rust
email
unconfirmed_email
payment_system_id
membership
membership_ends_at
admin
```
Depending on Hardcover API permissions, these may only appear for the authenticated user or admins, but including them in a general `User` model may surprise users.

Consider separating:
```rust
PublicUser
AuthenticatedUser
```
or documenting that some fields are only populated depending on API permissions.

Also consider whether querying fields like `email` should be avoided in general-purpose user lookups unless specifically requested.

---

## 40. `QUERY_FIELDS` are private constants repeated in each module

This is fine, but if you plan to support custom selection sets or nested data later, this design may limit you.

Possible future designs:

- Lightweight default queries.
- Separate “full” queries.
- Builder pattern for selected fields.
- `Book::from_id` for default model only, and `client.graphql(...)` for custom queries.

For alpha, just be aware that these static field lists become the library’s effective schema contract.

---

## 41. Inconsistent optionality around counts

Many count fields are required:
```rust
pub books_count: u64,
pub users_count: u64,
pub followers_count: u64,
```
Some are optional:
```rust
pub lists_count: Option<u64>,
```
This may be correct, but it should be audited. Count fields are often nullable in APIs depending on query permissions or computed availability.

If unsure, making questionable fields `Option<u64>` is safer for alpha, though less ergonomic.

---

## 42. Inconsistent scalar choice for titles/names

Some fields that seem required are optional:
```rust
pub title: Option<String>,
```
for `Book`, while other names are required:
```rust
pub name: String,
```
for `Author`.

That may reflect the API, but if a book can exist without title due to partial/import state, maybe requiredness should be tied to state. Otherwise, users may find it odd that `Book::title` is optional.

At minimum, document why major display fields are optional.

---

## 43. `BookStatus::OK` should probably be `Ok`

Rust enum variants usually use `UpperCamelCase`, not all caps:
```rust
pub enum BookStatus {
OK,
ToReview,
Deleted,
Deduplicated
}
```
Prefer:
```rust
pub enum BookStatus {
Ok,
ToReview,
Deleted,
Deduplicated,
}
```
This is a small polish issue, but alpha is the right time to fix it.

---

## 44. Formatting issues

There are minor style inconsistencies:
```rust
pub locked:	bool,
```
Missing trailing commas in enum definitions:
```rust
Nonbinary
```

```rust
Duplicate
```
rustfmt will handle most of this. Before release, run:
```bash
cargo fmt
```
Also run:
```bash
cargo clippy --all-targets --all-features
```
---

## 45. `serde_json` dependency version in project differs from stated dependency set

Your `Cargo.toml` has:
```toml
serde_json = "1.0.132"
```
The project context says the dependency in use is `serde_json1.0.151`.

This may just be lockfile resolution, but for reproducibility and expectations, check whether you want to update the declared version:
```toml
serde_json = "1.0.151"
```
Usually `"1.0"` is enough unless you need a specific newer feature.

---

## 46. The test suite sleeps to avoid rate limits, but this scales poorly

The test setup does:
```rust
sleep(Duration::from_secs(2)).await;
```
before every test.

If tests run in parallel, the sleeps do not necessarily serialize the requests. If they run sequentially, the suite becomes very slow.

Better options:

- Mark live tests ignored.
- Use `serial_test` for live API tests.
- Use a shared rate limiter.
- Use mocked JSON for ordinary tests.
- Use recorded fixtures.

---

## 47. No mocking strategy

For a wrapper library, parsing correctness is a major part of the value. You can test that without live API calls.

Consider adding fixture-based tests:
```
tests/fixtures/book_484946.json
tests/fixtures/author_132049.json
```
Then test:
```rust
let value: serde_json::Value = serde_json::from_str(include_str!("fixtures/book.json"))?;
let book = Book::new(value["data"]["books_by_pk"].clone());
```
Long term, if you switch to Serde, this becomes even cleaner.

---

## 48. No CI configuration visible

There is no visible GitHub Actions or other CI setup. For alpha, a simple CI workflow should run:
```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
cargo doc --no-deps
cargo publish --dry-run
```
For live API tests, keep them ignored or behind an explicit feature/environment gate.

---

## 49. No changelog

For an alpha library, a simple `CHANGELOG.md` is useful. It helps early users understand breaking changes.

Suggested sections:

# Changelog

## 0.1.0

- Initial alpha release.
- Supports fetching books, authors, users, editions, lists, goals, and related entities.

---

## 50. No explicit stability policy

Since you are preparing an alpha, it is good to say this clearly:

## Stability

This crate is currently in alpha. Public APIs may change between 0.x releases.

This reduces user surprise.

---

# Highest-priority fixes before alpha

If you only handle a few things before release, I would prioritize these:

1. **Replace or supplement the global API key with a reusable client type.**
2. **Reuse a single `reqwest::Client` instead of creating one per request.**
3. **Change GraphQL variables from `HashMap<&str, String>` to JSON values.**
4. **Audit public API exposure in `lib.rs`.**
5. **Rename obvious temporary/public names like `BookRecordState` and `EditionRecordState`.**
6. **Move `tokio` to dev-dependencies if the library itself does not need it.**
7. **Add Cargo metadata required for publication.**
8. **Expand the README with install/auth/usage/status examples.**
9. **Mark live API tests as ignored or add fixture-based tests.**
10. **Audit timestamp types, optional fields, and enum unknown-value handling.**

Those changes would make the alpha feel much more intentional, even if full error handling is still planned later.

