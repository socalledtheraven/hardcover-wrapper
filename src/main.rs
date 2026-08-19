use std::any::TypeId;
use std::collections::HashMap;
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE, USER_AGENT, HeaderMap};

use serde::{Deserialize, Serialize};

use serde_json;
use serde_json::Value;

use time::{Date, OffsetDateTime, PlainDateTime};
use time::format_description::well_known::Iso8601;
/*
request: Object {
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
                "updated_at": String("2026-08-19T08:36:30.703543+00:00"),
                "username": String("prophecyreviews"),
            },
        ],
    },
}
 */

/*
request: Object {
    "data": Object {
        "users": Array [
            Object {
                "access_level": Null,
                "account_privacy_setting_id": Number(1),
                "activity_privacy_settings_id": Number(1),
                "admin": Bool(true),
                "bio": String("Hey hey! I'm Adam, Hardcover's founder. I love science fiction, fantasy, sciencey-nonfiction and anything that helps me learn a little bit more about the world, or help empathize with others."),
                "birthdate": String("1982-05-18"),
                "books_count": Number(1262),
                "cached_cover": Object {
                    "color": String("#000000"),
                    "color_name": String("Black"),
                    "height": Number(400),
                    "id": Number(4744706),
                    "url": String("https://assets.hardcover.app/static/bookHeaders/science-fiction.webp"),
                    "width": Number(2000),
                },
                "cached_genres": Array [
                    Object {
                        "count": Number(499),
                        "tag": String("Fantasy"),
                        "tagSlug": String("fantasy"),
                    },
                    Object {
                        "count": Number(438),
                        "tag": String("Science fiction"),
                        "tagSlug": String("science-fiction"),
                    },
                    Object {
                        "count": Number(182),
                        "tag": String("Classics"),
                        "tagSlug": String("classics"),
                    },
                    Object {
                        "count": Number(180),
                        "tag": String("Fiction"),
                        "tagSlug": String("fiction"),
                    },
                    Object {
                        "count": Number(267),
                        "tag": String("Young Adult"),
                        "tagSlug": String("young-adult"),
                    },
                    Object {
                        "count": Number(30),
                        "tag": String("History"),
                        "tagSlug": String("history"),
                    },
                    Object {
                        "count": Number(8),
                        "tag": String("Literature"),
                        "tagSlug": String("literature"),
                    },
                    Object {
                        "count": Number(11),
                        "tag": String("Poetry"),
                        "tagSlug": String("poetry"),
                    },
                    Object {
                        "count": Number(3),
                        "tag": String("Fairy tales"),
                        "tagSlug": String("fairy-tales"),
                    },
                    Object {
                        "count": Number(1),
                        "tag": String("Ancient Greece"),
                        "tagSlug": String("ancient-greece"),
                    },
                    Object {
                        "count": Number(352),
                        "tag": String("Adventure"),
                        "tagSlug": String("adventure"),
                    },
                    Object {
                        "count": Number(205),
                        "tag": String("Dystopian"),
                        "tagSlug": String("dystopian"),
                    },
                    Object {
                        "count": Number(105),
                        "tag": String("Comics"),
                        "tagSlug": String("comics"),
                    },
                    Object {
                        "count": Number(121),
                        "tag": String("Space"),
                        "tagSlug": String("space"),
                    },
                    Object {
                        "count": Number(75),
                        "tag": String("Aliens"),
                        "tagSlug": String("aliens"),
                    },
                    Object {
                        "count": Number(112),
                        "tag": String("War"),
                        "tagSlug": String("war"),
                    },
                    Object {
                        "count": Number(1),
                        "tag": String("Tragedy"),
                        "tagSlug": String("tragedy"),
                    },
                    Object {
                        "count": Number(121),
                        "tag": String("Nonfiction"),
                        "tagSlug": String("nonfiction"),
                    },
                    Object {
                        "count": Number(42),
                        "tag": String("Biography"),
                        "tagSlug": String("biography"),
                    },
                    Object {
                        "count": Number(2),
                        "tag": String("Sports"),
                        "tagSlug": String("sports"),
                    },
                    Object {
                        "count": Number(4),
                        "tag": String("Middle Grade"),
                        "tagSlug": String("middle-grade"),
                    },
                ],
                "cached_image": Object {
                    "color": String("#c2aca2"),
                    "color_name": String("Silver"),
                    "height": Number(946),
                    "id": Number(3462414),
                    "url": String("https://assets.hardcover.app/users/1/4971270866700959.jpg"),
                    "width": Number(946),
                },
                "confirmation_sent_at": Null,
                "confirmed_at": Null,
                "created_at": String("2021-10-02T21:11:34.232537+00:00"),
                "current_sign_in_at": Null,
                "email": Null,
                "email_verified": Null,
                "flair": String("Supporter"),
                "followed_users_count": Number(1009),
                "followers_count": Number(9627),
                "id": Number(1),
                "image_id": Number(3462414),
                "last_activity_at": String("2026-08-18T21:57:25.009025"),
                "last_sign_in_at": Null,
                "librarian_roles": Array [
                    String(""),
                    String("appender"),
                    String("editor"),
                    String("librarian"),
                    String("reporter_admin"),
                    String("book_mapper"),
                    String("admin"),
                    String("edition_splitter"),
                ],
                "link": Null,
                "location": String("Salt Lake City, UT"),
                "locked_at": Null,
                "membership": String("Supporter"),
                "membership_ends_at": Null,
                "name": String("Adam"),
                "object_type": Null,
                "onboarded": Bool(true),
                "payment_system_id": Null,
                "pro": Bool(true),
                "pronoun_personal": String("he"),
                "pronoun_possessive": String("his"),
                "referrer_id": Null,
                "referrer_url": Null,
                "remember_created_at": Null,
                "reset_password_sent_at": Null,
                "sign_in_count": Null,
                "status_id": Number(2),
                "timezone": Null,
                "unconfirmed_email": Null,
                "updated_at": String("2026-08-19T10:10:29.894336+00:00"),
                "username": String("adam"),
            },
        ],
    },
}

 */

