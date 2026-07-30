// ============================================================
// 练习 e139: js_integration - 答案
// ============================================================

use leptos::prelude::*;
use wasm_bindgen::prelude::*;

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
