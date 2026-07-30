use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    let (count, set_count) = signal(0);

    view! {
        <div>
            <h2>"HMR 热重载演示"</h2>
            <p>"尝试修改此组件中的文本，观察浏览器是否自动更新（无需手动刷新）"</p>
            <p>"当前计数: " {count}</p>
            <button on:click=move |_| set_count.set(count.get() + 1)>"+1"</button>
            <button on:click=move |_| set_count.set(count.get() - 1)>"-1"</button>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