#[derive(Debug, Serialize, Deserialize)]
struct User {
    access_level: Option<u64>,
    // todo!
    account_privacy_settings_id: PrivacySetting,
    // todo!
    activities: MaybeInit<Vec<Activity>>,
    // todo!
    activity_privacy_settings_id: PrivacySetting,
    admin: bool,
    bio: Option<String>,
    birthdate: Option<Date>,
    // todo!
    blocked_users: MaybeInit<Vec<BlockedUser>>,
    books_count: u64,
    cached_cover: Image,
    cached_genres: Vec<Genre>,
    cached_image: Image,
    collection_imports: MaybeInit<Vec<Import>>,
    confirmation_sent_at: Option<PlainDateTime>,
    confirmed_at: Option<PlainDateTime>,
    // todo!
    created_at: Option<OffsetDateTime>,
    current_sign_in_at: Option<PlainDateTime>,
    email: Option<String>,
    // todo!
    email_verified: Option<OffsetDateTime>,
    flair: Option<String>,
    // todo!
    followed_by_users: MaybeInit<Vec<User>>,
    // todo!
    followed_lists: MaybeInit<Vec<List>>,
    // todo!
    followed_prompts: MaybeInit<Vec<Prompt>>,
    // todo!
    followed_users: MaybeInit<Vec<User>>,
    followed_users_count: u64,
    followers_count: u64,
    // todo!
    follows: MaybeInit<Vec<Follow>>,
    // todo!
    goals: MaybeInit<Vec<Goal>>,
    id: u64,
    // todo!
    images: MaybeInit<Vec<Image>>,
    image_id: u64,
    last_activity_at: Option<PlainDateTime>,
    last_sign_in_at: Option<PlainDateTime>,
    // todo!
    librarian_roles: Vec<String>,
    link: Option<String>,
    // todo!
    links: MaybeInit<Option<Vec<Option<Link>>>>,
    // todo!
    lists: MaybeInit<Vec<List>>,
    location: Option<String>,
    locked_at: Option<PlainDateTime>,
    membership: Option<String>,
    membership_ends_at: Option<PlainDateTime>,
    name: Option<String>,
    // todo!
    notification_deliveries: MaybeInit<Vec<NotificationDelivery>>,
    object_type: Option<String>,
    onboarded: bool,
    payment_system_id: Option<u64>,
    pro: bool,
    // todo!
    prompt_answers: MaybeInit<Vec<PromptAnswer>>,
    // todo!
    prompts: MaybeInit<Vec<Prompt>>,
    pronoun_personal: String,
    pronoun_possessive: String,
    referrer_id: Option<u64>,
    referrer_url: Option<String>,
    // todo!
    referred_users: MaybeInit<Vec<UserBook>>,
    remember_created_at: Option<PlainDateTime>,
    // todo!
    reported_user_flags: MaybeInit<Vec<UserFlag>>,
    reset_password_sent_at: Option<PlainDateTime>,
    sign_in_count: Option<u64>,
    // todo!
    status_id: u64,
    // todo!
    taggings: MaybeInit<Vec<Tagging>>,
    // todo!
    timezone: Option<String>,
    unconfirmed_email: Option<String>,
    // todo!
    updated_at: OffsetDateTime,
    // todo!
    user_books: MaybeInit<Vec<UserBook>>,
    // todo!
    user_flags: MaybeInit<Vec<UserFlag>>,
    username: String,
}


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

