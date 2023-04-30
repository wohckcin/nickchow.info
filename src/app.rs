use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::routes::*;

// Components
#[component]
fn ProgressBar(
    cx: Scope,
    #[prop(default = 100)] max: u16,
    #[prop(into)] progress: Signal<i32>,
) -> impl IntoView {
    view! { cx,
        <progress
            class="progress w-56 progress-secondary"
            max={max}
            value=progress
        />
    }
}

#[component]
fn RadialProgress(
    cx: Scope,
    #[prop(into)] progress: Signal<i32>,
) -> impl IntoView {
    // let nvalue = format!("--value:{};", {progress});
    view! { cx,
        <div class="radial-progress bg-primary text-primary-content border-4 border-primary" style="--value:20;">{progress}</div>
    }
}

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    provide_meta_context(cx);

    view! {
        cx,
        <>
            // <Stylesheet id="leptos" href="/pkg/tailwind.css"/>
            // <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
            <Router>
                <Nav />
                <main>
                    <Routes>
                        <Route path="" view=  move |cx| view! { cx, <Home/> }/>
                        <Route path="/test" view=|cx| view! { cx,
                            <p>"Select a contact to view more info."</p>
                        }/>
                    </Routes>
                </main>
            </Router>
        </>
    }
}

#[component]
fn Home(cx: Scope) -> impl IntoView {
    let (count, set_count) = create_signal(cx, 0);
    let double_count = move || count() * 2;
    let tentimes_count = move || count() * 10;
    let btn = move |_| set_count.update(|count| *count += 1);

    view! { cx,
        <div class="hero min-h-screen bg-base-200">
            <div class="hero-content text-center">
                <div class="max-w-md">
                    <h1 class="text-5xl font-bold">"👋 Hello there"</h1>
                    <p class="py-6">"Provident cupiditate voluptatem et in. Quaerat fugiat ut assumenda excepturi exercitationem quasi. In deleniti eaque aut repudiandae et a id nisi."</p>
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
