use leptos::*;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <div class="hero min-h-screen">
            <div class="hero-content text-center">
                <div class="max-w-md">
                    <div class="py-6 flex justify-center">
                        <img src="/media/nick.png" width="200" class="" alt="Nick's Memoji" />
                    </div>
                    <h1 class="text-5xl font-black">"👋 Hi, "
                        <span class="bg-indigo-600 text-white rounded px-1 font-semibold">"I’m Nick"</span>
                        ". "
                        <span class="bg-[linear-gradient(90deg,theme(colors.error)_0%,theme(colors.secondary)_9%,theme(colors.secondary)_42%,theme(colors.primary)_47%,theme(colors.accent)_100%)] bg-clip-text will-change-auto [transform:translate3d(0,0,0)] [-webkit-text-fill-color:transparent] motion-reduce:!tracking-normal max-[1279px]:!tracking-normal [@supports(color:oklch(0_0_0))]:bg-[linear-gradient(90deg,oklch(var(--s))_4%,color-mix(in_oklch,oklch(var(--s)),oklch(var(--er)))_22%,oklch(var(--p))_45%,color-mix(in_oklch,oklch(var(--p)),oklch(var(--a)))_67%,oklch(var(--a))_100.2%)]">"Nice to meet you."</span>
                    </h1>
                    <p class="py-6 text-succes">"I am a solution architect and cloud engineer. Also I am a Rust, DIY and embedded systems enthusiast in my spare time."</p>
                </div>
            </div>
        </div>
    }
}
