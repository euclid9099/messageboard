use crate::{
    api,
    types::{ApiToken, Post},
};
use leptos::*;

#[component]
pub fn PostView(cx: Scope, post: Post, token: ReadSignal<Option<ApiToken>>) -> impl IntoView {
    let (loading_error, set_loading_error) = create_signal(cx, None::<String>);
    let (wait_for_response, set_wait_for_response) = create_signal(cx, false);
    let (wait_for_reload, set_wait_for_reload) = create_signal(cx, false);
    let (posts, set_posts) = create_signal(cx, Vec::<Post>::new());
    let (self_post, set_self_post) = create_signal(cx, post.clone());
    let (edit, set_edit) = create_signal(cx, false);

    let reload_action = create_action(cx, move |()| {
        set_wait_for_reload.update(|v| *v = true);
        let token = token.clone();
        async move {
            let res = api::load_post(Some(self_post.get().id), None, None, token.get()).await;

            match res {
                Ok(res) => {
                    set_loading_error.set(None);
                    set_self_post.set(res.get(0).unwrap().clone());
                }
                Err(err) => {
                    set_loading_error.set(Some(err.to_string()));
                }
            }

            set_wait_for_reload.update(|v| *v = false);
        }
    });

    let impression_action = create_action(cx, move |(positive, reset): &(bool, bool)| {
        set_wait_for_reload.update(|v| *v = true);
        let fut = api::post_impression(self_post.get().id, token.get().unwrap(), *positive, *reset);
        async move {
            let res = fut.await;

            match res {
                Ok(res) => {
                    set_loading_error.update(|e| *e = None);
                    set_self_post.update(|p| *p = res.clone());
                }
                Err(err) => {
                    log::debug!("loading error: {}", err);
                    set_loading_error.update(|e| *e = Some(err.to_string()));
                }
            }

            set_wait_for_reload.set(false);
        }
    });

    let load_children_action = create_action(cx, move |()| {
        set_wait_for_response.set(true);
        let latest_post_time = match posts.get().last() {
            Some(p) => p.time.to_rfc3339(),
            None => "0".to_string(),
        };
        let token = token.clone();
        async move {
            let res = api::load_post(
                None,
                Some(self_post.get().id),
                Some(latest_post_time),
                token.get(),
            )
            .await;

            match res {
                Ok(res) => {
                    set_loading_error.set(None);
                    set_posts.update(|p| p.extend(res));
                }
                Err(err) => {
                    set_loading_error.update(|e| *e = Some(err.to_string()));
                }
            }

            set_wait_for_response.update(|v| *v = false);
        }
    });

    let submit_edit_action = create_action(cx, move |content: &String| {
        set_wait_for_reload.set(true);
        let fut = api::edit_post(self_post.get().id, token.get().unwrap(), content.clone());
        async move {
            let res = fut.await;

            match res {
                Ok(res) => {
                    set_loading_error.update(|e| *e = None);
                    set_self_post.update(|p| *p = res.clone());
                }
                Err(err) => {
                    log::debug!("loading error: {}", err);
                    set_loading_error.update(|e| *e = Some(err.to_string()));
                }
            }

            set_wait_for_reload.set(false);
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
                    <button
                        disabled=move|| wait_for_reload.get()
                        on:click=move|_| reload_action.dispatch(())>
                        "reload post"
                    </button>
                    <p>{format!("{}", post.time.format("%d. %b %Y at %k:%M"))}</p>
                </div>
                {move || if token.get().is_some() && self_post.get().author.is_some() && self_post.get().author.unwrap().id == token.get().unwrap().id() {
                    view! {cx, <div class="post-body">
                        <p id=self_post.get().id contenteditable=move || if edit.get() {"true"} else {"false"}>{move || self_post.get().message}</p>
                        <button
                            disabled=move || wait_for_reload.get()
                            on:click=move |_| {
                                set_edit.set(!edit.get());
                                if edit.get() {
                                    log::debug!("start edit");
                                } else {
                                    log::debug!("save new content");
                                    match leptos_dom::document().get_element_by_id(&self_post.get().id) {
                                        Some(el) => {
                                            let new_content = el.text_content().unwrap_or("".to_string());
                                            log::debug!("{}", new_content);
                                            submit_edit_action.dispatch(new_content);
                                        }
                                        None => log::debug!("no element found"),
                                    }
                                }
                            }>
                            {move || if edit.get() {"save"} else {"edit"}}
                        </button>
                    </div>}
                } else {
                    view! {cx, <div class="post-body">
                        <p>{move || self_post.get().message}</p>
                    </div>}
                }}
                <div class="post-controls">
                    <button
                        disabled=move || token.get().is_none() || wait_for_reload.get()
                        on:click=move |_| impression_action.dispatch((true, self_post.get().liked.unwrap()))>
                        <i class=move || if self_post.get().liked.unwrap_or(false) {"material-icons selected positive"} else {"material-icons"}>"thumb_up"</i>
                        <p>{move|| self_post.get().likes}</p>
                    </button>
                    <button
                        disabled=move || token.get().is_none() || wait_for_reload.get()
                        on:click=move |_| impression_action.dispatch((false, self_post.get().disliked.unwrap()))>
                        <i class=move || if self_post.get().disliked.unwrap_or(false) {"material-icons selected negative"} else {"material-icons"}>"thumb_down"</i>
                        <p>{move|| self_post.get().dislikes}</p>
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
                {move || if wait_for_response.get() {
                    view!{ cx,
                        <div>
                            <p>"Loading..."</p>
                        </div>
                    }
                } else {
                    view!{ cx,
                        <div>
                            <button
                                class="load-responses-button"
                                disabled=move|| wait_for_response.get()
                                on:click=move|_| load_children_action.dispatch(())>
                                "Load responses (" {post.responses} ")"
                            </button>
                        </div>
                    }
                }
            }
            </div>
        </div>
    }
}
