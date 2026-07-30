use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    let (count, set_count) = signal(0);
    let (text, set_text) = signal(String::new());

    Effect::new(move || {
        let c = count.get();
        if c > 0 {
            println!("count={}, text len={}", c, text.read().len());
        } else {
            println!("count={}, 未追踪 text", c);
        }
    });

    view! {
        <p>"count: " {count}</p>
        <p>"text: " {text.clone()}</p>
        <button on:click=move |_| set_count.update(|n| { *n += 1; })>"count +1"</button>
        <button on:click=move |_| set_count.update(|n| { if *n > 0 { *n -= 1; } })>"count -1"</button>
        <input
            prop:value=move || text.get()
            on:input=move |e| set_text.set(event_target_value(&e))
        />
    }
}

fn main() {
    mount_to_body(Exercise);
}
