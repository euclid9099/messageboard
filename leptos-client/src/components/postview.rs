use crate::{
    api,
    pages::Page,
    types::{ApiToken, Post, UserInfo},
};
use leptos::*;
use leptos_router::{AProps, A};

#[component]
pub fn PostView(
    cx: Scope,
    as_user: ReadSignal<Option<UserInfo>>,
    post: Post,
    token: Signal<Option<ApiToken>>,
    new_post_overlay: WriteSignal<Option<Option<Post>>>,
    sibling_list: WriteSignal<Vec<Post>>,
) -> impl IntoView {
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
                    set_loading_error.update(|e| *e = Some(err.to_string()));
                }
            }

            set_wait_for_reload.set(false);
        }
    });

    let delete_self_action = create_action(cx, move |()| {
        set_wait_for_reload.set(true);
        let fut = api::delete_post(self_post.get().id, token.get().unwrap());
        async move {
            let res = fut.await;

            match res {
                Ok(res) => match res.message.as_str() {
                    "ok" => {
                        sibling_list.update(|p| {
                            p.retain(|p| p.id != self_post.get().id);
                        });
                    }
                    m => log::debug!("database deletion failed: {}", m),
                },
                Err(err) => {
                    set_loading_error.set(Some(err.to_string()));
                }
            }

            set_wait_for_reload.set(false);
        }
    });

    view! {cx,
        <div class="post">
            <div class="post-content">
                <div class="post-header">
                    {match post.author {
                        Some(u) => view!{cx,
                            <><A href=Page::User.path(Some(u.id))><img src={u.profile_picture}/>{u.username.unwrap_or("error loading username".to_string())}</A></>
                        },
                        None => view!{cx, <><p>"anonymous"</p></>}
                    }}
                    <button
                        class="reload-button"
                        disabled=move|| wait_for_reload.get()
                        on:click=move|_| reload_action.dispatch(())>
                        <span class="material-symbols-outlined">
                        "sync"
                        </span>
                        <p>"reload"</p>
                    </button>
                    <p class="date">{format!("{}", post.time.format("%d. %b %Y at %k:%M"))}</p>
                </div>
                <div class="post-body">
                    <p id=self_post.get().id contenteditable=move || if edit.get() {"true"} else {"false"}>{move || self_post.get().message}</p>
                    <div class="reactions">
                {move || if token.get().is_some() {
                    if self_post.get().author.is_some() && self_post.get().author.unwrap().id == token.get().unwrap().id() {
                        view! {cx,
                            <>
                                <button
                                    disabled=move || wait_for_reload.get()
                                    on:click=move |_| {
                                        set_edit.set(!edit.get());
                                        if edit.get() {

                                        } else {

                                            match leptos_dom::document().get_element_by_id(&self_post.get().id) {
                                                Some(el) => {
                                                    let new_content = el.text_content().unwrap_or("".to_string());

                                                    if new_content != self_post.get().message {
                                                        submit_edit_action.dispatch(new_content);
                                                    }
                                                }
                                                None => log::debug!("no element found"),
                                            }
                                        }
                                    }>
                                    {move || if edit.get() {view! {cx,
                                        <span class="material-symbols-outlined">
                                        "save"
                                        </span>
                                        <p>"save"</p>
                                    }} else {view! {cx,
                                        <span class="material-symbols-outlined">
                                        "edit"
                                        </span>
                                        <p>"edit"</p>
                                    }}}
                                </button>
                                <button
                                    disabled=move || wait_for_reload.get()
                                    on:click=move |_| {
                                        delete_self_action.dispatch(());
                                    }>
                                    <span class="material-symbols-outlined">
                                    "delete"
                                    </span>
                                    <p>"delete"</p>
                                </button>
                            </>
                        }
                    } else if as_user.get().is_some() && as_user.get().unwrap().admin {
                        view! {cx,
                            <>
                                <button
                                    disabled=move || wait_for_reload.get()
                                    on:click=move |_| {
                                        delete_self_action.dispatch(());
                                    }>
                                    <span class="material-symbols-outlined">
                                    "delete"
                                    </span>
                                    <p>"delete"</p>
                                </button>
                            </>
                        }
                    } else {
                        view! {cx, <></>}
                    }

                } else {
                    view! {cx, <></>}
                }}
                </div>
                </div>
                <div class="post-controls">
                    <button
                        disabled=move || token.get().is_none() || wait_for_reload.get()
                        on:click=move |_| impression_action.dispatch((true, self_post.get().liked.unwrap()))>
                        <span class=move || if self_post.get().liked.unwrap_or(false) {"material-symbols-outlined selected positive"} else {"material-symbols-outlined"}>"thumb_up"</span>
                        <p>{move|| self_post.get().likes}</p>
                    </button>
                    <button
                        disabled=move || token.get().is_none() || wait_for_reload.get()
                        on:click=move |_| impression_action.dispatch((false, self_post.get().disliked.unwrap()))>
                        <span class=move || if self_post.get().disliked.unwrap_or(false) {"material-symbols-outlined selected negative"} else {"material-symbols-outlined"}>"thumb_down"</span>
                        <p>{move|| self_post.get().dislikes}</p>
                    </button>
                    <button
                        on:click=move |_| new_post_overlay.set(Some(Some(self_post.get())))>
                        <span class="material-symbols-outlined">
                        "add_comment"
                        </span>
                        <p>"respond"</p>
                    </button>
                </div>
                {move || loading_error.get().map(|err| view!{ cx,
                    <p style ="color:red;" >{ err }</p>
                  })
                }
            </div>
            <div class="post-children">
                <For
                    each=move|| posts.get()
                    key=|post| post.id.clone()
                    view=move |cx, p: Post| {
                        view! { cx,
                            <PostView as_user=as_user post=p token=token new_post_overlay=new_post_overlay sibling_list=set_posts/>
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
                                <span class="material-symbols-outlined">
                                "forum"
                                </span>
                                <p>"responses (" {self_post.get().responses} ")"</p>
                            </button>
                        </div>
                    }
                }
            }
            </div>
        </div>
    }
}