impl User {
    async fn new(username: &str) -> Result<Self, reqwest::Error> {
        let query = r#"
        query GetUser($user: citext!) {
          users(where: {username: {_eq: $user}}, limit: 1) {
            access_level
            account_privacy_setting_id
            activity_privacy_settings_id
            admin
            bio
            birthdate
            books_count
            cached_cover
            cached_genres
            cached_image
            confirmation_sent_at
            confirmed_at
            created_at
            current_sign_in_at
            email
            email_verified
            flair
            followed_users_count
            followers_count
            id
            image_id
            last_activity_at
            last_sign_in_at
            librarian_roles
            link
            location
            locked_at
            membership
            membership_ends_at
            name
            object_type
            onboarded
            payment_system_id
            pro
            pronoun_personal
            pronoun_possessive
            referrer_id
            referrer_url
            remember_created_at
            reset_password_sent_at
            sign_in_count
            status_id
            timezone
            unconfirmed_email
            updated_at
            username
          }
        }
    "#;

        let mut vars = HashMap::new();
        vars.insert("user", username);

        let resp = graphql_req(query, vars).await?;

        let data = &resp["data"]["users"][0];

        Ok(User {
            access_level: {
                get_u64_from_resp(data, "access_level")
            },
            account_privacy_settings_id: {
                // in either case of it being unknown, we default to private
                // this will never happen, but better to fail closed
                data["account_privacy_setting_id"]
                    .as_u64()
                    .map(|v| match v {
                        1 => PrivacySetting::Public,
                        2 => PrivacySetting::FollowersOnly,
                        3 => PrivacySetting::Private,
                        _ => PrivacySetting::Private,
                    })
                    .unwrap_or(PrivacySetting::Private)
            },
            activities: { MaybeInit::Uninitialised },
            activity_privacy_settings_id: {
                // in either case of it being unknown, we default to private,
                // this will never happen, but better to fail closed
                data["activity_privacy_settings_id"]
                    .as_u64()
                    .map(|v| match v {
                        1 => PrivacySetting::Public,
                        2 => PrivacySetting::FollowersOnly,
                        3 => PrivacySetting::Private,
                        _ => PrivacySetting::Private,
                    })
                    .unwrap_or(PrivacySetting::Private)
            },
            admin: {
                // should always be present
                get_bool_from_resp(data, "admin")
            },
            bio: {
                get_str_from_resp(data, "bio")
            },
            birthdate: {
                get_date_from_resp(data, "birthdate")
            },
            blocked_users: { MaybeInit::Uninitialised },
            books_count: {
                get_u64_from_resp(data, "books_count").unwrap()
            },
            cached_cover: {
                Image::new(data["cached_cover"].clone())
            },
            cached_genres: {
                let x = data["cached_genres"].as_array().unwrap();
                x.iter().filter_map(|v| {
                    let count = v["count"].as_u64()?;
                    let tag = v["tag"].as_str()?.to_string();
                    let tag_slug = v["tag_slug"].as_str()?.to_string();
                    Some(Genre { count, tag, tag_slug })
                }).collect()
            },
            cached_image: { Image::new(data["cached_image"].clone()) },
            collection_imports: { MaybeInit::Uninitialised },
            confirmation_sent_at: {
                get_plaindatetime_from_resp(data, "confirmation_sent_at")
            },
            confirmed_at: {
                get_plaindatetime_from_resp(data, "confirmed_at")
            },
            created_at: {
                get_offsetdatetime_from_resp(data, "created_at")
            },
            current_sign_in_at: {
                get_plaindatetime_from_resp(data, "current_sign_in_at")
            },
            email: {
                get_str_from_resp(data, "email")
            },
            email_verified: {
                get_offsetdatetime_from_resp(data, "email_verified")
            },
            flair: {
                get_str_from_resp(data, "flair")
            },
            followed_by_users: { MaybeInit::Uninitialised },
            followed_lists: { MaybeInit::Uninitialised },
            followed_prompts: { MaybeInit::Uninitialised },
            followed_users: { MaybeInit::Uninitialised },
            followed_users_count: {
                get_u64_from_resp(data, "followed_users_count").unwrap()
            },
            followers_count: {
                get_u64_from_resp(data, "followers_count").unwrap()
            },
            follows: { MaybeInit::Uninitialised },
            goals: { MaybeInit::Uninitialised },
            id: get_u64_from_resp(data, "id").unwrap(),
            images: { MaybeInit::Uninitialised },
            image_id: get_u64_from_resp(data, "image_id").unwrap(),
            last_activity_at: get_plaindatetime_from_resp(data, "last_activity_at"),
            last_sign_in_at: get_plaindatetime_from_resp(data, "last_sign_in_at"),
            librarian_roles: {
                let x = data["librarian_roles"].as_array().unwrap();
                x.iter().filter_map(|v| {
                    v.as_str().map(|s| s.to_string())
                }).collect()
            },
            link: {
                get_str_from_resp(data, "link")
            },
            links: { MaybeInit::Uninitialised },
            lists: { MaybeInit::Uninitialised },
            location: {
                get_str_from_resp(data, "location")
            },
            locked_at: {
                get_plaindatetime_from_resp(data, "locked_at")
            },
            membership: {
                get_str_from_resp(data, "membership")
            },
            membership_ends_at: {
                get_plaindatetime_from_resp(data, "membership_ends_at")
            },
            name: {
                get_str_from_resp(data, "name")
            },
            notification_deliveries: { MaybeInit::Uninitialised },
            object_type: {
                get_str_from_resp(data, "object_type")
            },
            onboarded: {
                get_bool_from_resp(data, "onboarded")
            },
            payment_system_id: {
                get_u64_from_resp(data, "payment_system_id")
            },
            pro: {
                get_bool_from_resp(data, "pro")
            },
            prompt_answers: { MaybeInit::Uninitialised },
            prompts: { MaybeInit::Uninitialised },
            pronoun_personal: {
                get_str_from_resp(data, "pronoun_personal").unwrap()
            },
            pronoun_possessive: {
                get_str_from_resp(data, "pronoun_possessive").unwrap()
            },
            referrer_id: {
                get_u64_from_resp(data, "referrer_id")
            },
            referrer_url: {
                get_str_from_resp(data, "referrer_url")
            },
            referred_users: { MaybeInit::Uninitialised },
            remember_created_at: {
                get_plaindatetime_from_resp(data, "remember_created_at")
            },
            reported_user_flags: { MaybeInit::Uninitialised },
            reset_password_sent_at: {
                get_plaindatetime_from_resp(data, "reset_password_sent_at")
            },
            sign_in_count: {
                get_u64_from_resp(data, "sign_in_count")
            },
            status_id: {
                get_u64_from_resp(data, "status_id").unwrap()
            },
            taggings: { MaybeInit::Uninitialised },
            timezone: {
                get_str_from_resp(data, "timezone")
            },
            unconfirmed_email: {
                get_str_from_resp(data, "unconfirmed_email")
            },
            updated_at: {
                get_offsetdatetime_from_resp(data, "updated_at").unwrap()
            },
            user_books: { MaybeInit::Uninitialised },
            user_flags: { MaybeInit::Uninitialised },
            username: {
                get_str_from_resp(data, "username").unwrap()
            },
        })
    }
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let me = User::new("adam").await;

