use leptos::prelude::*;
use leptos::task::spawn_local;

#[component]
fn Exercise() -> impl IntoView {
    let (count, set_count) = signal(0);
    let (msg, set_msg) = signal(String::new());

    Effect::new(move || {
        let c = count.get();
        if c > 0 {
            set_msg.set(format!("正在加载 {}...", c));
            spawn_local(async move {
                let result = simulate_async_work(c).await;
                set_msg.set(result);
            });
        } else {
            set_msg.set(String::new());
        }
    });

    view! {
        <p>"count: " {count}</p>
        <p>"msg: " {msg.clone()}</p>
        <button on:click=move |_| set_count.update(|n| *n += 1)>"+1"</button>
        <button on:click=move |_| set_count.set(0)>"重置"</button>
    }
}

async fn simulate_async_work(n: i32) -> String {
    leptos::task::tick().await;
    format!("异步结果: {}", n * 2)
}

fn main() {
    mount_to_body(Exercise);
}
