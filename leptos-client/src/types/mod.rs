use base64::{engine::general_purpose, Engine};
use chrono::{DateTime, Utc};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize)]
pub struct Credentials {
    pub username: String,
    pub password: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserInfo {
    pub id: String,
    pub username: Option<String>,
    pub admin: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Post {
    pub author: Option<UserInfo>,
    pub edited: bool,
    pub id: String,
    pub message: String,
    pub time: DateTime<Utc>,
    pub likes: u32,
    pub liked: Option<bool>,
    pub dislikes: u32,
    pub disliked: Option<bool>,
    pub responses: u16,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ApiToken {
    pub token: String,
    id: String,
}

impl ApiToken {
    pub fn new(token: String) -> Self {
        let payload = base64::engine::GeneralPurpose::new(
            &base64::alphabet::STANDARD,
            general_purpose::NO_PAD,
        )
        .decode(token.split(".").nth(1).expect("invalid token"))
        .expect("token is not valid base64 encoding");
        ApiToken {
            token,
            id: serde_json::from_str::<Value>(
                &String::from_utf8(payload).expect("token payload is not valid utf8"),
            )
            .unwrap()["ID"]
                .as_str()
                .unwrap()
                .to_string(),
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Error {
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Reply<T> {
    pub message: String,
    pub content: Option<T>,
    pub error: Option<Error>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmptyReply {
    pub message: String,
    pub error: Option<Error>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "status")]
pub enum DBReply<T> {
    OK { time: String, result: T },
    ERR { time: String, detail: String },
}
