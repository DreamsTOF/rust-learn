// ============================================================
// 练习 e94: error_boundary_fallback — 自定义错误 fallback
//
// 核心知识点:
//   - fallback=|| "出错了" — 忽略错误详情，显示统一提示
//   - ErrorBoundary 的 fallback 闭包可以忽略 errors 参数
//
// 难度: ⭐⭐ (TODO 约 50%)
// ============================================================

use leptos::prelude::*;
use std::fmt;

#[derive(Debug, Clone)]
struct CrashError(String);

impl fmt::Display for CrashError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for CrashError {}

#[component]
fn Exercise() -> impl IntoView {
    let (value, set_value) = signal(Ok("正常显示"));

    view! {
        <h2>"自定义 Fallback"</h2>
        <button on:click=move |_| set_value.set(Ok("正常状态"))>
            "恢复正常"
        </button>
        <button on:click=move |_| set_value.set(Err(CrashError("组件崩溃！".into())))>
            "触发错误"
        </button>

        <ErrorBoundary fallback=|_| {
            view! { <p style="color: red; font-weight: bold;">"出错了"</p> }
        }>
            <p>{move || value.get()}</p>
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
// struct CrashError(String);
// impl fmt::Display for CrashError {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "{}", self.0)
//     }
// }
// impl std::error::Error for CrashError {}
//
// #[component]
// fn Exercise() -> impl IntoView {
//     let (value, set_value) = signal(Ok("正常显示"));
//     view! {
//         <h2>"自定义 Fallback"</h2>
//         <button on:click=move |_| set_value.set(Ok("正常状态"))>"恢复正常"</button>
//         <button on:click=move |_| set_value.set(Err(CrashError("组件崩溃！".into())))>"触发错误"</button>
//         <ErrorBoundary fallback=|_| view! { <p style="color: red; font-weight: bold;">"出错了"</p> }>
//             <p>{move || value.get()}</p>
//         </ErrorBoundary>
//     }
// }
//
// fn main() { mount_to_body(Exercise); }
//
// ### 知识点
// - fallback 闭包的 `|_|` 忽略 errors 参数，直接返回静态视图
// - 适合只需要显示"出错了"的通用错误提示场景
// - 对比 e93: 这里 fallback 不读取错误详情，更简单
// </details>
