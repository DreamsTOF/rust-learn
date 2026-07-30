// ============================================================
// 练习 e230: route_data — 路由数据共享
//
// 目标: 使用 provide_context / use_context 在路由组件间共享数据，
//       实现全局路由状态管理
//
// 难度: ⭐⭐
// 核心知识点: provide_context、use_context、RwSignal、状态共享
// ============================================================

use leptos::prelude::*;
use leptos_router::components::{Router, Routes, Route, A};
use leptos_router::path;

// 共享数据包装类型（需要 Clone + Copy 以便在组件间传递）
#[derive(Clone, Copy)]
struct SharedCount(RwSignal<i32>);

// 计数器页面 — 可以修改共享数据
// TODO: 使用 use_context 获取共享数据，提供增减按钮
#[component]
fn Counter() -> impl IntoView {
    // === 步骤 1 ——————————————————————————————————————————
    // TODO: 使用 use_context::<SharedCount>() 获取共享状态
    // let count = use_context::<SharedCount>().expect("SharedCount 未提供");

    view! {
        <h2>"计数器"</h2>
        <p>"当前计数: "</p>
        // TODO: 显示 count.0 的值
        <p style="font-size:2rem;font-weight:bold;margin:0.5rem 0;">0</p>
        <button>TODO: 增加</button>
        <button>TODO: 减少</button>
        <br/><br/>
        <A href="/display">"查看显示页"</A>
    }
}

// 显示页面 — 只读共享数据
// TODO: 使用 use_context 读取共享数据
#[component]
fn Display() -> impl IntoView {
    // === 步骤 2 ——————————————————————————————————————————
    // TODO: 使用 use_context::<SharedCount>() 获取共享状态
    // let count = use_context::<SharedCount>().expect("SharedCount 未提供");

    view! {
        <h2>"数据显示页"</h2>
        <p>"共享计数: "</p>
        <p style="font-size:2rem;font-weight:bold;margin:0.5rem 0;">0</p>
        <p>"提示: 在计数器页面修改后，导航到此页面查看变化"</p>
        <A href="/counter">"回到计数器"</A>
    }
}

#[component]
fn Exercise() -> impl IntoView {
    // === 步骤 3 ——————————————————————————————————————————
    // TODO: 创建 SharedCount(RwSignal::new(0)) 并通过 provide_context 注入
    // let shared = SharedCount(RwSignal::new(0));
    // provide_context(shared);

    view! {
        <Router>
            <nav>
                <A href="/counter">"计数器"</A>
                <A href="/display">"显示页"</A>
            </nav>
            <main>
                <Routes fallback=|| "页面未找到">
                    <Route path=path!("/counter") view=Counter/>
                    <Route path=path!("/display") view=Display/>
                </Routes>
            </main>
        </Router>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(Exercise);
}

// ============================================================
// 参考答案 (思考后再看!)
// ============================================================
// <details>
// <summary>点击展开答案</summary>
//
// ### 代码说明
// - `provide_context(shared)` 在 Router 层级注入共享数据
// - `use_context::<SharedCount>()` 在子组件中获取共享状态
// - `RwSignal<i32>` 是可读写信号，路由切换后值保持不变
// - `#[derive(Clone, Copy)]` 使包装类型可以轻松在组件间传递
//
// ### 知识点
// - `provide_context` 基于组件树作用域，所有子路由组件共享
// - 相比 props 逐层传递，context 避免 prop drilling
// - 路由切换不会销毁 context 提供者，状态得以保留
// - 可共享任意类型数据（信号、Resource、配置等）
//
// </details>
