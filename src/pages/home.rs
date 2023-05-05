use leptos::*;

#[component]
pub fn Home(cx: Scope) -> impl IntoView {
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
