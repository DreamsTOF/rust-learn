// ============================================================
// 练习 e372: 组件测试 — 参考答案
//
// 核心知识点:
//   - wasm-bindgen-test 测试框架
//   - 组件渲染结果断言
//   - 响应式状态测试
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

#[component]
fn Exercise() -> impl IntoView {
    view! {
        <div style="max-width: 500px; margin: 20px auto; font-family: sans-serif;">
            <h3>"🧪 组件测试演示"</h3>
            <p>"下方是一个计数器组件，请在答案目录中为其编写测试。"</p>
            <div style="border: 1px solid #ddd; border-radius: 8px; padding: 20px; margin: 12px 0;">
                <Counter initial=0/>
            </div>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}

#[cfg(test)]
mod tests {
    use wasm_bindgen_test::*;
    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    use super::*;
    use wasm_bindgen::JsCast;
    use leptos::web_sys;

    fn setup_counter() -> web_sys::Element {
        let document = leptos::document();
        let mount = document.create_element("div").unwrap();
        let _ = document.body().unwrap().append_child(&mount);
        leptos::mount_to(mount.clone().unchecked_into(), Counter(0));
        mount
    }

    #[wasm_bindgen_test]
    fn counter_initial_value() {
        let mount = setup_counter();
        let value_el = mount.query_selector("[data-testid=\"value\"]")
            .unwrap().unwrap();
        assert_eq!(value_el.text_content().unwrap(), "计数: 0");
    }

    #[wasm_bindgen_test]
    fn counter_increment() {
        let mount = setup_counter();
        let inc_btn = mount.query_selector("[data-testid=\"increment\"]")
            .unwrap().unwrap();
        inc_btn.dyn_ref::<web_sys::HtmlElement>().unwrap().click();
        let value_el = mount.query_selector("[data-testid=\"value\"]")
            .unwrap().unwrap();
        assert_eq!(value_el.text_content().unwrap(), "计数: 1");
    }

    #[wasm_bindgen_test]
    fn counter_decrement() {
        let mount = setup_counter();
        let dec_btn = mount.query_selector("[data-testid=\"decrement\"]")
            .unwrap().unwrap();
        dec_btn.dyn_ref::<web_sys::HtmlElement>().unwrap().click();
        let value_el = mount.query_selector("[data-testid=\"value\"]")
            .unwrap().unwrap();
        assert_eq!(value_el.text_content().unwrap(), "计数: -1");
    }

    #[wasm_bindgen_test]
    fn counter_reset() {
        let mount = setup_counter();
        // 先递增
        let inc_btn = mount.query_selector("[data-testid=\"increment\"]")
            .unwrap().unwrap();
        inc_btn.dyn_ref::<web_sys::HtmlElement>().unwrap().click();
        // 再重置
        let reset_btn = mount.query_selector("[data-testid=\"reset\"]")
            .unwrap().unwrap();
        reset_btn.dyn_ref::<web_sys::HtmlElement>().unwrap().click();
        let value_el = mount.query_selector("[data-testid=\"value\"]")
            .unwrap().unwrap();
        assert_eq!(value_el.text_content().unwrap(), "计数: 0");
    }
}
