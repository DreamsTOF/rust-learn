// ============================================================
// 练习 e352: 生命周期监控 — Effect::new + on_cleanup
//
// 核心知识点:
//   - Effect::new 在组件挂载时执行回调
//   - on_cleanup 注册组件卸载时的清理函数
//   - 用 console.log 观察组件生命周期
//
// 难度: ⭐⭐ (关键位置有 TODO，补全约 50%)
// ============================================================

use leptos::prelude::*;

#[component]
fn Child() -> impl IntoView {
    // === 步骤 1 ——————————————————————————————————————————
    // TODO: 使用 Effect::new 在挂载时打印 "Child 已挂载"
    //   (使用 leptos::web_sys::console::log_1)
    Effect::new(move || {
       leptos::web_sys::console::log_1(&"Child 已挂载".into());
   });

    // === 步骤 2 ——————————————————————————————————————————
    // TODO: 使用 on_cleanup 注册卸载时的清理函数
    //   打印 "Child 已卸载"
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
    // === 步骤 3 ——————————————————————————————————————————
    // TODO: 创建信号控制 Child 组件的显示/隐藏
    let (visible, set_visible) = signal(true);

    view! {
        <div>
            <h2>"练习 e352: 生命周期监控"</h2>
            <p>"打开浏览器控制台 (F12)，观察子组件挂载/卸载日志。"</p>
            <button on:click=move |_| set_visible.update(|v| *v = !*v)>
                {move || if visible() { "隐藏子组件" } else { "显示子组件" }}
            </button>
            // TODO: 根据 visible 信号条件渲染 Child 组件
            {move || visible().then(|| view! { <Child /> })}
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}

// ============================================================
// 参考答案 (思考后再看!)
// ============================================================
// <details>
// <summary>点击展开答案</summary>
//
// ### 完整代码
// ```rust
// use leptos::prelude::*;
//
// #[component]
// fn Child() -> impl IntoView {
//     Effect::new(move |_| {
//         leptos::web_sys::console::log_1(&"Child 已挂载".into());
//     });
//
//     on_cleanup(move || {
//         leptos::web_sys::console::log_1(&"Child 已卸载".into());
//     });
//
//     view! {
//         <div style="border: 1px solid #4CAF50; padding: 12px; margin: 8px 0; border-radius: 4px;">
//             <p>"🧒 子组件 — 查看浏览器控制台 (F12)"</p>
//         </div>
//     }
// }
//
// #[component]
// fn Exercise() -> impl IntoView {
//     let (visible, set_visible) = signal(true);
//
//     view! {
//         <div>
//             <h2>"练习 e352: 生命周期监控"</h2>
//             <p>"打开浏览器控制台 (F12)，观察子组件挂载/卸载日志。"</p>
//             <button on:click=move |_| set_visible.update(|v| *v = !*v)>
//                 {move || if visible() { "隐藏子组件" } else { "显示子组件" }}
//             </button>
//             {move || visible().then(|| view! { <Child /> })}
//         </div>
//     }
// }
//
// fn main() {
//     mount_to_body(Exercise);
// }
// ```
//
// ### 知识点
// - `Effect::new` 回调在创建时立即执行一次，之后在响应式依赖变化时重新执行
// - `on_cleanup` 注册的回调在组件卸载（从 DOM 中被移除）时执行
// - 使用 `visible().then(...)` 根据条件渲染/卸载组件
// - 在 Leptos CSR 应用中通过 `leptos::web_sys::console::log_1` 输出到浏览器控制台
//
// </details>
