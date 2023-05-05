use leptos::*;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Cat {
    url: String,
}

#[derive(Error, Clone, Debug)]
pub enum FetchError {
    #[error("Please request more than zero cats.")]
    NonZeroCats,
    #[error("Error loading data from serving.")]
    Request,
    #[error("Error deserializaing cat data from request.")]
    Json,
}

async fn fetch_cats(count: u32) -> Result<Vec<String>, FetchError> {
    if count > 0 {
        // make the request
        let res = reqwest::get(&format!(
            "https://api.thecatapi.com/v1/images/search?limit={count}",
        ))
        .await
        .map_err(|_| FetchError::Request)?
        // convert it to JSON
        .json::<Vec<Cat>>()
        .await
        .map_err(|_| FetchError::Json)?
        // extract the URL field for each cat
        .into_iter()
        .map(|cat| cat.url)
        .collect::<Vec<_>>();

        log!("{:#?}", &res);

        Ok(res)
    } else {
        Err(FetchError::NonZeroCats)
    }
}

#[component]
pub fn FetchExample(cx: Scope) -> impl IntoView {
    let (cat_count, set_cat_count) = create_signal::<u32>(cx, 0);

    // we use local_resource here because
    // 1) our error type isn't serializable/deserializable
    // 2) we're not doing server-side rendering in this example anyway
    //    (during SSR, create_resource will begin loading on the server and resolve on the client)
    let cats = create_local_resource(cx, cat_count, fetch_cats);

    let fallback = move |cx, errors: RwSignal<Errors>| {
        let error_list = move || {
            errors.with(|errors| {
                errors
                    .iter()
                    .map(|(_, e)| view! { cx, <li><span>{e.to_string()}</span></li> })
                    .collect::<Vec<_>>()
            })
        };

        view! { cx,
            <div class="alert alert-error shadow-lg">
                <svg xmlns="http://www.w3.org/2000/svg" class="stroke-current flex-shrink-0 h-6 w-6" fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z" /></svg>
                <ul>{error_list}</ul>
            </div>
        }
    };

    // the renderer can handle Option<_> and Result<_> states
    // by displaying nothing for None if the resource is still loading
    // and by using the ErrorBoundary fallback to catch Err(_)
    // so we'll just implement our happy path and let the framework handle the rest
    let cats_view = move || {
        cats.read(cx).map(|data| {

            data.map(|data| {
                data.iter()
                    .map(|s| {
                        log!("data: {:#?}", &s);
                        view! { cx, <div class="carousel-item"><img class="rounded-box" src={s} /></div> }
                    })
                    .collect::<Vec<_>>()
            })
        })
    };

    view! { cx,
        <div>
            <label>
                "How many cats would you like?"
                <input
                    type="number"
                    prop:value=move || cat_count.get().to_string()
                    on:input=move |ev| {
                        let val = event_target_value(&ev).parse::<u32>().unwrap_or(0);
                        set_cat_count(val);
                    }
                />
            </label>
            <ErrorBoundary fallback>
                <Transition fallback=move || view! { cx, <div>"Loading (Suspense Fallback)..."</div> }>
                    <div>
                        "TEST"
                    </div>
                </Transition>
            </ErrorBoundary>
            <div>
                {cats_view}
            </div>
        </div>
    }
}
