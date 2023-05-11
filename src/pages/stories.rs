use leptos::*;
use leptos_router::*;

use crate::api;

fn category(from: &str) -> &'static str {
    match from {
        "new" => "newest",
        "show" => "show",
        "ask" => "ask",
        "job" => "jobs",
        _ => "news",
    }
}

#[component]
pub fn Stories(cx: Scope) -> impl IntoView {
    let query = use_query_map(cx);
    let params = use_params_map(cx);
    let page = move || {
        query
            .with(|q| q.get("page").and_then(|page| page.parse::<usize>().ok()))
            .unwrap_or(1)
    };
    let story_type = move || {
        params
            .with(|p| p.get("stories").cloned())
            .unwrap_or_else(|| "top".to_string())
    };
    let stories = create_resource(
        cx,
        move || (page(), story_type()),
        move |(page, story_type)| async move {
            let path = format!("{}?page={}", category(&story_type), page);
            api::fetch_api::<Vec<api::Story>>(cx, &api::story(&path)).await
        },
    );
    let (pending, set_pending) = create_signal(cx, false);

    let hide_more_link = move |cx| {
        pending()
            || stories.read(cx).unwrap_or(None).unwrap_or_default().len() < 28
    };

    view! {
        cx,
        <div class="news-view">
            <div class="btn-group">
                <button class="btn">
                    {move || if page() > 1 {
                        view! {
                            cx,
                            <a class="page-link"
                                href=move || format!("/{}?page={}", story_type(), page() - 1)
                                attr:aria_label="Previous Page"
                            >
                                "<<"
                            </a>
                        }.into_any()
                    } else {
                        view! {
                            cx,
                            <span class="page-link disabled" aria-hidden="true">
                                "<<"
                            </span>
                        }.into_any()
                    }}
                </button>
                 <button class="btn">"Page " {page}</button>
                <Transition
                    fallback=move || view! { cx,  <p>"Loading..."</p> }
                >
                    <button class="btn"
                        class:disabled=move || hide_more_link(cx)
                        aria-hidden=move || hide_more_link(cx)
                    >
                        <a href=move || format!("/{}?page={}", story_type(), page() + 1)
                            aria-label="Next Page"
                        >
                            ">>"
                        </a>
                    </button>
                </Transition>
            </div>
            <main class="news-list">
                <div>
                    <Transition
                        fallback=move || view! { cx,  <p>"Loading..."</p> }
                        set_pending=set_pending.into()
                    >
                        {move || match stories.read(cx) {
                            None => None,
                            Some(None) => Some(view! { cx,  <p>"Error loading stories."</p> }.into_any()),
                            Some(Some(stories)) => {
                                Some(view! { cx,
                                    <ul class="divide-y divide-gray-100">
                                        <For
                                            each=move || stories.clone()
                                            key=|story| story.id
                                            view=move |cx, story: api::Story| {
                                                view! { cx,
                                                    <Story story/>
                                                }
                                            }
                                        />
                                    </ul>
                                }.into_any())
                            }
                        }}
                    </Transition>
                </div>
            </main>
        </div>
    }
}

#[component]
fn Story(cx: Scope, story: api::Story) -> impl IntoView {
    view! { cx,
         <li class="flex justify-between gap-x-6 py-5">
            <div class="flex gap-x-4">
                <span class="score">{story.points}</span>
                <span class="title">
                    {if !story.url.starts_with("item?id=") {
                        view! { cx,
                            <span>
                                <a href=story.url target="_blank" rel="noreferrer">
                                    {story.title.clone()}
                                </a>
                                <span class="host">"("{story.domain}")"</span>
                            </span>
                        }.into_view(cx)
                    } else {
                        let title = story.title.clone();
                        view! { cx,  <A href=format!("/stories/{}", story.id)>{title.clone()}</A> }.into_view(cx)
                    }}
                </span>
            </div>
            // <br />
            <div class="hidden sm:flex sm:flex-col sm:items-end">
                <span class="meta">
                    {if story.story_type != "job" {
                        view! { cx,
                            <span>
                                <p class="text-sm font-semibold leading-6">
                                    {story.user.map(|user| view ! { cx, <A href=format!("/users/{user}")>{user.clone()}</A>})}
                                    {format!(" | {}", story.time_ago)}
                                </p>
                                <p class="mt-1 truncate text-xs leading-5">
                                    <A  href=format!("/stories/{}", story.id)>
                                        {if story.comments_count.unwrap_or_default() > 0 {
                                            format!("{} comments", story.comments_count.unwrap_or_default())
                                        } else {
                                            "discuss".into()
                                        }}
                                    </A>
                                </p>
                            </span>
                        }.into_view(cx)
                    } else {
                        let title = story.title.clone();
                        view! { cx,  <A href=format!("/item/{}", story.id)>{title.clone()}</A> }.into_view(cx)
                    }}
                </span>
                {(story.story_type != "link").then(|| view! { cx,
                    " "
                    <span class="mt-1 truncate text-xs leading-5">{story.story_type}</span>
                })}
            </div>
        </li>
    }
}
