use gloo_net::http::{Request, Response};
use serde::de::DeserializeOwned;
use serde_json::{json, Value};
use thiserror::Error;

use crate::types::{self, *};
use crate::DEFAULT_API_URL;

pub async fn login(credentials: &Credentials) -> Result<ApiToken> {
    let url = format!("{}/login", DEFAULT_API_URL);
    authenticate(credentials, url).await
}

pub async fn register(credentials: &Credentials) -> Result<ApiToken> {
    let url = format!("{}/signup", DEFAULT_API_URL);
    authenticate(credentials, url).await
}

async fn authenticate(credentials: &Credentials, url: String) -> Result<ApiToken> {
    let response = Request::post(&url).json(credentials)?.send().await?;
    let json_transformation = into_json::<types::Reply<Value>>(response).await;
    match json_transformation {
        Ok(token_reply) => match token_reply.content {
            Some(val) => Ok(ApiToken::new(val["token"].as_str().unwrap().to_string())),
            None => Err(Error::Api(token_reply.error.unwrap_or(types::Error {
                message: "neither token nor response have been set".to_string(),
            }))),
        },
        Err(e) => Err(e),
    }
}

pub async fn load_user(user_id: Option<String>, as_user: Option<String>) -> Result<UserInfo> {
    let mut url = format!("{}/users", DEFAULT_API_URL,);
    if let Some(user_id) = user_id {
        url.push_str(&format!("/{}", user_id));
        if let Some(user_id) = as_user {
            url.push_str(&format!("?as={}", user_id));
        }
    }

    let user_reply: types::Reply<Vec<DBReply<Vec<UserInfo>>>> =
        into_json(Request::get(&url).send().await?).await?;
    match user_reply.content {
        Some(user_in_db) => match user_in_db.get(0).unwrap() {
            DBReply::OK { time: _, result } => {
                let user = result.get(0).unwrap().to_owned();

                return Ok(user);
            }
            DBReply::ERR { time: _, detail } => {
                return Err(Error::Api(types::Error {
                    message: detail.to_owned(),
                }))
            }
        },
        None => {
            return Err(Error::Api(user_reply.error.unwrap_or(types::Error {
                message: "neither token nor response have been set".to_string(),
            })))
        }
    }
}

pub async fn delete_user(user_id: String, token: ApiToken) -> Result<Reply<Value>> {
    let url = format!("{}/users/{}", DEFAULT_API_URL, user_id);
    log::debug!("url: {}", url);

    into_json(
        Request::delete(&url)
            .header("x-token", &token.token)
            .send()
            .await?,
    )
    .await
}

pub async fn load_followers(user_id: String, offset: Option<usize>) -> Result<Vec<UserInfo>> {
    let offset = match offset {
        Some(offset) => offset,
        None => 0,
    };
    let url = format!(
        "{}/users/{}/followers?offset={}",
        DEFAULT_API_URL, user_id, offset
    );
    log::debug!("url: {}", url);

    let res = Request::get(&url).send().await?;

    let user_reply: Reply<Vec<DBReply<Vec<FollowerReply>>>> = into_json(res).await?;

    match user_reply.content {
        Some(user_in_db) => match user_in_db.get(0).unwrap() {
            DBReply::OK { time: _, result } => Ok(result.get(0).unwrap().followers.to_owned()),
            DBReply::ERR { time: _, detail } => {
                return Err(Error::Api(types::Error {
                    message: detail.to_owned(),
                }))
            }
        },
        None => {
            return Err(Error::Api(user_reply.error.unwrap_or(types::Error {
                message: "could not receive followers".to_string(),
            })))
        }
    }
}

pub async fn load_following(user_id: String, offset: Option<usize>) -> Result<Vec<UserInfo>> {
    let offset = match offset {
        Some(offset) => offset,
        None => 0,
    };
    let url = format!(
        "{}/users/{}/following?offset={}",
        DEFAULT_API_URL, user_id, offset
    );
    log::debug!("url: {}", url);

    let res = Request::get(&url).send().await?;

    let user_reply: Reply<Vec<DBReply<Vec<FollowingReply>>>> = into_json(res).await?;

    match user_reply.content {
        Some(user_in_db) => match user_in_db.get(0) {
            Some(reply) => match reply {
                DBReply::OK { time: _, result } => Ok(result.get(0).unwrap().following.to_owned()),
                DBReply::ERR { time: _, detail } => {
                    return Err(Error::Api(types::Error {
                        message: detail.to_owned(),
                    }))
                }
            },
            None => {
                return Err(Error::Api(types::Error {
                    message: "no new followers".to_string(),
                }))
            }
        },
        None => {
            return Err(Error::Api(user_reply.error.unwrap_or(types::Error {
                message: "could not receive followers".to_string(),
            })))
        }
    }
}

pub async fn follow_user(user_id: String, token: ApiToken) -> Result<Reply<Vec<DBReply<Value>>>> {
    let url = format!("{}/users/{}/follow", DEFAULT_API_URL, user_id);
    log::debug!("url: {}", url);

    into_json(
        Request::post(&url)
            .header("x-token", &token.token)
            .send()
            .await?,
    )
    .await
}

