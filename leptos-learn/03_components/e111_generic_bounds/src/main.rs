// ============================================================
// 练习 e111: generic_bounds — 带约束的泛型组件
//
// 核心知识点:
//   - 泛型参数 + trait bound
//   - 组件可接收任意实现了特定 trait 的类型
//
// 难度: ⭐⭐
// ============================================================

use leptos::prelude::*;
use std::fmt::Display;

// TODO: 定义泛型组件 DisplayItem，接收任意实现了 Display + 'static 的值
// 提示: 在组件名称后添加 <T: Display + 'static> 泛型参数
#[component]
fn DisplayItem<T: Display + 'static>(value: T, label: &'static str) -> impl IntoView {
    view! {
        // TODO: 显示 label: value 的格式化文本
        <p><strong>{label}</strong> ": " {value.to_string()}</p>
    }
}

#[component]
fn Exercise() -> impl IntoView {
    view! {
        <div>
            <h3>"泛型 + Trait Bound"</h3>
            // TODO: 使用 DisplayItem 组件分别传入 i32、字符串和 f64 类型的值
            <DisplayItem value=42 label="数字" />
            <DisplayItem value="Hello Leptos!" label="字符串" />
            <DisplayItem value=3.14159 label="浮点数" />
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
// use std::fmt::Display;
//
// #[component]
// fn DisplayItem<T: Display + 'static>(value: T, label: &'static str) -> impl IntoView {
//     view! {
//         <p><strong>{label}</strong> ": " {value.to_string()}</p>
//     }
// }
//
// #[component]
// fn Exercise() -> impl IntoView {
//     view! {
//         <div>
//             <h3>"泛型 + Trait Bound"</h3>
//             <DisplayItem value=42 label="数字" />
//             <DisplayItem value="Hello Leptos!" label="字符串" />
//             <DisplayItem value=3.14159 label="浮点数" />
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
// - `#[component]` 支持泛型参数，写法与普通 Rust 泛型一致
// - trait bound 限制可传入的类型，如 `Display` 确保能 `.to_string()`
// - `'static` 约束确保泛型类型不包含非静态引用，这是 Leptos 响应式系统的要求
// - Leptos 会在编译时为每个具体类型生成独立的组件实例
//
// </details>
