use leptos::*;
use leptos_router::*;

use crate::{
    api::{self, AuthorizedApi, UnauthorizedApi},
    components::credentials::*,
    types::Credentials,
    Page,
};

#[component]
pub fn Register<F>(cx: Scope, api: UnauthorizedApi, on_success: F) -> impl IntoView
where
    F: Fn(AuthorizedApi) + 'static + Clone,
{
    let (register_error, set_register_error) = create_signal(cx, None::<String>);
    let (wait_for_response, set_wait_for_response) = create_signal(cx, false);

    let register_action = create_action(cx, move |(username, password): &(String, String)| {
        log::debug!("Try to register with {username}");
        let username = username.to_string();
        let password = password.to_string();
        let credentials = Credentials { username, password };
        let on_success = on_success.clone();
        async move {
            set_wait_for_response.update(|w| *w = true);
            let result = api.register(&credentials).await;
            set_wait_for_response.update(|w| *w = false);
            match result {
                Ok(res) => {
                    set_register_error.update(|e| *e = None);
                    on_success(res);
                }
                Err(err) => {
                    let msg = match err {
                        api::Error::Fetch(js_err) => {
                            format!("{js_err:?}")
                        }
                        api::Error::Api(err) => err.message,
                    };
                    error!("Unable to register with {}: {msg}", credentials.username);
                    set_register_error.update(|e| *e = Some(msg));
                }
            }
        }
    });

    let disabled = Signal::derive(cx, move || wait_for_response.get());

    view! { cx,
      <CredentialsForm
        title = "Register your account with desirde login and password"
        action_label = "Register"
        action = register_action
        error = register_error.into()
        disabled
      />
      <p>"Already have an account?"</p>
      <A href=Page::Login.path()>"Login"</A>
    }
}
