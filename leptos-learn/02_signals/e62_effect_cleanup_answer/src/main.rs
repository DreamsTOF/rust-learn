use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    let (count, set_count) = signal(0);

    Effect::new(move |_| {
        let c = count.get();
        println!("Effect 运行: count={}", c);
        on_cleanup(|| {
            println!("Cleanup 运行");
        });
    });

    view! {
        <p>"count: " {count}</p>
        <button on:click=move |_| set_count.update(|n| *n += 1)>"+1"</button>
    }
}

fn main() {
    mount_to_body(Exercise);
}
