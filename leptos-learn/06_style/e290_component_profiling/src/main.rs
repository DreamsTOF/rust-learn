// 练习 290: 组件性能分析 (Component Profiling)
//
// 目标: 追踪组件渲染次数，观察响应式更新对渲染的影响。
//
// 提示:
// - 使用 AtomicU32 或 Cell 可以安全地计数而不触发响应式循环
// - std::sync::atomic::AtomicU32 是线程安全的计数器
// - leptos::logging::log! 可以在控制台输出日志
//
// 步骤:
// 1. 创建一个全局渲染计数器（AtomicU32）
// 2. 每次组件渲染时递增计数器
// 3. 在控制台输出渲染日志
// 4. 使用按钮触发更新，观察渲染模式

use leptos::prelude::*;

// TODO: 创建全局渲染计数器
// static RENDER_COUNT: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);

#[component]
fn Exercise() -> impl IntoView {
    // TODO: 递增渲染计数器
    // TODO: 在控制台输出渲染日志

    let (click_count, set_click_count) = signal(0u32);

    view! {
        <div>
            <h2>"组件性能分析"</h2>
            // TODO: 显示渲染次数（使用闭包读取原子值）
            // <p>"组件渲染次数: " {move || RENDER_COUNT.load(std::sync::atomic::Ordering::Relaxed)}</p>
            <p>"用户点击次数: " {click_count}</p>
            <button on:click=move |_| {
                set_click_count.update(|n| *n += 1);
                // TODO: 同时输出操作日志
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
