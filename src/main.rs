pub mod graphql;
pub mod user;

use serde::{Deserialize, Serialize};

use serde_json;
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
enum PrivacySetting {
    Public,
    FollowersOnly,
    Private,
}

#[derive(Debug, Serialize, Deserialize)]
enum MaybeInit<T> {
    Uninitialised,
    Initialised(T),
}

#[derive(Debug, Serialize, Deserialize)]
struct Genre {
    count: u64,
    tag: String,
    tag_slug: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Tagging {}

#[derive(Debug, Serialize, Deserialize)]
struct UserFlag {}

#[derive(Debug, Serialize, Deserialize)]
struct UserBook {}

#[derive(Debug, Serialize, Deserialize)]
struct PromptAnswer {}

#[derive(Debug, Serialize, Deserialize)]
struct NotificationDelivery {}

#[derive(Debug, Serialize, Deserialize)]
struct Link {}

#[derive(Debug, Serialize, Deserialize)]
struct Image {
    color: String,
    color_name: String,
    height: u64,
    id: u64,
    url: String,
    width: u64,
}

impl Image {
    fn new(resp: Value) -> Self {
        Image {
            color: resp["color"].as_str().unwrap().to_string(),
            color_name: resp["color_name"].as_str().unwrap().to_string(),
            height: resp["height"].as_u64().unwrap(),
            id: resp["id"].as_u64().unwrap(),
            url: resp["url"].as_str().unwrap().to_string(),
            width: resp["width"].as_u64().unwrap(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Goal {}

#[derive(Debug, Serialize, Deserialize)]
struct Follow {}

#[derive(Debug, Serialize, Deserialize)]
struct Prompt {}

#[derive(Debug, Serialize, Deserialize)]
struct List {}

#[derive(Debug, Serialize, Deserialize)]
struct Import {}

#[derive(Debug, Serialize, Deserialize)]
struct BlockedUser {}

#[derive(Debug, Serialize, Deserialize)]
struct Activity {}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let me = user::User::from_username("prophecyreviews").await;

    println!("{me:#?}");

    let t = r##"resp: Object {
    "data": Object {
        "me": Array [
            Object {
                "access_level": Number(10),
                "account_privacy_setting_id": Number(1),
                "activity_privacy_settings_id": Number(1),
                "admin": Bool(false),
                "bio": Null,
                "birthdate": Null,
                "books_count": Number(537),
                "cached_cover": Object {
                    "color": String("#000000"),
                    "color_name": String("Black"),
                    "height": Number(400),
                    "id": Number(4744686),
                    "url": String("https://assets.hardcover.app/static/bookHeaders/bookstore.webp"),
                    "width": Number(2000),
                },
                "cached_genres": Array [],
                "cached_image": Object {
                    "color": String("#000000"),
                    "color_name": String("Black"),
                    "height": Number(500),
                    "id": Number(3455125),
                    "url": String("https://assets.hardcover.app/static/avatars/profile3.png"),
                    "width": Number(500),
                },
                "confirmation_sent_at": String("2025-10-31T11:44:46.631696"),
                "confirmed_at": String("2025-10-31T11:45:28.99063"),
                "created_at": String("2025-10-31T11:44:46.631601+00:00"),
                "current_sign_in_at": String("2025-12-22T15:11:39.85551"),
                "email": String("cassie.dalrymple3@gmail.com"),
                "email_verified": Null,
                "flair": Null,
                "followed_users_count": Number(0),
                "followers_count": Number(1),
                "id": Number(52626),
                "image_id": Number(3455125),
                "last_activity_at": String("2026-08-19T08:36:30.693079"),
                "last_sign_in_at": String("2025-11-26T02:58:07.409246"),
                "librarian_roles": Array [
                    String("appender"),
                    String("editor"),
                    String("librarian"),
                ],
                "link": String(""),
                "location": String(""),
                "locked_at": Null,
                "membership": Null,
                "membership_ends_at": Null,
                "name": String("prophecyreviews"),
                "object_type": String("User"),
                "onboarded": Bool(true),
                "payment_system_id": Null,
                "pro": Bool(false),
                "pronoun_personal": String("she"),
                "pronoun_possessive": String("her"),
                "referrer_id": Number(28377),
                "referrer_url": String("https://news.ycombinator.com/"),
                "remember_created_at": String("2025-11-05T19:46:20.95935"),
                "reset_password_sent_at": Null,
                "sign_in_count": Number(4),
                "status_id": Number(2),
                "timezone": String("Europe/London"),
                "unconfirmed_email": Null,
                "username": String("prophecyreviews"),
            },
        ],
    },
}
"##;

    let t2 = r##"resp: Object {
    "data": Object {
        "users": Array [
            Object {
                "access_level": Number(10),
                "account_privacy_setting_id": Number(1),
                "activity_privacy_settings_id": Number(1),
                "admin": Bool(false),
                "bio": Null,
                "birthdate": Null,
                "books_count": Number(537),
                "cached_cover": Object {
                    "color": String("#000000"),
                    "color_name": String("Black"),
                    "height": Number(400),
                    "id": Number(4744686),
                    "url": String("https://assets.hardcover.app/static/bookHeaders/bookstore.webp"),
                    "width": Number(2000),
                },
                "cached_genres": Array [],
                "cached_image": Object {
                    "color": String("#000000"),
                    "color_name": String("Black"),
                    "height": Number(500),
                    "id": Number(3455125),
                    "url": String("https://assets.hardcover.app/static/avatars/profile3.png"),
                    "width": Number(500),
                },
                "confirmation_sent_at": String("2025-10-31T11:44:46.631696"),
                "confirmed_at": String("2025-10-31T11:45:28.99063"),
                "created_at": String("2025-10-31T11:44:46.631601+00:00"),
                "current_sign_in_at": String("2025-12-22T15:11:39.85551"),
                "email": String("cassie.dalrymple3@gmail.com"),
                "email_verified": Null,
                "flair": Null,
                "followed_users_count": Number(0),
                "followers_count": Number(1),
                "id": Number(52626),
                "image_id": Number(3455125),
                "last_activity_at": String("2026-08-19T08:36:30.693079"),
                "last_sign_in_at": String("2025-11-26T02:58:07.409246"),
                "librarian_roles": Array [
                    String("appender"),
                    String("editor"),
                    String("librarian"),
                ],
                "link": String(""),
                "location": String(""),
                "locked_at": Null,
                "membership": Null,
                "membership_ends_at": Null,
                "name": String("prophecyreviews"),
                "object_type": String("User"),
                "onboarded": Bool(true),
                "payment_system_id": Null,
                "pro": Bool(false),
                "pronoun_personal": String("she"),
                "pronoun_possessive": String("her"),
                "referrer_id": Number(28377),
                "referrer_url": String("https://news.ycombinator.com/"),
                "remember_created_at": String("2025-11-05T19:46:20.95935"),
                "reset_password_sent_at": Null,
                "sign_in_count": Number(4),
                "status_id": Number(2),
                "timezone": String("Europe/London"),
                "unconfirmed_email": Null,
                "username": String("prophecyreviews"),
            },
        ],
    },
}
"##;

    println!("{}", t == t2);

    Ok(())
}
