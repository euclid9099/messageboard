use leptos::*;
use leptos_router::*;

use crate::{
    api::{self},
    components::credentials::*,
    types::{ApiToken, Credentials},
    Page,
};

#[component]
pub fn Login<F>(cx: Scope, on_success: F) -> impl IntoView
where
    F: Fn(ApiToken) + 'static + Clone,
{
    let (login_error, set_login_error) = create_signal(cx, None::<String>);
    let (wait_for_response, set_wait_for_response) = create_signal(cx, false);

    let login_action = create_action(cx, move |(username, password): &(String, String)| {
        let username = username.to_string();
        let password = password.to_string();
        let credentials = Credentials { username, password };
        let on_success = on_success.clone();
        async move {
            set_wait_for_response.update(|w| *w = true);
            let result = api::login(&credentials).await;
            set_wait_for_response.update(|w| *w = false);
            match result {
                Ok(res) => {
                    set_login_error.update(|e| *e = None);
                    on_success(res);
                }
                Err(err) => {
                    let msg = match err {
                        api::Error::Fetch(js_err) => {
                            format!("{js_err:?}")
                        }
                        api::Error::Api(err) => err.message,
                    };
                    error!("Unable to login with {}: {msg}", credentials.username);
                    set_login_error.update(|e| *e = Some(msg));
                }
            }
        }
    });

    let disabled = Signal::derive(cx, move || wait_for_response.get());

    view! { cx,
        <div class="credential-form">
        <CredentialsForm
            title = "Please login to your account"
            action_label = "Login"
            action = login_action
            error = login_error.into()
            disabled
        />
        <p>"Don't have an account?"</p>
        <A href=Page::Register.path(None)>"Register"</A>
      </div>
    }
}
