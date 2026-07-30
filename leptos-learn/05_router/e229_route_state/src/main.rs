// ============================================================
// 练习 e229: route_state — 路由状态传递
//
// 目标: 使用 use_navigate() 的 state 参数在页面间传递数据，
//       配合 use_location() 读取传入的 state
//
// 难度: ⭐⭐
// 核心知识点: use_navigate、NavigateOptions.state、use_location、JsValue
// ============================================================

use leptos::prelude::*;
use leptos_router::components::{Router, Routes, Route, A};
use leptos_router::hooks::{use_navigate, use_location};
use leptos_router::location::State;
use leptos_router::path;
use leptos_router::NavigateOptions;
use wasm_bindgen::JsValue;

// 首页 — 输入数据并传递到详情页
// TODO: 实现 use_navigate 的 state 传递
#[component]
fn Home() -> impl IntoView {
    let (input, set_input) = signal(String::new());

    view! {
        <h2>"首页"</h2>
        <p>"在下方输入内容，然后点击按钮传递到详情页"</p>
        <input
            type="text"
            placeholder="输入要传递的数据"
            on:input:target=move |ev| set_input.set(ev.target().value())
        />
        <button on:click=move |_| {
            // === 步骤 1 ——————————————————————————————————————————
            // TODO: 使用 use_navigate() 导航到 /detail，传入 input 作为 state
            // navigate("/detail", NavigateOptions {
            //     state: State::new(Some(JsValue::from_str(&input.get()))),
            //     ..Default::default()
            // });
        }>
            "传递数据到详情页"
        </button>
        <br/><br/>
        <A href="/detail">"直接跳转详情页（无数据）"</A>
    }
}

// 详情页 — 读取 use_location() 获取传入的 state
// TODO: 使用 use_location 读取 state，没有 state 时显示默认信息
#[component]
fn Detail() -> impl IntoView {
    // === 步骤 2 ——————————————————————————————————————————
    // TODO: 获取 location.state 并转为字符串
    // let location = use_location();
    // let state = location.state.get().to_js_value().as_string();

    view! {
        <h2>"详情页"</h2>
        <p>"从首页传入的数据:"</p>
        // TODO: 显示 state 数据
        <p style="color:#666;">
            "(没有接收到数据)"
        </p>
        <A href="/">"返回首页"</A>
    }
}

#[component]
fn Exercise() -> impl IntoView {
    view! {
        <Router>
            <nav>
                <A href="/">"首页"</A>
                <A href="/detail">"详情页"</A>
            </nav>
            <main>
                <Routes fallback=|| "页面未找到">
                    <Route path=path!("/") view=Home/>
                    <Route path=path!("/detail") view=Detail/>
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
// - `use_navigate()` 返回导航函数，`NavigateOptions.state` 传入历史状态
// - `State::new(Some(JsValue::from_str(...)))` 创建路由状态
// - `use_location().state` 是 `ReadSignal<State>`，响应式读取
// - `state.get().to_js_value().as_string()` 将 JsValue 转回 Option<String>
// - 直接访问 /detail（无 state）时 `as_string()` 返回 None
//
// ### 知识点
// - 路由 state 基于浏览器 History API 的 state 机制
// - 前进/后退导航可以正确保留 state
// - JsValue 支持多种数据类型（字符串、数字、对象等）
// - 复杂数据可通过 serde 序列化为 JsValue
//
// </details>
