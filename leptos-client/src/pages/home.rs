use crate::{types::UserInfo, Page};
use leptos::*;
use leptos_router::*;

#[component]
pub fn Home(cx: Scope, user_info: ReadSignal<Option<UserInfo>>) -> impl IntoView {
    view! { cx,
      <h2>"Welcome to "</h2>
      {move || match user_info.get() {
        Some(info) => view!{ cx,
          <p>"You are logged in with "{ info.username }"."</p>
        }.into_view(cx),
        None => view!{ cx,
          <p>"You are not logged in."</p>
          <A href=Page::Login.path(None) >"Login now."</A>
        }.into_view(cx)
      }}
    }
}
