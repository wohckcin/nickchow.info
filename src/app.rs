use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::{counters::*, fetch::*, notfound::*, progress::*, routes::*};

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
                        <Route path="" view=  move |cx| view! { cx, <Home/> }/>
                        <Route path="/test" view=|cx| view! { cx, <Test/> }/>
                        <Route path="/counters" view=|cx| view! { cx, <Counters/> }/>
                        <Route path="/fetch" view=|cx| view! { cx, <FetchExample/> }/>
                        <Route path="/*any" view=|cx| view! { cx, <NotFound/> }/>
                    </Routes>
                </main>
                <Footer />
            </Router>
        </>
    }
}

#[component]
fn Home(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="hero min-h-screen bg-base-200">
            <div class="hero-content text-center">
                <div class="max-w-md">
                    <div class="py-6 flex justify-center">
                        <img src="/media/nick.png" width="200" class="" alt="Nick's Memoji" />
                    </div>
                    <h1 class="text-5xl font-bold">"👋 Hi, "
                        <span class="bg-indigo-600 text-white rounded px-1">"I’m Nick"</span>
                        ". Nice to meet you."
                    </h1>
                    <p class="py-6">"I am a Solution Architect and Cloud Engineer. Also I am a Rust, DIY and Embedded Systems enthusiast in my spare time."</p>
                </div>
            </div>
        </div>
    }
}

#[component]
fn Test(cx: Scope) -> impl IntoView {
    let (count, set_count) = create_signal(cx, 0);
    let double_count = move || count() * 2;
    let tentimes_count = move || count() * 10;
    let btn = move |_| set_count.update(|count| *count += 1);

    view! { cx,
            <div class="hero min-h-screen bg-base-200">
            <div class="hero-content text-center">
                <div class="max-w-md">
                    <button
                        class="btn btn-primary"
                        on:click=btn
                    >
                        "Something's here | "
                        {move || if count() == 0 {
                            "Click me!".to_string()
                        } else {
                            count().to_string()
                        }}
                        " | Some more text"
                    </button>
                    <div class="flex flex-col py-6 gap-2 items-center">
                        <ProgressBar progress=count/>
                        <ProgressBar progress=Signal::derive(cx, double_count)/>
                    </div>
                    <div class="flex flex-col py-6 gap-2 items-center">
                        <RadialProgress progress=Signal::derive(cx, tentimes_count)/>
                    </div>
                </div>
            </div>
        </div>
    }
}
