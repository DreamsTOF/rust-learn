use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    let (count, set_count) = signal(0i32);
    let (update_count, set_update_count) = signal(0u32);

    view! {
        <div>
            <h2>"性能分析示例"</h2>
            <p>"当前计数: " {count}</p>
            <p>"更新次数: " {update_count}</p>
            <button on:click=move |_| {
                let _sum: i32 = (0..1000).sum();
                set_count.update(|n| *n += 1);
                set_update_count.update(|n| *n += 1);
                leptos::logging::log!("更新 #{} 完成", update_count.get() + 1);
            }>"更新"</button>
            <button on:click=move |_| {
                set_count.set(0);
                set_update_count.set(0);
            }>"重置"</button>
            <p>"提示: 打开浏览器控制台查看日志"</p>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