    println!("{me:#?}");
    Ok(())
}

fn create_headers(api_key: &str) -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        format!("Bearer {}", api_key).parse().unwrap()
    );
    headers.insert(
        USER_AGENT,
        "hardcover-api-wrapper".parse().unwrap()
    );
    headers.insert(
        CONTENT_TYPE,
        "application/json".parse().unwrap()
    );

    headers
}

async fn graphql_req(query: &str, variables: HashMap<&str, &str>) -> Result<Value, reqwest::Error> {
    let headers = create_headers(env!("API_KEY"));

    let payload = serde_json::json!({
        "query": query,
        "variables": variables,
    });

    let request: Value = reqwest::Client::new()
        .post("https://api.hardcover.app/v1/graphql")
        .headers(headers)
        .json(&payload)
        .send()
        .await?
        .json()
        .await?;

    // todo! add error handling for internal request errors

    Ok(request)
}

fn get_u64_from_resp(data: &Value, key: &str) -> Option<u64> {
    data.get(key)
        .and_then(|v| v.as_u64())
}

fn get_str_from_resp(data: &Value, key: &str) -> Option<String> {
    data.get(key)
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
}

fn get_date_from_resp(data: &Value, key: &str) -> Option<Date> {
    data.get(key)
        .and_then(|v| v.as_str())
        .and_then(|s| Date::parse(s, &Iso8601::DATE).ok())
        .and_then(|d| Some(d))
}

fn get_plaindatetime_from_resp(data: &Value, key: &str) -> Option<PlainDateTime> {
    data.get(key)
        .and_then(|v| v.as_str())
        .and_then(|s| PlainDateTime::parse(s, &Iso8601::DATE_TIME).ok())
        .and_then(|d| Some(d))
}

fn get_offsetdatetime_from_resp(data: &Value, key: &str) -> Option<OffsetDateTime> {
    data.get(key)
        .and_then(|v| v.as_str())
        .and_then(|s| OffsetDateTime::parse(s, &Iso8601::DATE_TIME).ok())
        .and_then(|d| Some(d))
}

fn get_bool_from_resp(data: &Value, key: &str) -> bool {
    data[key]
        .as_bool()
        .unwrap()
}