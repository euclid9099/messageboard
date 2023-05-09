use gloo_net::http::Request;
use leptos::*;
use serde_json::Value;

use crate::{
    api::{self},
    components::*,
    types::{ApiToken, Post},
};

#[component]
pub fn Posts(cx: Scope, token: ReadSignal<Option<ApiToken>>) -> impl IntoView {
    let (loading_error, set_loading_error) = create_signal(cx, None::<String>);
    let (wait_for_response, set_wait_for_response) = create_signal(cx, false);
    let (posts, set_posts) = create_signal(cx, Vec::<Post>::new());

    let load_posts_action = create_action(cx, move |()| {
        let latest_post_time = match posts.get().last() {
            Some(p) => p.time.to_rfc3339(),
            None => "0".to_string(),
        };
        async move {
            set_wait_for_response.update(|v| *v = true);

            let res = api::load_post(None, None, Some(latest_post_time), token.get()).await;

            match res {
                Ok(res) => {
                    set_loading_error.update(|e| *e = None);
                    set_posts.update(|p| p.extend(res));
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
                view=move |cx, p: Post| {
                    view! { cx,
                        <div class="tl-post">
                        <PostView post=p token=token/>
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
