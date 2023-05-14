use leptos::*;

use crate::{
    api,
    types::{ApiToken, Post},
};

#[component]
pub fn CreatePost<F>(
    cx: Scope,
    token: ReadSignal<Option<ApiToken>>,
    parent: Option<Post>,
    on_close: F,
) -> impl IntoView
where
    F: Fn() + 'static + Clone,
{
    let (creation_error, set_creation_error) = create_signal(cx, None::<String>);
    let (wait_for_response, set_wait_for_response) = create_signal(cx, false);
    let (cont, set_cont) = create_signal(cx, "".to_string());

    let parent_id = match &parent {
        Some(p) => Some(p.id.clone()),
        None => None,
    };

    let parent_message = match parent {
        Some(p) => Some(p.message),
        None => None,
    };

    let new_post_action = create_action(cx, move |content: &String| {
        let on_close = on_close.clone();
        let content = content.clone();
        let parent_id = parent_id.clone();

        async move {
            set_wait_for_response.set(true);
            if content.is_empty() {
                on_close();
                return;
            }
            let req = api::create_post(token.get(), parent_id, content);
            let result = req.await;
            set_wait_for_response.set(false);
            match result {
                Ok(_) => {
                    on_close();
                }
                Err(err) => {
                    let msg = match err {
                        api::Error::Fetch(js_err) => {
                            format!("{js_err:?}")
                        }
                        api::Error::Api(err) => err.message,
                    };
                    error!("Unable to create new post");
                    set_creation_error.set(Some(msg));
                }
            }
        }
    });

    view! { cx,
      <div class="new-post-window">
        <button class="new-post-close-button" on:click=move |_| {
          new_post_action.dispatch("".to_string());
        }>"X"</button>
        {
            move || if parent_message.is_some() { view!{ cx,
                <>
                    <p>"Responding to post"</p>
                    <blockquote>{parent_message.clone().unwrap()}</blockquote>
                </>
            }} else {
                view!{cx, <> </>}
            }
        }
        <textarea id="new-post-textarea" placeholder="What's on your mind?" on:keyup=move |e| set_cont.set(event_target_value(&e))></textarea>
        {move || creation_error.get().map(|err| view!{ cx,
          <p style ="color:red;" >{ err }</p>
        })}
        <button
          class="new-post-button"
          disabled=move || wait_for_response.get()
          on:click=move |_| {

            new_post_action.dispatch(cont.get());
          }>{if wait_for_response.get() {"processing"} else {"Post"}}</button>
      </div>
    }
}
