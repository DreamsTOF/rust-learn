// ============================================================
// 练习 e114: conditional_props — 条件 Props
//
// 核心知识点:
//   - #[prop(default)] 提供默认值
//   - 根据 prop 值条件渲染不同样式/内容
//
// 难度: ⭐⭐⭐
// ============================================================

use leptos::prelude::*;

// TODO: 定义 MessageBox 组件
// 使用 #[prop(default = ...)] 为 variant 和 highlight 提供默认值
#[component]
fn MessageBox(
    /// 变体类型: "info" | "warning" | "error"
    #[prop(default = "info")]
    variant: &'static str,
    /// 是否高亮显示
    #[prop(default = false)]
    highlight: bool,
    /// 消息文本
    text: &'static str,
) -> impl IntoView {
    // TODO: 根据 variant 和 highlight 计算样式
    let bg_color = if highlight { "#fff3cd" } else { "#f8f9fa" };
    let border_color = match variant {
        "warning" => "#ffc107",
        "error" => "#dc3545",
        _ => "#0d6efd",
    };
    let icon = match variant {
        "warning" => "⚠️",
        "error" => "❌",
        _ => "ℹ️",
    };

    view! {
        // TODO: 应用计算后的样式到 div
        <div style=format!(
            "background: {}; border-left: 4px solid {}; padding: 12px; border-radius: 4px; margin: 8px 0;",
            bg_color, border_color,
        )>
            <p>{icon} " [" {variant} "] " {text}</p>
        </div>
    }
}

#[component]
fn Exercise() -> impl IntoView {
    view! {
        <div>
            <h3>"条件 Props"</h3>
            // TODO: 使用 MessageBox 组件，分别测试不同变体和高亮
            // 不传 variant 和 highlight，使用默认值
            <MessageBox text="这是一条普通信息" />
            // 传入 variant="warning"
            <MessageBox variant="warning" text="注意：磁盘空间不足" />
            // 传入 variant="error" 并高亮
            <MessageBox variant="error" highlight=true text="系统发生错误！" />
            // 普通信息但高亮
            <MessageBox highlight=true text="这是一条高亮信息" />
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
//
// #[component]
// fn MessageBox(
//     #[prop(default = "info")]
//     variant: &'static str,
//     #[prop(default = false)]
//     highlight: bool,
//     text: &'static str,
// ) -> impl IntoView {
//     let bg_color = if highlight { "#fff3cd" } else { "#f8f9fa" };
//     let border_color = match variant {
//         "warning" => "#ffc107",
//         "error" => "#dc3545",
//         _ => "#0d6efd",
//     };
//     let icon = match variant {
//         "warning" => "⚠️",
//         "error" => "❌",
//         _ => "ℹ️",
//     };
//     view! {
//         <div style=format!(
//             "background:{}; border-left:4px solid {}; padding:12px; border-radius:4px; margin:8px 0;",
//             bg_color, border_color,
//         )>
//             <p>{icon} " [" {variant} "] " {text}</p>
//         </div>
//     }
// }
//
// #[component]
// fn Exercise() -> impl IntoView {
//     view! {
//         <div>
//             <h3>"条件 Props"</h3>
//             <MessageBox text="这是一条普通信息" />
//             <MessageBox variant="warning" text="注意：磁盘空间不足" />
//             <MessageBox variant="error" highlight=true text="系统发生错误！" />
//             <MessageBox highlight=true text="这是一条高亮信息" />
//         </div>
//     }
// }
//
// fn main() {
//     mount_to_body(Exercise);
// }
// ```
//
// ### 知识点
// - `#[prop(default = 值)]` 让 prop 可选，不传时使用默认值
// - 与 `Option<T>` 不同，调用方无需 `Some(...)` 包装
// - 默认值可以是字面量或简单表达式（如 `false`、`"info"`、`0`）
// - 配合条件逻辑（match/if），可以实现灵活的组件变体
//
// </details>
