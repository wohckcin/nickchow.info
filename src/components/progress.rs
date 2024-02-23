use leptos::*;

// Components
#[component]
pub fn ProgressBar(
    #[prop(default = 100)] max: u16,
    #[prop(into)] progress: Signal<i32>,
) -> impl IntoView {
    view! {
        <progress
            class="progress w-56 progress-secondary"
            max=max
            value=progress
        />
    }
}

#[component]
pub fn RadialProgress(#[prop(into)] progress: Signal<i32>) -> impl IntoView {
    let value = move || format!("--value:{};", { progress.get() });
    view! {
        <div class="radial-progress bg-primary text-primary-content border-4 border-primary" style=value>{progress}"%"</div>
    }
}
