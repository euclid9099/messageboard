use gloo_net::http::Request;
use leptos::*;
use serde_json::Value;
use urlencoding::encode;

use crate::{
    api::{self, ApiTypes::*, Result},
    types::{DBReply, Post, Reply},
    DEFAULT_API_URL,
};

#[component]
pub fn Posts(cx: Scope, api: api::ApiTypes) -> impl IntoView {
    let (loading_error, set_loading_error) = create_signal(cx, None::<String>);
    let (wait_for_response, set_wait_for_response) = create_signal(cx, false);
    let (posts, set_posts) = create_signal(cx, Vec::<Post>::new());

    let load_posts_action = create_action(cx, move |_| {
        let api = api.clone();
        let latest_post_time = match posts.get().last() {
            Some(p) => p.time.to_rfc3339(),
            None => "0".to_string(),
        };
        async move {
            set_wait_for_response.update(|v| *v = true);
            let res: Result<Reply<DBReply<Vec<Post>>>> = match &api {
                Authorized(api) => {
                    let url = format!(
                        "{}/posts?after={}&as={}",
                        DEFAULT_API_URL,
                        encode(&latest_post_time),
                        encode(
                            &api.token().clone().body_as_object::<Value>().unwrap()["ID"]
                                .as_str()
                                .unwrap()
                        )
                    );
                    api.send(Request::get(&url)).await
                }
                Unauthorized(api) => {
                    let url = format!(
                        "{}/posts?after={}",
                        DEFAULT_API_URL,
                        encode(&latest_post_time)
                    );
                    api.send(Request::get(&url)).await
                }
            };

            match res {
                Ok(res) => {
                    if let Some(db) = res.content {
                        set_loading_error.update(|e| *e = None);
                        set_posts.update(|p| p.extend(db.result))
                    }
                }
                Err(err) => {
                    set_loading_error.update(|e| *e = Some(err.to_string()));
                }
            }

            set_wait_for_response.update(|v| *v = false);
        }
    });

    view! {cx,
        <h1>"Posts"</h1>
        <div>
            <For
                each=move|| posts.get()
                key=|post| post.id.clone()
                view=move |cx, post: Post| {
                    view! { cx,
                        <div class="post">
                            <div class="post-header">
                                <p>{match post.author {
                                    Some(user) => user.username.unwrap_or("error loading username".to_string()),
                                    None => "anonymous".to_string()}
                                }</p>
                                <p>{post.time.to_string()}</p>
                            </div>
                            <p>{post.message}</p>
                        </div>
                    }
                }
            />
        </div>
        {move || loading_error.get().map(|err| view!{ cx,
            <p style ="color:red;" >{ err }</p>
          })
        }
        <button on:click=move|_| load_posts_action.dispatch(()) prop:enabled=move|| false>"Load posts"</button>
    }
}
