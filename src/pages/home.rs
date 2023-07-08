use leptos::*;

#[component]
pub fn Home(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="hero min-h-screen">
            <div class="hero-content text-center">
                <div class="max-w-md">
                    <div class="py-6 flex justify-center">
                        <img src="/media/nick.png" width="200" class="" alt="Nick's Memoji" />
                    </div>
                    <h1 class="text-5xl font-black">"👋 Hi, "
                        <span class="bg-indigo-600 text-white rounded px-1 font-semibold">"I’m Nick"</span>
                        ". "
                        <span class="bg-[linear-gradient(90deg,hsl(var(--s))_0%,hsl(var(--sf))_9%,hsl(var(--pf))_42%,hsl(var(--p))_47%,hsl(var(--a))_100%)] bg-clip-text [-webkit-text-fill-color:transparent] max-[1280px]:!tracking-normal [@supports(color:oklch(0_0_0))]:bg-[linear-gradient(90deg,hsl(var(--s))_4%,color-mix(in_oklch,hsl(var(--sf)),hsl(var(--pf)))_22%,hsl(var(--p))_45%,color-mix(in_oklch,hsl(var(--p)),hsl(var(--a)))_67%,hsl(var(--a))_100.2%)]">"Nice to meet you."</span>
                    </h1>
                    <p class="py-6 text-succes">"I am a solution architect and cloud engineer. Also I am a Rust, DIY and embedded systems enthusiast in my spare time."</p>
                </div>
            </div>
        </div>
    }
}
