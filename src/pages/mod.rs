mod home;
pub use home::*;

mod test;
pub use test::*;

mod counters;
pub use counters::*;

mod notfound;
pub use notfound::*;

mod fetch;
pub use fetch::*;

mod error;
pub use error::*;

mod user;
pub use user::*;

mod stories;
pub use stories::*;

mod story;
pub use story::*;

#[derive(Debug, Clone, Copy, Default)]
pub enum Page {
    #[default]
    Home,
    Fetch,
    Test,
    Counters,
    Error,
    User,
    Story,
    Stories,
    NotFound,
}

impl Page {
    pub fn path(&self) -> &'static str {
        match self {
            Self::Home => "/",
            Self::Fetch => "/fetch",
            Self::Test => "/test",
            Self::Counters => "/counters",
            Self::Error => "/error",
            Self::User => "users/:id",
            Self::Story => "stories/:id",
            Self::Stories => ":stories?",
            Self::NotFound => "/*any",
        }
    }
}
