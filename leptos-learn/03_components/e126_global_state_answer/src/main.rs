use leptos::prelude::*;

#[derive(Clone)]
struct AppState {
    count: RwSignal<i32>,
}

impl AppState {
    fn new() -> Self {
        Self {
            count: RwSignal::new(0),
        }
    }
}

#[component]
fn Counter() -> impl IntoView {
    let state = use_context::<AppState>().expect("AppState should be provided at root");

    view! {
        <p>"Global count: " {state.count}</p>
        <button on:click=move |_| state.count.update(|c| *c += 1)>"+1"</button>
        <button on:click=move |_| state.count.update(|c| *c -= 1)>"-1"</button>
    }
}

#[component]
fn Display() -> impl IntoView {
    let state = use_context::<AppState>().expect("AppState should be provided at root");

    view! {
        <p>"Current value x 2 = " {move || state.count.get() * 2}</p>
    }
}

#[component]
fn Exercise() -> impl IntoView {
    let state = AppState::new();
    provide_context(state);

    view! {
        <h2>"Global State Pattern"</h2>
        <Counter/>
        <Display/>
    }
}

fn main() {
    mount_to_body(Exercise);
}
