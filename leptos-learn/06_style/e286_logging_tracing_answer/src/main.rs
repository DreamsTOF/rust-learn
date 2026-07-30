use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    let (count, set_count) = signal(0);

    view! {
        <div>
            <h2>"日志示例"</h2>
            <p>"点击次数: " {count}</p>
            <button on:click=move |_| {
                set_count.update(|n| *n += 1);
                leptos::logging::log!("按钮被点击，当前次数: {}", count.get());
                if count.get() >= 5 {
                    leptos::logging::log!("警告: 点击次数已达到 {} 次，超过阈值！", count.get());
                }
            }>"点击我"</button>
            <button on:click=move |_| {
                set_count.set(0);
                leptos::logging::log!("计数器已重置为 0");
            }>"重置"</button>
            <p>"打开浏览器控制台 (F12) 查看日志"</p>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
