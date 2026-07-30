// 练习 289: 源码映射 (Source Maps)
//
// 目标: 理解 WASM 源码映射的配置与调试技巧。
//
// 背景:
// - Leptos 在 debug 模式下自动包含 DWARF debug 信息
// - 浏览器可以将 WASM 映射回 Rust 源码
// - 在浏览器 Sources 面板中可以设置断点调试 Rust 代码
//
// 步骤:
// 1. 使用 leptos::logging::log! 添加调试日志
// 2. 创建一个带计算函数的组件
// 3. 在浏览器 Sources 面板中查找 main.rs 并设置断点

use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    let (result, set_result) = signal(0);

    // TODO: 定义一个有意义的计算函数，添加日志输出
    // fn calculate(x: i32, y: i32) -> i32 { ... }

    view! {
        <div>
            <h2>"源码映射示例"</h2>
            <p>"Debug 模式下 WASM 包含 DWARF debug 信息"</p>
            <p>"结果: " {result}</p>
            <button on:click=move |_| {
                // TODO: 调用计算函数并更新结果
                // 可以在浏览器中设置断点调试
            }>"计算"</button>
            <p>"提示: 打开浏览器 DevTools > Sources 面板"</p>
            <p>"在 Page 标签中搜索 main.rs 可设置断点"</p>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
