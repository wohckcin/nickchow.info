use leptos::*;
use leptos_router::*;

#[component]
pub fn Nav(cx: Scope) -> impl IntoView {
    view! { cx,
        <header class="navbar">
            <nav class="navbar-start">
                <A class="btn btn-ghost normal-case text-xl" href="/">"NC"</A>
                <ul tabindex="0" class="menu menu-compact mt-3 p-2 shadow bg-base-100 rounded-box w-52">
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
