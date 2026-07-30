use leptos::prelude::*;


// ============================================================
// 练习 318: 水合基础 (Hydration Basic)
//
// 目标: 理解 hydrate vs mount，实现可水合的应用
//
// 核心知识点:
//   - hydrate() 函数: 复用服务端渲染的 HTML，附加事件
//   - mount_to_body(): 从头渲染（CSR 方式）
//   - #[component] + 客户端交互
//
// ⭐⭐: 填充 hydrate 入口和基本交互组件
// ============================================================

// TODO: 创建 ButtonCounter 组件
//   - 使用 signal() 创建计数信号
//   - 按钮点击增加计数
//   - 显示当前计数

#[component]
fn Exercise() -> impl IntoView {
    view! {
        <div>
            <h1>"练习 318: 水合基础"</h1>
            // TODO: 添加 ButtonCounter 组件
        </div>
    }
}

fn main() {
    // TODO: 使用 mount_to_body(Exercise) 挂载应用
}
