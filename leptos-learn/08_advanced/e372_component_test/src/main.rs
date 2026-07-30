// ============================================================
// 练习 e372: 组件测试 — 使用 wasm-bindgen-test 测试 Leptos 组件
//
// 核心知识点:
//   - wasm-bindgen-test 测试框架
//   - 组件渲染结果断言
//   - 响应式状态测试
//
// 难度: ⭐⭐ (关键位置有 TODO，补全约 50%)
// ============================================================

use leptos::prelude::*;

/// 计数器组件 — 供测试使用
#[component]
fn Counter(initial: i32) -> impl IntoView {
    let (count, set_count) = signal(initial);

    view! {
        <div data-testid="counter">
            <p data-testid="value">"计数: " {move || count.get()}</p>
            <button data-testid="increment" on:click={move |_| set_count.update(|n| *n += 1)}>"+"</button>
            <button data-testid="decrement" on:click={move |_| set_count.update(|n| *n -= 1)}>"-"</button>
            <button data-testid="reset" on:click={move |_| set_count.set(initial)}>"重置"</button>
        </div>
    }
}

// TODO: 在答案目录中，添加 #[cfg(test)] 测试模块，测试以下场景:
//   1. 计数器初始值正确显示
//   2. 点击 + 按钮后计数增加
//   3. 计数器可以设为负值
//   4. 重置按钮恢复初始值
// 提示: 使用 wasm_bindgen_test::wasm_bindgen_test_configure! 和 #[wasm_bindgen_test]

#[component]
fn Exercise() -> impl IntoView {
    view! {
        <div style="max-width: 500px; margin: 20px auto; font-family: sans-serif;">
            <h3>"🧪 组件测试演示"</h3>

            <p>"下方是一个计数器组件，请在答案目录中为其编写测试。"</p>

            <div style="border: 1px solid #ddd; border-radius: 8px; padding: 20px; margin: 12px 0;">
                <Counter initial=0/>
            </div>

            <div style="background: #f8f9fa; border-radius: 8px; padding: 16px; font-size: 0.9em; color: #555;">
                <h4 style="margin: 0 0 8px 0;">"测试清单"</h4>
                <ol style="margin: 0; padding-left: 20px; line-height: 1.8;">
                    <li>"初始值渲染测试 — 验证计数显示为 0"</li>
                    <li>"递增测试 — 点击 + 后计数变为 1"</li>
                    <li>"递减测试 — 点击 - 后计数变为 -1"</li>
                    <li>"重置测试 — 修改后点击重置恢复初始值"</li>
                </ol>
            </div>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}

// <details>
// 参考答案:
// (1) 在 Cargo.toml 中添加 wasm-bindgen-test = "0.3" 作为 dev-dependency
// (2) 在 main.rs 中添加测试模块:
//
// #[cfg(test)]
// mod tests {
//     use wasm_bindgen_test::*;
//     wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);
//
//     use super::*;
//
//     #[wasm_bindgen_test]
//     fn counter_initial_value() {
//         let document = leptos::document();
//         let mount = document.create_element("div").unwrap();
//         let _ = document.body().unwrap().append_child(&mount);
//         leptos::mount_to(
//             mount.clone().unchecked_into(),
//             Counter(0),
//         );
//         assert_eq!(mount.query_selector("[data-testid=\"value\"]")
//             .unwrap().unwrap().text_content().unwrap(),
//             "计数: 0"
//         );
//     }
// }
// </details>
