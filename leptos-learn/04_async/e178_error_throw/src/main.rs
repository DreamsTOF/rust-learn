// ============================================================
// 练习 e178: error_throw — 在任意层级触发错误
//
// 核心知识点:
//   - 子组件中的 Result::Err 自动向上传播到最近 ErrorBoundary
//   - 多层嵌套时，错误从抛出点冒泡，中间组件无需感知
//   - 每个 ErrorBoundary 只捕获其子树内的错误
//
// 难度: ⭐⭐⭐ (TODO 约 50%)
// ============================================================

use leptos::prelude::*;
use std::fmt;

// TODO: 定义一个错误类型 ValidationError
#[derive(Debug, Clone)]
struct ValidationError(&'static str);

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "验证失败: {}", self.0)
    }
}

impl std::error::Error for ValidationError {}

// TODO: 深层子组件 — 接收一个 Result 信号，渲染按钮
// 提示: 接收 set_value: WriteSignal<Result<&'static str, ValidationError>>
#[component]
fn DeepChild(
    value: ReadSignal<Result<&'static str, ValidationError>>,
    set_value: WriteSignal<Result<&'static str, ValidationError>>,
) -> impl IntoView {
    view! {
        <div style="border: 1px solid gray; padding: 8px; margin: 4px;">
            <p>"深层子组件"</p>
            // TODO: 显示当前值（仅显示 Ok 的值）
            // 提示: move || value.get().ok()
            <p>"当前值: " {move || value.get().ok()}</p>
            <button on:click=move |_| {
                set_value.set(Err(ValidationError("深层组件触发的错误！")));
            }>
                "触发错误"
            </button>
            <button on:click=move |_| {
                set_value.set(Ok("已恢复"));
            }>
                "清除错误"
            </button>
        </div>
    }
}

#[component]
fn Exercise() -> impl IntoView {
    let (value, set_value) = signal(Ok("初始状态"));

    view! {
        <h2>"错误传播 — 任意层级触发"</h2>
        // TODO: 使用 <ErrorBoundary> 包裹整个组件树
        // 错误从 DeepChild 中触发，自动向上传播到这里
        <ErrorBoundary fallback=move |errors| {
            view! {
                <div style="border: 2px solid red; padding: 12px; margin: 8px 0; background: #fff0f0;">
                    <p style="color: red; font-weight: bold;">"⚠ 错误被 ErrorBoundary 捕获"</p>
                    <ul>
                        {move || errors.get().iter().map(|(_, e)| {
                            view! { <li>{e.to_string()}</li> }
                        }).collect::<Vec<_>>()}
                    </ul>
                    <button on:click=move |_| set_value.set(Ok("已手动恢复"))>
                        "恢复"
                    </button>
                </div>
            }
        }>
            <p>"外层组件"</p>
            // 将信号传递给 DeepChild
            <DeepChild value set_value />
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
// struct ValidationError(&'static str);
// impl fmt::Display for ValidationError {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "验证失败: {}", self.0)
//     }
// }
// impl std::error::Error for ValidationError {}
//
// #[component]
// fn DeepChild(
//     value: ReadSignal<Result<&'static str, ValidationError>>,
//     set_value: WriteSignal<Result<&'static str, ValidationError>>,
// ) -> impl IntoView {
//     view! {
//         <div style="border: 1px solid gray; padding: 8px; margin: 4px;">
//             <p>"深层子组件"</p>
//             <p>"当前值: " {move || value.get().ok()}</p>
//             <button on:click=move |_| set_value.set(Err(ValidationError("深层组件触发的错误！")))>"触发错误"</button>
//             <button on:click=move |_| set_value.set(Ok("已恢复"))>"清除错误"</button>
//         </div>
//     }
// }
//
// #[component]
// fn Exercise() -> impl IntoView {
//     let (value, set_value) = signal(Ok("初始状态"));
//     view! {
//         <h2>"错误传播 — 任意层级触发"</h2>
//         <ErrorBoundary fallback=|errors| {
//             view! {
//                 <div style="border: 2px solid red; padding: 12px; margin: 8px 0; background: #fff0f0;">
//                     <p style="color: red; font-weight: bold;">"⚠ 错误被 ErrorBoundary 捕获"</p>
//                     <ul>
//                         {move || errors.get().iter().map(|(_, e)| view! { <li>{e.to_string()}</li> }).collect::<Vec<_>>()}
//                     </ul>
//                     <button on:click=move |_| set_value.set(Ok("已手动恢复"))>"恢复"</button>
//                 </div>
//             }
//         }>
//             <p>"外层组件"</p>
//             <DeepChild value set_value />
//         </ErrorBoundary>
//     }
// }
//
// fn main() { mount_to_body(Exercise); }
//
// ### 知识点
// - 错误可以从组件树的任意深度向上传播到 ErrorBoundary
// - 中间组件无需执行任何错误处理逻辑
// - ErrorBoundary 的捕获范围是其 subtree
// - 信号（signal）持有 Result 类型时，修改为 Err 会触发 fallback
//
// </details>
