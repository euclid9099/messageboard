use std::collections::BTreeSet;

use crate::{
    api,
    components::postview::*,
    pages::Page,
    types::{self, ApiToken, Post, UserInfo},
};
use leptos::*;
use leptos_router::*;

#[component]
pub fn User(
    cx: Scope,
    self_info: ReadSignal<Option<UserInfo>>,
    self_token: RwSignal<Option<ApiToken>>,
) -> impl IntoView {
    let (user_info, set_user_info) = create_signal(cx, None::<types::UserInfo>);
    let (followers, set_followers) = create_signal(cx, Vec::<types::UserInfo>::new());
    let (following, set_following) = create_signal(cx, Vec::<types::UserInfo>::new());
    let params = use_params_map(cx);

    let user_id = move || params.with(|params| params.get("user").unwrap().clone());
    if user_id() == "self" {
        match self_info.get() {
            Some(self_user) => {
                let navigate = use_navigate(cx);
                let _ = navigate(
                    &Page::User.path(Some(self_user.id.clone())),
                    Default::default(),
                );
            }
            None => {
                log::error!("can't display self page on anonymous user");
            }
        }
    }

    let fetch_user_info = create_action(cx, move |()| {
        let req = api::load_user(
            Some(user_id()),
            match self_info.get() {
                Some(self_user) => Some(self_user.id.clone()),
                None => None,
            },
        );

        async move {
            if let Some(self_user) = self_info.get() {
                if self_user.id == user_id() || user_id() == "self" {
                    log::debug!("user is self");
                    set_user_info.set(Some(self_user));
                    return;
                }
            }
            let res = req.await;
            match res {
                Ok(res) => {
                    log::debug!("user info loaded");
                    set_user_info.set(Some(res));
                }
                Err(err) => {
                    log::error!("Failed to load user info: {}", err);
                }
            }
        }
    });

    let delete_user = create_action(cx, move |()| {
        let req = api::delete_user(user_id(), self_token.get().unwrap());
        async move {
            match window().confirm_with_message(
                "are you sure you want to delete this account?\nTHIS ACTION IS IRREVERSIBLE!",
            ) {
                Ok(true) => {}
                _ => {
                    return;
                }
            };
            let res = req.await;
            match res {
                Ok(res) => match res.error {
                    Some(err) => {
                        log::error!("Failed to delete user: {:?}", err);
                    }
                    None => {
                        log::debug!("user deleted: {:?}", res);
                        if self_info.get().unwrap().id == user_id() {
                            self_token.set(None);
                        }
                        let _ = use_navigate(cx)(&Page::Home.path(None), Default::default());
                    }
                },
                Err(err) => {
                    log::error!("Failed to delete user: {}", err);
                }
            }
        }
    });

    let fetch_followers = create_action(cx, move |()| {
        let req = api::load_followers(user_id(), Some(followers.get().len()));

        async move {
            let res = req.await;
            match res {
                Ok(res) => {
                    log::debug!("followers loaded");
                    set_followers.update(|f| f.extend(res));
                }
                Err(err) => {
                    log::error!("Failed to load followers: {}", err);
                }
            }
        }
    });

    let fetch_following = create_action(cx, move |()| {
        let req = api::load_following(user_id(), Some(following.get().len()));

        async move {
            let res = req.await;
            match res {
                Ok(res) => {
                    log::debug!("followers loaded");
                    set_following.update(|f| f.extend(res));
                }
                Err(err) => {
                    log::error!("Failed to load followers: {}", err);
                }
            }
        }
    });

    let follow_or_unfollow = create_action(cx, move |()| async move {
        if self_token.get().is_none() || self_info.get().is_none() {
            return;
        }
        if user_info.get().unwrap().you_follow {
            match api::unfollow_user(user_id(), self_token.get().unwrap()).await {
                Ok(_) => {
                    log::debug!("unfollowed user");
                    set_user_info.update(|info| {
                        if let Some(info) = info {
                            info.you_follow = false;
                        }
                    });
                }
                Err(err) => {
                    log::error!("failed to unfollow user: {}", err);
                }
            };
        } else {
            match api::follow_user(user_id(), self_token.get().unwrap()).await {
                Ok(_) => {
                    log::debug!("followed user");
                    set_user_info.update(|info| {
                        if let Some(info) = info {
                            info.you_follow = true;
                        }
                    });
                }
                Err(err) => {
                    log::error!("failed to follow user: {}", err);
                }
            };
        };
    });

    create_effect(cx, move |_| {
        log::debug!("loading user ID: {}", user_id());
        if user_info.get().is_some() && user_id() == user_info.get().unwrap().id {
            return;
        }
        fetch_user_info.dispatch(());
        set_followers.update(|f| f.clear());
        set_following.update(|f| f.clear());
    });

    view! { cx,
        <Show
            when = move || user_info.get().is_some()
            fallback = |cx| view! { cx,
                <p>"Loading user info..."</p>
            }
        >
            <div class=move || {if user_info.get().unwrap().admin {"user_info admin_user"} else {"user_info"}}>
                <img src=move || user_info.get().unwrap().profile_picture />
                <h3>{user_info.get().unwrap().username}</h3>
                <p>{user_info.get().unwrap().about}</p>
                <div class="user_controls">
                    {if self_info.get().is_some() && self_info.get().unwrap().id != user_id() {
                        view! {cx, <>
                            <button
                                on:click=move|_| follow_or_unfollow.dispatch(())
                            >
                                <p>{if user_info.get().unwrap().you_follow {"unfollow"} else {"follow"}}</p>
                            </button>
                        </>}
                    } else {
                        view! {cx, <></>}
                    }}
                    {if self_info.get().is_some() && (self_info.get().unwrap().id == user_id() || self_info.get().unwrap().admin) {
                        view! {cx, <>
                            <button
                                on:click=move|_| delete_user.dispatch(())
                            >
                                <p>"delete"</p>
                            </button>
                        </>}
                    } else {
                        view! {cx, <></>}
                    }}
                </div>
            </div>
            <div class="user_followers">
                <p>"Followers:"</p>
                <ul>
                    <For
                        each=move || followers.get()
                        key=move |f| f.id.clone()
                        view=move |cx, f: UserInfo| {
                            view! { cx,
                                <A
                                    href=move || Page::User.path(Some(f.id.clone()))
                                    class="user_badge"
                                >
                                    <img src=f.profile_picture />
                                    <p>{f.username}</p>
                                </A>
                            }
                        }
                    />
                    <button on:click=move|_| fetch_followers.dispatch(())><p>"load more"</p></button>
                </ul>
            </div>
            <div class="user_following">
                <p>"Following:"</p>
                <ul>
                    <For
                        each=move || following.get()
                        key=move |f| f.id.clone()
                        view=move |cx, f: UserInfo| {
                            view! { cx,
                                <A
                                    href=move || Page::User.path(Some(f.id.clone()))
                                    class="user_badge"
                                >
                                    <img src=f.profile_picture />
                                    <p>{f.username}</p>
                                </A>
                            }
                        }
                    />
                    <button on:click=move|_| fetch_following.dispatch(())><p>"load more"</p></button>
                </ul>
            </div>
        </Show>
    }
}
