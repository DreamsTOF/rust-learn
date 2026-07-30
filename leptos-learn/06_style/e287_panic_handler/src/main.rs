// 练习 287: Panic 处理 (console_error_panic_hook)
//
// 目标: 设置 panic hook，在浏览器中捕获并显示格式化的 panic 信息。
// 需要先在 Cargo.toml 中添加 console_error_panic_hook.workspace = true
//
// 步骤:
// 1. 调用 console_error_panic_hook::set_once() 设置 panic hook
// 2. 创建一个 Exercise 组件
// 3. 点击按钮触发一个 panic，观察控制台中的格式化错误信息

use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    view! {
        <div>
            <h2>"Panic 处理示例"</h2>
            <p>"点击下方按钮触发一个 panic"</p>
            <p>"打开浏览器控制台 (F12) 查看格式化的 panic 信息"</p>
            <button on:click=move |_| {
                // TODO: 触发一个 panic
                // 使用 panic!() 宏，提供有意义的错误信息
                panic!("TODO: 在这里触发 panic");
            }>"触发 Panic"</button>
        </div>
    }
}

fn main() {
    // TODO: 设置 panic hook
    // console_error_panic_hook::set_once();
    mount_to_body(Exercise);
}
