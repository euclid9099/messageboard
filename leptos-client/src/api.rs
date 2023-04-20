use gloo_net::http::{Request, Response};
use serde::de::DeserializeOwned;
use serde_json::Value;
use thiserror::Error;

use crate::types::{self, *};

#[derive(Clone, Copy)]
pub struct UnauthorizedApi {
    url: &'static str,
}

#[derive(Clone)]
pub struct AuthorizedApi {
    url: &'static str,
    token: ApiToken,
}

impl UnauthorizedApi {
    pub const fn new(url: &'static str) -> Self {
        Self { url }
    }
    pub async fn register(&self, credentials: &Credentials) -> Result<AuthorizedApi> {
        let url = format!("{}/signup", self.url);
        let response = Request::post(&url).json(credentials)?.send().await?;
        let json_transformation = into_json::<types::Reply<ApiToken>>(response).await;
        match json_transformation {
            Ok(token_reply) => match token_reply.content {
                Some(token) => Ok(AuthorizedApi::new(self.url, token)),
                None => Err(Error::Api(token_reply.error.unwrap_or(types::Error {
                    message: "neither token nor response have been set".to_string(),
                }))),
            },
            Err(e) => Err(e),
        }
    }
    pub async fn login(&self, credentials: &Credentials) -> Result<AuthorizedApi> {
        let url = format!("{}/login", self.url);
        let response = Request::post(&url).json(credentials)?.send().await?;
        let json_transformation = into_json::<types::Reply<ApiToken>>(response).await;
        match json_transformation {
            Ok(token_reply) => match token_reply.content {
                Some(token) => Ok(AuthorizedApi::new(self.url, token)),
                None => Err(Error::Api(token_reply.error.unwrap_or(types::Error {
                    message: "neither token nor response have been set".to_string(),
                }))),
            },
            Err(e) => Err(e),
        }
    }
}

impl AuthorizedApi {
    pub const fn new(url: &'static str, token: ApiToken) -> Self {
        Self { url, token }
    }
    async fn send<T>(&self, req: Request) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let response = req.header("x-token", &self.token.token).send().await?;
        into_json(response).await
    }
    pub async fn logout(&self) -> Result<()> {
        return Ok(());
    }
    pub async fn user_info(&self) -> Result<UserInfo> {
        log::debug!("{}", self.token.token);
        let user_url = format!(
            "{}/users/{}",
            self.url,
            self.token.clone().body_as_object::<Value>().unwrap()["ID"]
                .as_str()
                .unwrap()
        );
        log::debug!("User info url: {}", user_url);

        let user_reply: types::Reply<Vec<DBReply<Vec<UserInfo>>>> =
            into_json(Request::get(&user_url).send().await?).await?;
        match user_reply.content {
            Some(user_in_db) => {
                let user = user_in_db.get(0).unwrap().result.get(0).unwrap().to_owned();
                log::debug!("User info: {:?}", user);
                return Ok(user);
            }
            None => {
                return Err(Error::Api(
                    user_reply
                        .error
                        .expect("neither token nor response content have been set"),
                ))
            }
        }
    }
    pub fn token(&self) -> &ApiToken {
        &self.token
    }
}

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Fetch(#[from] gloo_net::Error),
    #[error("{0:?}")]
    Api(types::Error),
}

impl From<types::Error> for Error {
    fn from(e: types::Error) -> Self {
        Self::Api(e)
    }
}

async fn into_json<T>(response: Response) -> Result<T>
where
    T: DeserializeOwned,
{
    // ensure we've got 2xx status
    Ok(response.json().await?)
}
