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

#[derive(Debug, Clone, Copy, Default)]
pub enum Page {
    #[default]
    Home,
    Fetch,
    Test,
    Counters,
    NotFound,
}

impl Page {
    pub fn path(&self) -> &'static str {
        match self {
            Self::Home => "/",
            Self::Fetch => "/fetch",
            Self::Test => "/test",
            Self::Counters => "/counters",
            Self::NotFound => "/*any",
        }
    }
}
