// ============================================================
// 练习 e139: js_integration
//
// 目标: 使用 wasm_bindgen 调用 JS 函数 / 集成第三方 JS 库
//
// 难度: ⭐⭐⭐
// 核心知识点: wasm_bindgen 调用 JS
//
// TODO: 利用 wasm_bindgen 声明外部 JS 函数并调用
//       (如 console.log / alert / prompt 等)
// ============================================================

use leptos::prelude::*;
use wasm_bindgen::prelude::*;

// 声明需要调用的 JS 函数
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    fn alert(s: &str);
    fn prompt(s: &str) -> Option<String>;
}

#[component]
fn JsIntegration() -> impl IntoView {
    let (name, set_name) = signal(String::from("未输入"));

    let ask_name = move || {
        log("按钮被点击！");
        if let Some(input) = prompt("请输入你的名字:") {
            log(&format!("用户输入: {input}"));
            set_name.set(input);
        } else {
            log("用户取消了输入");
            alert("你取消了输入");
        }
    };

    view! {
        <div>
            <h2>"练习 e139: JS 集成 (wasm_bindgen)"</h2>
            <button on:click=move |_| ask_name()>"输入名字"</button>
            <p>"你好, " {name}</p>
        </div>
    }
}

fn main() {
    mount_to_body(JsIntegration);
}

// ============================================================
// 参考答案
// ============================================================
// <details>
// <summary>点击展开</summary>
//
// ```rust
// #[wasm_bindgen]
// extern "C" {
//     #[wasm_bindgen(js_namespace = console)]
//     fn log(s: &str);
//     fn alert(s: &str);
//     fn prompt(s: &str) -> Option<String>;
// }
// ```
//
// `#[wasm_bindgen] extern "C"` 声明 JS 函数签名。
// `js_namespace` 指定命名空间 (如 console.log)。
// `Option<String>` 映射可能为 null 的 JS 返回值。
// 这是集成任意第三方 JS 库的基础模式。
//
// </details>
