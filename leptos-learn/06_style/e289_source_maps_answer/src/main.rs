use leptos::prelude::*;

fn calculate(x: i32, y: i32) -> i32 {
    let result = x + y;
    leptos::logging::log!("calculate({}, {}) = {}", x, y, result);
    // 在这里设置断点可以观察参数值
    result
}

#[component]
fn Exercise() -> impl IntoView {
    let (result, set_result) = signal(0);

    view! {
        <div>
            <h2>"源码映射示例"</h2>
            <p>"Debug 模式下 WASM 包含 DWARF debug 信息"</p>
            <p>"结果: " {result}</p>
            <button on:click=move |_| {
                let value = calculate(42, 58);
                leptos::logging::log!("计算完成: {}", value);
                set_result.set(value);
            }>"计算 42 + 58"</button>
            <button on:click=move |_| {
                let value = calculate(100, 200);
                set_result.set(value);
            }>"计算 100 + 200"</button>
            <p>"提示: 打开浏览器 DevTools > Sources 面板"</p>
            <p>"在 Page 标签中搜索 main.rs 可设置断点"</p>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
