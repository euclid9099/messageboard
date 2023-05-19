pub mod home;
pub mod login;
pub mod posts;
pub mod register;
pub mod user;

pub use self::{home::*, login::*, posts::*, register::*, user::*};

#[derive(Debug, Clone, Copy, Default)]
pub enum Page {
    #[default]
    Home,
    Login,
    Register,
    Posts,
    User,
    NotFound,
}

impl Page {
    pub fn path(&self, specifier: Option<String>) -> String {
        match self {
            Self::Home => "/".to_string(),
            Self::Login => "/login".to_string(),
            Self::Register => "/register".to_string(),
            Self::Posts => "/posts".to_string(),
            Self::User => format!("/user/{}", specifier.unwrap_or(":user".to_string())),
            Self::NotFound => "*".to_string(),
        }
    }
}
