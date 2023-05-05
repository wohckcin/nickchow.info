use leptos::*;
use leptos_meta::*;
use leptos_router::*;

// use self::{counters::*, fetch::*, notfound::*, progress::*, routes::*};
use crate::{components::*, pages::*};

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    provide_meta_context(cx);

    view! {
        cx,
        <>
            // <Stylesheet id="leptos" href="/pkg/tailwind.css"/>
            // <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
            <Router>
                // <Nav />
                <main>
                    <Routes>
                        <Route path=Page::Home.path() view=  move |cx| view! { cx, <Home/> }/>
                        <Route path=Page::Test.path() view=|cx| view! { cx, <Test/> }/>
                        <Route path=Page::Counters.path() view=|cx| view! { cx, <Counters/> }/>
                        <Route path=Page::Fetch.path() view=|cx| view! { cx, <FetchExample/> }/>
                        <Route path=Page::NotFound.path() view=|cx| view! { cx, <NotFound/> }/>
                    </Routes>
                </main>
                <Footer />
            </Router>
        </>
    }
}
