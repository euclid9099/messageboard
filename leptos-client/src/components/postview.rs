use std::rc::Rc;

use crate::{
    api,
    types::{ApiToken, Post},
};
use leptos::*;
use serde_json::Value;

#[component]
pub fn PostView(cx: Scope, post: Post, token: ReadSignal<Option<ApiToken>>) -> impl IntoView {
    let (loading_error, set_loading_error) = create_signal(cx, None::<String>);
    let (wait_for_response, set_wait_for_response) = create_signal(cx, false);
    let (posts, set_posts) = create_signal(cx, Vec::<Post>::new());
    let (self_post, set_self_post) = create_signal(cx, post.clone());

    let reload_action = create_action(cx, move |()| {
        let token = token.clone();
        async move {
            set_wait_for_response.update(|v| *v = true);

            let res = api::load_post(Some(self_post.get().id), None, None, token.get()).await;

            match res {
                Ok(res) => {
                    set_loading_error.update(|e| *e = None);
                    set_self_post.update(|p| *p = res.get(0).unwrap().clone());
                }
                Err(err) => {
                    set_loading_error.update(|e| *e = Some(err.to_string()));
                }
            }

            set_wait_for_response.update(|v| *v = false);
        }
    });

    let load_children_action = create_action(cx, move |()| {
        let latest_post_time = match posts.get().last() {
            Some(p) => p.time.to_rfc3339(),
            None => "0".to_string(),
        };
        let token = token.clone();
        async move {
            set_wait_for_response.update(|v| *v = true);

            let res = api::load_post(
                None,
                Some(self_post.get().id),
                Some(latest_post_time),
                token.get(),
            )
            .await;

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
        <div class="post">
            <div class="post-content">
                <div class="post-header">
                    <p>{match post.author {
                        Some(u) => u.username.unwrap_or("error loading username".to_string()),
                        None => "anonymous".to_string()
                    }}</p>
                    <p>{post.time.to_rfc3339()}</p>
                </div>
                <div class="post-body">
                    <p>{post.message}</p>
                </div>
                <div class="post-controls">
                    <button
                        props:disabled=move|| wait_for_response.get()
                        on:click=move|_| load_children_action.dispatch(())>
                        "Load responses"
                    </button>
                    <button
                        props:disabled=move|| wait_for_response.get()
                        on:click=move|_| reload_action.dispatch(())>
                        "reload post"
                    </button>
                </div>
            </div>
            <div class="post-children">
                <For
                    each=move|| posts.get()
                    key=|post| post.id.clone()
                    view=move |cx, p: Post| {
                        view! { cx,
                            <PostView post=p token=token/>
                        }
                    }
                />
            </div>
        </div>
    }
}
