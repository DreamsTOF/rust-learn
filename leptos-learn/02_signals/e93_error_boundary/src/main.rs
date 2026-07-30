// ============================================================
// 练习 e93: error_boundary — 捕获组件渲染错误
//
// 核心知识点:
//   - <ErrorBoundary> 捕获子组件中的 Result::Err
//   - fallback 闭包接收 Errors 信号，显示自定义错误界面
//
// 难度: ⭐⭐ (TODO 约 50%)
// ============================================================

use leptos::prelude::*;
use std::fmt;

#[derive(Debug, Clone)]
struct DivideByZero;

impl fmt::Display for DivideByZero {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "除数不能为零")
    }
}

impl std::error::Error for DivideByZero {}

#[component]
fn Exercise() -> impl IntoView {
    let (value, set_value) = signal(Ok(42i32));

    view! {
        <h2>"ErrorBoundary 示例"</h2>
        <input
            type="number"
            on:input=move |ev| {
                let val = event_target_value(&ev).parse::<i32>();
                match val {
                    Ok(0) => set_value.set(Err(DivideByZero)),
                    Ok(n) => set_value.set(Ok(100 / n)),
                    Err(_) => set_value.set(Err(DivideByZero)),
                }
            }
            placeholder="输入除数"
        />
        <ErrorBoundary fallback=|errors| {
            view! {
                <p style="color: red;">
                    "出错了: " {move || {
                        errors
                            .get()
                            .iter()
                            .map(|(_, e)| e.to_string())
                            .collect::<Vec<_>>()
                            .join(", ")
                    }}
                </p>
            }
        }>
            <p>"计算结果: " {move || value.get()}</p>
        </ErrorBoundary>
    }
}

fn main() {
    mount_to_body(Exercise);
}

// <details>
// 参考答案（去除注释后的纯净版本）:
//
// use leptos::prelude::*;
// use std::fmt;
//
// #[derive(Debug, Clone)]
// struct DivideByZero;
// impl fmt::Display for DivideByZero {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "除数不能为零")
//     }
// }
// impl std::error::Error for DivideByZero {}
//
// #[component]
// fn Exercise() -> impl IntoView {
//     let (value, set_value) = signal(Ok(42i32));
//     view! {
//         <h2>"ErrorBoundary 示例"</h2>
//         <input type="number" on:input=move |ev| {
//             let val = event_target_value(&ev).parse::<i32>();
//             match val {
//                 Ok(0) => set_value.set(Err(DivideByZero)),
//                 Ok(n) => set_value.set(Ok(100 / n)),
//                 Err(_) => set_value.set(Err(DivideByZero)),
//             }
//         } placeholder="输入除数"/>
//         <ErrorBoundary fallback=|errors| {
//             view! {
//                 <p style="color: red;">
//                     "出错了: " {move || errors.get().iter().map(|(_, e)| e.to_string()).collect::<Vec<_>>().join(", ")}
//                 </p>
//             }
//         }>
//             <p>"计算结果: " {move || value.get()}</p>
//         </ErrorBoundary>
//     }
// }
//
// fn main() { mount_to_body(Exercise); }
//
// ### 知识点
// - ErrorBoundary 要求 Error 类型实现 `std::error::Error`
// - 子组件中返回 `Result::Err` 会被 ErrorBoundary 捕获
// - fallback 闭包接收 `ArcRwSignal<Errors>`，可读取错误信息
// - 还可使用 `leptos::error::on_cleanup` 在错误时执行清理
// </details>
