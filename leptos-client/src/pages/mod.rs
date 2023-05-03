pub mod home;
pub mod login;
pub mod posts;
pub mod register;

pub use self::{home::*, login::*, posts::*, register::*};

#[derive(Debug, Clone, Copy, Default)]
pub enum Page {
    #[default]
    Home,
    Login,
    Register,
    Posts,
    NotFound,
}

impl Page {
    pub fn path(&self) -> &'static str {
        match self {
            Self::Home => "/",
            Self::Login => "/login",
            Self::Register => "/register",
            Self::Posts => "/posts",
            Self::NotFound => "*",
        }
    }
}