pub async fn unfollow_user(user_id: String, token: ApiToken) -> Result<Reply<Vec<DBReply<Value>>>> {
    let url = format!("{}/users/{}/unfollow", DEFAULT_API_URL, user_id);
    log::debug!("url: {}", url);

    into_json(
        Request::post(&url)
            .header("x-token", &token.token)
            .send()
            .await?,
    )
    .await
}

pub async fn load_post(
    post_id: Option<String>,
    parent: Option<String>,
    after_timestamp: Option<String>,
    usertoken: Option<ApiToken>,
) -> Result<Vec<Post>> {
    let mut url = format!("{}/posts", DEFAULT_API_URL,);
    if let Some(post_id) = post_id {
        url.push_str(&format!("/{}", post_id));
    }
    url.push('?');

    if let Some(parent) = parent {
        url.push_str(&format!("&parent={}", parent));
    }
    if let Some(after) = after_timestamp {
        url.push_str(&format!("&after={}", after));
    }

    if let Some(token) = usertoken.clone() {
        url.push_str(&format!("&as={}", token.id()));
    }

    let req = match usertoken {
        Some(token) => Request::get(&url).header("x-token", &token.token),
        None => Request::get(&url),
    };

    let post_reply: types::Reply<DBReply<Vec<Post>>> = into_json(req.send().await?).await?;
    match post_reply.content {
        Some(dbreply) => match dbreply {
            DBReply::OK { time: _, result } => {
                return Ok(result);
            }
            DBReply::ERR { time: _, detail } => {
                return Err(Error::Api(types::Error {
                    message: detail.to_owned(),
                }))
            }
        },
        None => {
            return Err(Error::Api(post_reply.error.unwrap_or(types::Error {
                message: "An unknown error occured".to_string(),
            })))
        }
    }
}

pub async fn post_impression(
    post_id: String,
    usertoken: ApiToken,
    positive: bool,
    reset: bool,
) -> Result<Post> {
    let mut url = format!(
        "{}/posts/{}/{}",
        DEFAULT_API_URL,
        post_id,
        if positive { "like" } else { "dislike" }
    );
    if reset {
        url.push_str("?reset");
    }

    let req = Request::post(&url).header("x-token", &usertoken.token);

    let res = req.send().await.unwrap();
    let post_reply: types::Reply<DBReply<Vec<Post>>> = into_json(res).await.unwrap();
    match post_reply.content {
        Some(dbrepl) => match dbrepl {
            DBReply::OK { time: _, result } => {
                return Ok(result.get(0).unwrap().to_owned());
            }
            DBReply::ERR { time: _, detail } => {
                return Err(Error::Api(types::Error {
                    message: detail.to_owned(),
                }));
            }
        },
        None => {
            return Err(Error::Api(post_reply.error.unwrap_or(types::Error {
                message: "An unknown error occured".to_string(),
            })))
        }
    }
}

pub async fn edit_post(post_id: String, usertoken: ApiToken, content: String) -> Result<Post> {
    let url = format!("{}/posts/{}", DEFAULT_API_URL, post_id);

    let req = Request::patch(&url)
        .header("x-token", &usertoken.token)
        .json(&json!({ "message": content }))?;

    let res: types::Reply<DBReply<Vec<Post>>> = into_json(req.send().await?).await?;
    match res.content {
        Some(dbreply) => match dbreply {
            DBReply::OK { time: _, result } => {
                return Ok(result.get(0).unwrap().to_owned());
            }
            DBReply::ERR { time: _, detail } => {
                return Err(Error::Api(types::Error {
                    message: detail.to_owned(),
                }))
            }
        },
        None => {
            return Err(Error::Api(res.error.unwrap_or(types::Error {
                message: "An unknown error occured".to_string(),
            })))
        }
    }
}

pub async fn create_post(
    usertoken: Option<ApiToken>,
    parent_id: Option<String>,
    content: String,
) -> Result<Post> {
    let mut url = format!("{}/posts", DEFAULT_API_URL);
    if let Some(p_id) = parent_id {
        url.push_str(&format!("?parent={}", p_id));
    }

    let mut req = Request::post(&url);

    if let Some(token) = usertoken {
        req = req.header("x-token", &token.token);
    }
    req = req.json(&json!({ "message": content }))?;

    let res: types::Reply<DBReply<Vec<Post>>> = into_json(req.send().await?).await?;
    match res.content {
        Some(dbreply) => match dbreply {
            DBReply::OK { time: _, result } => {
                return Ok(result.get(0).unwrap().to_owned());
            }
            DBReply::ERR { time: _, detail } => {
                return Err(Error::Api(types::Error {
                    message: detail.to_owned(),
                }))
            }
        },
        None => {
            return Err(Error::Api(res.error.unwrap_or(types::Error {
                message: "An unknown error occured".to_string(),
            })))
        }
    }
}

pub async fn delete_post(post_id: String, usertoken: ApiToken) -> Result<types::EmptyReply> {
    let url = format!("{}/posts/{}", DEFAULT_API_URL, post_id);

    let req = Request::delete(&url).header("x-token", &usertoken.token);

    into_json(req.send().await?).await
}

pub type Result<T> = std::result::Result<T, Error>;

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
