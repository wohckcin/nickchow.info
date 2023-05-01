use leptos::*;

const MANY_COUNTERS: usize = 10;

type CounterHolder = Vec<(usize, (ReadSignal<i32>, WriteSignal<i32>))>;

#[derive(Copy, Clone)]
struct CounterUpdater {
    set_counters: WriteSignal<CounterHolder>,
}

#[component]
pub fn Counters(cx: Scope) -> impl IntoView {
    let (next_counter_id, set_next_counter_id) = create_signal(cx, 0);
    let (counters, set_counters) = create_signal::<CounterHolder>(cx, vec![]);
    provide_context(cx, CounterUpdater { set_counters });

    let add_counter = move |_| {
        let id = next_counter_id.get();
        let sig = create_signal(cx, 0);
        set_counters.update(move |counters| counters.push((id, sig)));
        set_next_counter_id.update(|id| *id += 1);
    };

    let add_many_counters = move |_| {
        let next_id = next_counter_id.get();
        let new_counters = (next_id..next_id + MANY_COUNTERS).map(|id| {
            let signal = create_signal(cx, 0);
            (id, signal)
        });

        set_counters.update(move |counters| counters.extend(new_counters));
        set_next_counter_id.update(|id| *id += MANY_COUNTERS);
    };

    let clear_counters = move |_| {
        set_counters.update(|counters| counters.clear());
    };

    let total = move || {
        counters
            .get()
            .iter()
            .map(|(_, (count, _))| count.get())
            .sum::<i32>()
            .to_string()
    };

    let counter_number =
        move || counters.with(|counters| counters.len()).to_string();

    view! { cx,
        <div class="hero min-h-full">
            <div class="hero-content text-center">
                <div class="max-w-md">
                    <div class="btn-group">
                        <button class="btn btn-sm" on:click=add_counter>
                            "Add Counter"
                        </button>
                        <button class="btn btn-sm" on:click=add_many_counters>
                            {format!("Add {MANY_COUNTERS} Counters")}
                        </button>
                        <button class="btn btn-error btn-sm" on:click=clear_counters>
                            "Clear Counters"
                        </button>
                    </div>
                    <p class="py-6 text-base-content text-sm text-opacity-80">
                        "Total: "
                        <span>{total}</span>
                        " from "
                        <span>{counter_number}</span>
                        " counters."
                    </p>
                    <ul>
                        <For
                            each={move || counters.get()}
                            key={|counter| counter.0}
                            view=move |cx, (id, (value, set_value))| {
                                view! {
                                    cx,
                                    <Counter id value set_value/>
                                }
                            }
                        />
                    </ul>
                </div>
            </div>
        </div>
    }
}

#[component]
fn Counter(
    cx: Scope,
    id: usize,
    value: ReadSignal<i32>,
    set_value: WriteSignal<i32>,
) -> impl IntoView {
    let CounterUpdater { set_counters } = use_context(cx).unwrap();

    let input = move |ev| {
        set_value
            .set(event_target_value(&ev).parse::<i32>().unwrap_or_default())
    };

    let countdown = move || format!("--value:{};", value.get());

    view! { cx,
        <li class="gap-2 py-2 flex flex-wrap items-center justify-center">
            <span class="countdown">
                <span style={countdown}></span>
            </span>
            <input type="text" class="input input-xs"
            prop:value={move || value.get().to_string()}
            on:input=input
            />
            <button class="kbd" on:click=move |_| set_value.update(move |value| *value -= 1)>"▼"</button>
            <button class="kbd" on:click=move |_| set_value.update(move |value| *value += 1)>"▲"</button>
            <button class="kbd" on:click=move |_| set_counters.update(move |counters| counters.retain(|(counter_id, _)| counter_id != &id))>
                <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" /></svg>
            </button>
        </li>
    }
}
