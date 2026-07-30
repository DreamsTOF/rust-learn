use std::sync::atomic::{AtomicU32, Ordering};
use leptos::prelude::*;

static RENDER_COUNT: AtomicU32 = AtomicU32::new(0);

#[component]
fn Exercise() -> impl IntoView {
    RENDER_COUNT.fetch_add(1, Ordering::Relaxed);
    leptos::logging::log!(
        "组件渲染 #{}",
        RENDER_COUNT.load(Ordering::Relaxed)
    );

    let (click_count, set_click_count) = signal(0u32);

    view! {
        <div>
            <h2>"组件性能分析"</h2>
            <p>"组件渲染次数: " {move || RENDER_COUNT.load(Ordering::Relaxed)}</p>
            <p>"用户点击次数: " {click_count}</p>
            <button on:click=move |_| {
                set_click_count.update(|n| *n += 1);
                leptos::logging::log!(
                    "按钮点击 - 总渲染次数: {}",
                    RENDER_COUNT.load(Ordering::Relaxed)
                );
            }>"点击"</button>
            <button on:click=move |_| {
                set_click_count.set(0);
            }>"重置"</button>
            <p>"提示: 打开浏览器控制台查看渲染日志"</p>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
