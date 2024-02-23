use leptos::*;

use crate::components::{ProgressBar, RadialProgress};

#[component]
pub fn Test() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    let double_count = move || count() * 2;
    let tentimes_count = move || count() * 10;
    let btn = move |_| set_count.update(|count| *count += 1);

    view! {
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
                        <ProgressBar progress=Signal::derive(double_count)/>
                    </div>
                    <div class="flex flex-col py-6 gap-2 items-center">
                        <RadialProgress progress=Signal::derive(tentimes_count)/>
                    </div>
                </div>
            </div>
            <div class="btm-nav bottom-7 max-w-xl mx-auto rounded-box bg-slate-500/25 gap-2 flex flex-row justify-center px-4">
                <button class="btn btn-warning basis-1/2">
                    <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6" /></svg>"Button"
                </button>
                <button class="btn text-success basis-1/4">
                    <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" /></svg>
                </button>
                <button class="btn text-success basis-1/4">
                    <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z" /></svg>
                </button>
            </div>
        </div>
    }
}
