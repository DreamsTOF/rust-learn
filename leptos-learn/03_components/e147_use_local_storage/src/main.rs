// ============================================================
// 练习 e147: use_local_storage — localStorage 读写
//
// 目标: 封装浏览器 localStorage 为响应式 Hook
//
// 难度: ⭐⭐⭐
// 核心知识点: localStorage 读写, Effect 同步, 响应式封装
//
// TODO: 按照注释提示补全代码
// ============================================================

use leptos::prelude::*;
use wasm_bindgen::prelude::*;

// 通过 #[wasm_bindgen] 定义 localStorage API 绑定
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["localStorage"])]
    fn getItem(key: &str) -> Option<String>;
    #[wasm_bindgen(js_namespace = ["localStorage"])]
    fn setItem(key: &str, value: &str);
}

/// Hook: 将信号同步到 localStorage
fn use_local_storage(key: &str, default: &str) -> (ReadSignal<String>, WriteSignal<String>) {
    let initial = getItem(key).unwrap_or_else(|| default.to_string());
    let (value, set_value) = signal(initial);
    let key = key.to_string();

    Effect::new(move |_| {
        let v = value.get();
        setItem(&key, &v);
    });

    (value, set_value)
}

#[component]
fn Exercise() -> impl IntoView {
    let (name, set_name) = use_local_storage("username", "匿名用户");

    view! {
        <div>
            <h2>"e147: use_local_storage"</h2>
            <input
                type="text"
                prop:value=name
                on:input=move |ev| set_name(event_target_value(&ev))
                placeholder="输入你的名字"
            />
            <p>"你好, " {name}</p>
            <p style="font-size:0.8em;color:#888">
                "刷新页面后名字依然保留!"
            </p>
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
// ### 代码
// ```rust
// use leptos::prelude::*;
// use wasm_bindgen::prelude::*;
//
// #[wasm_bindgen]
// extern "C" {
//     #[wasm_bindgen(js_namespace = ["localStorage"])]
//     fn getItem(key: &str) -> Option<String>;
//     #[wasm_bindgen(js_namespace = ["localStorage"])]
//     fn setItem(key: &str, value: &str);
// }
//
// fn use_local_storage(key: &str, default: &str) -> (ReadSignal<String>, WriteSignal<String>) {
//     let initial = getItem(key).unwrap_or_else(|| default.to_string());
//     let (value, set_value) = signal(initial);
//     let key = key.to_string();
//     Effect::new(move |_| {
//         let v = value.get();
//         setItem(&key, &v);
//     });
//     (value, set_value)
// }
// ```
//
// ### 知识点
// - 通过 `#[wasm_bindgen(js_namespace = ["localStorage"])]` 直接绑定浏览器 API
// - `Effect::new` 在信号变化时自动将新值同步到 localStorage
// - 每次组件挂载从 localStorage 读取初始值，实现状态持久化
// - 返回 `(ReadSignal, WriteSignal)` 与 Leptos 原生信号完全兼容
//
// </details>
