use base64::{engine::general_purpose, Engine};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Credentials {
    pub username: String,
    pub password: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserInfo {
    pub id: String,
    pub username: Option<String>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ApiToken {
    pub token: String,
}

impl ApiToken {
    pub fn body_as_object<T>(self) -> Option<T>
    where
        T: DeserializeOwned,
    {
        match self.token.split(".").nth(1) {
            Some(payload) => {
                let payload = base64::engine::GeneralPurpose::new(
                    &base64::alphabet::STANDARD,
                    general_purpose::NO_PAD,
                )
                .decode(payload)
                .expect("token is not valid base64 encoding");
                let payload = String::from_utf8(payload).expect("token payload is not valid utf8");
                log::debug!("API token payload: {}", payload);
                serde_json::from_str(&payload).ok()
            }
            None => {
                log::error!("Unable to decode API token payload");
                None
            }
        }
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
pub struct DBReply<T> {
    pub status: String,
    pub time: String,
    pub result: T,
}