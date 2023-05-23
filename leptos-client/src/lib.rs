use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

use gloo_storage::{LocalStorage, Storage};
use leptos::*;
use leptos_router::*;

mod api;
mod components;
mod pages;
mod types;

use self::{components::*, pages::*, types::*};
const DEFAULT_API_URL: &str = "http://10.10.1.218:7700";
const API_TOKEN_STORAGE_KEY: &str = "api-token";

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // -- signals -- //

    let token = create_rw_signal(cx, None::<ApiToken>);
    let (user_info, set_user_info) = create_signal(cx, None::<types::UserInfo>);
    let darkmode = create_rw_signal(cx, true);

    // -- actions -- //

    let fetch_user_info = create_action(cx, move |_| async move {
        match token.get() {
            Some(token) => match api::load_user(Some(token.id().to_string()), None).await {
                Ok(info) => {
                    set_user_info.set(Some(info));
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
        token.update(|a| *a = None);
        set_user_info.set(None);
    });

    // -- callbacks -- //

    let on_logout = move || {
        logout.dispatch(());
    };

    // -- init API -- //

    if let Ok(token_storage) = LocalStorage::get::<ApiToken>(API_TOKEN_STORAGE_KEY) {
        log::debug!("{:?}", token_storage);
        if token_storage.exp() > chrono::Utc::now().timestamp() {
            token.set(Some(token_storage));
        }
    }

    let token_invalidation_handle = Arc::new(Mutex::new(None));
    // -- effects -- //
    create_effect(cx, move |_| match token.get() {
        Some(t) => {
            let dur = Duration::from_secs(
                (t.exp() - chrono::Utc::now().timestamp())
                    .try_into()
                    .unwrap(),
            );
            log::debug!("Token ({:?}) expires in {:?}", token, dur);
            LocalStorage::set(API_TOKEN_STORAGE_KEY, t).expect("LocalStorage::set");
            fetch_user_info.dispatch(());
            if let Ok(handle) = set_timeout_with_handle(
                move || {
                    token.set(None);
                    let _ = window().alert_with_message("Your login was invalidated");
                },
                dur,
            ) {
                match token_invalidation_handle.lock() {
                    Ok(mut handle_lock) => {
                        *handle_lock = Some(handle);
                    }
                    Err(err) => {
                        log::error!("Unable to lock token_invalidation_handle: {err}");
                    }
                }
            };
        }
        None => {
            LocalStorage::delete(API_TOKEN_STORAGE_KEY);
            set_user_info.set(None);
            match token_invalidation_handle.lock() {
                Ok(mut handle) => {
                    if handle.is_some() {
                        handle.unwrap().clear();
                        handle.take();
                    }
                }
                Err(err) => {
                    log::error!("Unable to lock token_invalidation_handle: {err}");
                }
            }
        }
    });

    view! { cx,
      <div id="control-root" class=move || if darkmode.get() {"dark"} else {"light"}>
        <title>"Kornelja"</title>
        <Router>
          <NavBar logged_in=user_info on_logout darkmode/>
          <main>
            <Routes>
              <Route
                path=Page::Home.path(None)
                view=move |cx| view! { cx,
                  <Home user_info = user_info />
                }
              />
              <Route
                path=Page::Login.path(None)
                view=move |cx| view! { cx,
                  <Login
                      on_success = move |t| {
                          log::info!("Successfully logged in");
                          token.update(|v| *v = Some(t));
                          let navigate = use_navigate(cx);
                          navigate(&Page::Home.path(None), Default::default()).expect("Home route");
                          fetch_user_info.dispatch(());
                  } />
                }
              />
              <Route
                path=Page::Register.path(None)
                view=move |cx| view! { cx,
                  <Register
                      on_success = move |t| {
                          log::info!("Successfully registered and logged in");
                          token.update(|v| *v = Some(t));
                          let navigate = use_navigate(cx);
                          navigate(&Page::Home.path(None), Default::default()).expect("Home route");
                          fetch_user_info.dispatch(());
                  } />
                }
              />
              <Route
                path=Page::Posts.path(None)
                view=move |cx| view! { cx,
                  <Posts user=user_info token=Signal::from(token)/>
              }/>
              <Route
                path=Page::User.path(None)
                view=move |cx| view! { cx,
                  <User self_info=user_info self_token=token/>
              }/>
              <Route
              path=Page::NotFound.path(None)
              view=move |cx| view! { cx,
                  <h2>"404 - Page not found"</h2>
                  }
              />
            </Routes>
          </main>
        </Router>
      </div>
    }
}
