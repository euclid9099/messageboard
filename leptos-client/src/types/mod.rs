use std::cmp::Ordering;

use base64::{engine::general_purpose, Engine};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize)]
pub struct Credentials {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct FollowerReply {
    pub followers: Vec<UserInfo>,
}

#[derive(Serialize, Deserialize)]
pub struct FollowingReply {
    pub following: Vec<UserInfo>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub struct UserInfo {
    pub id: String,
    pub username: Option<String>,
    #[serde(default)]
    pub admin: bool,
    pub about: Option<String>,
    pub profile_picture: Option<String>,
    pub followers: Option<u32>,
    pub following: Option<u32>,
    #[serde(default)]
    pub you_follow: bool,
}

impl Ord for UserInfo {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.id.cmp(&other.id)
    }
}

impl PartialOrd for UserInfo {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
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
    payload: Value,
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
            payload: serde_json::from_str::<Value>(
                &String::from_utf8(payload).expect("token payload is not valid utf8"),
            )
            .unwrap(),
        }
    }

    pub fn id(&self) -> &str {
        self.payload["ID"].as_str().unwrap()
    }

    pub fn exp(&self) -> i64 {
        self.payload["exp"].as_i64().unwrap_or(0)
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
