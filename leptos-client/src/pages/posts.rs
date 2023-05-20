use leptos::*;

use crate::{
    api::{self},
    components::*,
    types::{ApiToken, Post, UserInfo},
};

#[component]
pub fn Posts(
    cx: Scope,
    user: ReadSignal<Option<UserInfo>>,
    token: Signal<Option<ApiToken>>,
) -> impl IntoView {
    let (loading_error, set_loading_error) = create_signal(cx, None::<String>);
    let (wait_for_response, set_wait_for_response) = create_signal(cx, false);
    let (posts, set_posts) = create_signal(cx, Vec::<Post>::new());
    let (create_post_overlay, set_create_post_overlay) = create_signal(cx, None::<Option<Post>>);

    let load_posts_action = create_action(cx, move |()| {
        let latest_post_time = match posts.get().last() {
            Some(p) => format!("{}", p.time.format("%Y-%m-%dT%H:%M:%S%.fZ")),
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
        <button class="create-post-button"
            on:click=move |_| set_create_post_overlay.set(Some(None))>"Create post"</button>
        {move || if create_post_overlay.get().is_some() { view!{ cx,
            <>
                <CreatePost
                    token=token
                    on_close=move || {

                        set_create_post_overlay.set(None);
                        load_posts_action.dispatch(());}
                    parent=create_post_overlay.get().unwrap()
                ></CreatePost>
            </>
          }
        } else {
            view!{cx, <> </>}
        }
        }
        <div class="post-list">
            <For
                each=move|| posts.get()
                key=|post| post.id.clone()
                view=move |cx, p: Post| {
                    view! { cx,
                        <div class="tl-post">
                        <PostView as_user=user post=p token=token new_post_overlay=set_create_post_overlay
                            sibling_list=set_posts/>
                        </div>
                    }
                }
            />
        </div>
        {move || loading_error.get().map(|err| view!{ cx,
            <p style ="color:red;" >{ err }</p>
          })
        }
        {move || if wait_for_response.get() {
            view!{ cx,
                <div>
                    <p>"Loading..."</p>
                </div>
            }
        } else {
            view!{ cx,
                <div>
                    <button on:click=move|_| load_posts_action.dispatch(()) enabled=move || wait_for_response.get()>"Load posts"</button>
                </div>
            }
        }
    }
    }
}
