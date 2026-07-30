use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    let (a, set_a) = signal(10);
    let (b, set_b) = signal(5);

    let sum = move || a() + b();

    view! {
        <div>
            <p>"练习 39: multi_signal_derived"</p>
            <p>"a = " {a}</p>
            <p>"b = " {b}</p>
            <p>"a + b = " {sum}</p>
            <button on:click=move |_| set_a.update(|v| *v += 1)>"a += 1"</button>
            <button on:click=move |_| set_b.update(|v| *v += 1)>"b += 1"</button>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
