// ============================================================
// 练习 e176: error_boundary — 捕获渲染错误显示 fallback
//
// 核心知识点:
//   - <ErrorBoundary> 捕获子组件中的 Result::Err
//   - fallback 闭包接收 Errors 信号，显示自定义错误界面
//
// 难度: ⭐⭐ (TODO 约 50%)
// ============================================================

use leptos::prelude::*;
use std::fmt;

// TODO: 定义一个错误类型 DivideByZero
// 要求: 实现 Debug、Clone、Display、std::error::Error
// 提示: Display 输出中文错误信息 "除数不能为零"
#[derive(Debug, Clone)]
struct DivideByZero;

impl fmt::Display for DivideByZero {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO: 输出错误信息
        write!(f, "除数不能为零")
    }
}

impl std::error::Error for DivideByZero {}

#[component]
fn Exercise() -> impl IntoView {
    // TODO: 创建一个信号 (value, set_value)，初始值为 Ok(100)
    // 提示: 使用 signal() 宏，类型为 Result<i32, DivideByZero>
    let (value, set_value) = signal(Ok(100i32));

    view! {
        <h2>"ErrorBoundary 示例"</h2>
        <input
            type="number"
            on:input:target=move |ev| {
                let val = ev.target().value().parse::<i32>();
                match val {
                    Ok(0) => set_value.set(Err(DivideByZero)),
                    Ok(n) => set_value.set(Ok(100 / n)),
                    Err(_) => set_value.set(Err(DivideByZero)),
                }
            }
            placeholder="输入除数"
        />
        // TODO: 使用 <ErrorBoundary> 包裹 <p>
        // fallback 接收 errors，遍历显示所有错误信息
        // 子组件显示 "计算结果: {value}"
        <ErrorBoundary fallback=|errors| {
            view! {
                <p style="color: red;">
                    "出错了: "
                    {move || {
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
//     let (value, set_value) = signal(Ok(100i32));
//     view! {
//         <h2>"ErrorBoundary 示例"</h2>
//         <input type="number"
//             on:input:target=move |ev| {
//                 let val = ev.target().value().parse::<i32>();
//                 match val {
//                     Ok(0) => set_value.set(Err(DivideByZero)),
//                     Ok(n) => set_value.set(Ok(100 / n)),
//                     Err(_) => set_value.set(Err(DivideByZero)),
//                 }
//             }
//             placeholder="输入除数"
//         />
//         <ErrorBoundary fallback=|errors| {
//             view! {
//                 <p style="color: red;">
//                     "出错了: "
//                     {move || errors.get().iter().map(|(_, e)| e.to_string()).collect::<Vec<_>>().join(", ")}
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
// - fallback 中的 errors 是响应式信号，动态更新
//
// </details>
