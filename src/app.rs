use leptos::*;
use leptos_meta::*;
use leptos_router::*;

// use self::{counters::*, fetch::*, notfound::*, progress::*, routes::*};
use crate::{components::*, pages::*};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <>
            // <Stylesheet id="leptos" href="/pkg/tailwind.css"/>
            // <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
            <Router>
                // <Nav />
                <main>
                    <Routes>
                        <Route path=Page::Home.path() view=  move || view! { <Home/> }/>
                        <Route path=Page::Test.path() view= move || view! { <Test/> }/>
                        <Route path=Page::Counters.path() view= move || view! { <Counters/> }/>
                        <Route path=Page::Fetch.path() view= move || view! { <FetchExample/> }/>
                        <Route path=Page::Error.path() view= move || view! { <Error/> }/>
                        <Route path=Page::User.path() view= move || view! { <User/> }/>
                        <Route path=Page::Stories.path() view= move || view! { <Stories/> }/>
                        <Route path=Page::Story.path() view= move || view! { <Story/> }/>
                        <Route path=Page::NotFound.path() view= move || view! { <NotFound/> }/>
                    </Routes>
                </main>
                <Footer />
            </Router>
        </>
    }
}
