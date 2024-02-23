use leptos::*;

#[component]
pub fn NotFound() -> impl IntoView {
    view! {
        <section class="bg-red-200 pt-20 rounded-xl">
            <div class="py-6 flex justify-center">
                <img src="/media/nick.png" width="200" class="" alt="Nick's Memoji" />
            </div>
            <div class="max-w-2xl px-6 mx-auto text-center">
            <h1 class="text-8xl font-semibold text-gray-800">
                "Oops! "
                <span class="bg-red-900 text-white rounded-xl px-3">{"4😕4"}</span>
            </h1>
            <h2 class="text-4xl font-semibold text-gray-800 mt-8">
                "Um, yeah. This is awkward."
            </h2>

            <p class="text-gray-600 mt-4 text-xl">
                "We tried really hard, but couldn't find the page you were looking for.
                You may find what you were looking for on our homepage."
            </p>

            <div class="flex items-end justify-center mt-16">
            </div>
            </div>
        </section>
    }
}
