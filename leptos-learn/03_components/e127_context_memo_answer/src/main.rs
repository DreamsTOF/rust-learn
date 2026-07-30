use leptos::prelude::*;

#[component]
fn Child() -> impl IntoView {
    let doubled = use_context::<Memo<i32>>().expect("doubled Memo should be provided via context");

    view! {
        <p>"Derived (Memo): " {doubled}</p>
    }
}

#[component]
fn Exercise() -> impl IntoView {
    let (count, set_count) = signal(1);
    let doubled = Memo::new(move |_| count.get() * 2);

    provide_context(doubled);

    view! {
        <h2>"Context + Memo"</h2>
        <p>"Original: " {count}</p>
        <button on:click=move |_| set_count.update(|c| *c += 1)>"+1"</button>
        <hr/>
        <Child/>
    }
}

fn main() {
    mount_to_body(Exercise);
}
