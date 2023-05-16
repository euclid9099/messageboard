use leptos::*;
use leptos_router::*;

use crate::Page;

#[component]
pub fn NavBar<F>(
    cx: Scope,
    logged_in: Signal<bool>,
    on_logout: F,
    darkmode: RwSignal<bool>,
) -> impl IntoView
where
    F: Fn() + 'static + Clone,
{
    view! { cx,
      <nav>
        <Show
          when = move || logged_in.get()
          fallback = |cx| view! { cx,
            <A href=Page::Login.path() >"Login"</A>
            " | "
            <A href=Page::Register.path() >"Register"</A>
          }
        >
          <a href="#" on:click={
            let on_logout = on_logout.clone();
            move |_| on_logout()
          }>"Logout"</a>
        </Show>
        " | "
        <A href=Page::Posts.path() >"Posts"</A>
        " | "
        <div class="toggle-switch">
          <label class="darkmode-switch">
            <input type="checkbox" checked=darkmode.get() on:change={
              let darkmode = darkmode.clone();
              move |e| darkmode.set(event_target_checked(&e))
            }/>
            <span class="slider"></span>
          </label>
        </div>
      </nav>
    }
}
