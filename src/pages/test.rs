use leptos::*;

use crate::components::{ProgressBar, RadialProgress};

#[component]
pub fn Test(cx: Scope) -> impl IntoView {
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
