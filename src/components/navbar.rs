// use crate::Page;
use leptos::*;
use leptos_router::*;

#[component]
pub fn NavBar() -> impl IntoView {
    view! {
        <header class="top-0flex h-16 w-full justify-cente transition-all">
            <nav class="navbar w-full">
                <A class="btn btn-ghost normal-case text-xl" href="/">"NC"</A>
                <ul tabindex="0" class="bg-base-100 rounded-box">
                    <li>
                        <A href="/" class="justify-between">
                            <strong>"HN"</strong>
                        </A>
                    </li>
                    <li>
                        <A href="/new">
                            <strong>"New"</strong>
                        </A>
                    </li>
                    <li>
                        <A href="/show">
                            <strong>"Show"</strong>
                        </A>
                    </li>
                    <li>
                        <A href="/ask">
                            <strong>"Ask"</strong>
                        </A>
                    </li>
                </ul>
            </nav>
        </header>
    }
}
