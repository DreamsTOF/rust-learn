// ============================================================
// 答案 e352: 生命周期监控 — Effect::new + on_cleanup
//
// 完整可编译实现，不含 TODO。
// 子组件挂载时打印日志，卸载时清理；通过按钮切换显示。
// ============================================================

use leptos::prelude::*;

#[component]
fn Child() -> impl IntoView {
    // 挂载时打印日志
    Effect::new(move || {
       leptos::web_sys::console::log_1(&"Child 已挂载".into());
   });

    // 卸载时执行清理
    on_cleanup(move || {
        leptos::web_sys::console::log_1(&"Child 已卸载".into());
    });

    view! {
        <div style="border: 1px solid #4CAF50; padding: 12px; margin: 8px 0; border-radius: 4px;">
            <p>"🧒 子组件 — 查看浏览器控制台 (F12)"</p>
        </div>
    }
}

#[component]
fn Exercise() -> impl IntoView {
    let (visible, set_visible) = signal(true);

    view! {
        <div>
            <h2>"答案 e352: 生命周期监控"</h2>
            <p>"打开浏览器控制台 (F12)，观察子组件挂载/卸载日志。"</p>
            <button on:click=move |_| set_visible.update(|v| *v = !*v)>
                {move || if visible() { "隐藏子组件" } else { "显示子组件" }}
            </button>
            {move || visible().then(|| view! { <Child /> })}
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
