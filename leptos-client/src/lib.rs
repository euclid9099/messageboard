use gloo_storage::{LocalStorage, Storage};
use leptos::*;
use leptos_router::*;
use serde_json::Value;

mod api;
mod components;
mod pages;
mod types;

use self::{components::*, pages::*, types::*};
const DEFAULT_API_URL: &str = "http://127.0.0.1:7700";
const API_TOKEN_STORAGE_KEY: &str = "api-token";

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // -- signals -- //

    let (token, set_token) = create_signal(cx, None::<ApiToken>);
    let user_info = create_rw_signal(cx, None::<types::UserInfo>);
    let logged_in = Signal::derive(cx, move || token.get().is_some());

    // -- actions -- //

    let fetch_user_info = create_action(cx, move |_| async move {
        match token.get() {
            Some(token) => match api::load_user(Some(token.id())).await {
                Ok(info) => {
                    user_info.update(|i| *i = Some(info));
                }
                Err(err) => {
                    log::error!("Unable to fetch user info: {err}")
                }
            },
            None => {
                log::error!("Unable to fetch user info: not logged in")
            }
        }
    });

    let logout = create_action(cx, move |_| async move {
        set_token.update(|a| *a = None);
        user_info.update(|i| *i = None);
    });

    // -- callbacks -- //

    let on_logout = move || {
        logout.dispatch(());
    };

    // -- init API -- //

    if let Ok(token_storage) = LocalStorage::get(API_TOKEN_STORAGE_KEY) {
        set_token.update(|a| *a = Some(token_storage));
    }

    log::debug!("User is logged in: {}", logged_in.get());

    // -- effects -- //

    create_effect(cx, move |_| {
        log::debug!("API authorization state changed");
        match token.get() {
            Some(token) => {
                log::debug!("API is now authorized: save token in LocalStorage");
                LocalStorage::set(API_TOKEN_STORAGE_KEY, token).expect("LocalStorage::set");
            }
            None => {
                log::debug!("API is no longer authorized: delete token from LocalStorage");
                LocalStorage::delete(API_TOKEN_STORAGE_KEY);
            }
        }
    });

    view! { cx,
      <Router>
        <NavBar logged_in on_logout />
        <main>
          <Routes>
            <Route
              path=Page::Home.path()
              view=move |cx| view! { cx,
                <Home user_info = user_info.into() />
              }
            />
            <Route
              path=Page::Login.path()
              view=move |cx| view! { cx,
                <Login
                    on_success = move |t| {
                        log::info!("Successfully logged in");
                        set_token.update(|v| *v = Some(t));
                        let navigate = use_navigate(cx);
                        navigate(Page::Home.path(), Default::default()).expect("Home route");
                        fetch_user_info.dispatch(());
                } />
              }
            />
            <Route
              path=Page::Register.path()
              view=move |cx| view! { cx,
                <Register
                    on_success = move |t| {
                        log::info!("Successfully registered and logged in");
                        set_token.update(|v| *v = Some(t));
                        let navigate = use_navigate(cx);
                        navigate(Page::Home.path(), Default::default()).expect("Home route");
                        fetch_user_info.dispatch(());
                } />
              }
            />
            <Route
              path=Page::Posts.path()
              view=move |cx| view! { cx,
                <Posts token=token/>
            }/>
            <Route
            path=Page::NotFound.path()
            view=move |cx| view! { cx,
                <h2>"404 - Page not found"</h2>
                }
            />
          </Routes>
        </main>
      </Router>
    }
}
