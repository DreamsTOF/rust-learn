// ============================================================
// 练习 e177: error_boundary_fallback — 展开错误详情
//
// 核心知识点:
//   - fallback=|\|errors\|| 使用 <details> 展开错误信息
//   - errors.get() / errors.read() 返回 Errors 结构
//   - Errors.iter() 遍历 (ErrorId, Error) 键值对
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
    let (value, set_value) = signal(Ok("正常状态"));

    view! {
        <h2>"可展开的 ErrorBoundary"</h2>
        <button on:click=move |_| set_value.set(Ok("正常状态"))>
            "恢复正常"
        </button>
        <button on:click=move |_| {
            set_value.set(Err(CrashError("发生了一个不可恢复的错误！请检查输入数据。".into())))
        }>
            "触发错误"
        </button>

        // TODO: 使用 <ErrorBoundary> 包裹 <p>
        // fallback 使用 <details>/<summary> 展开显示错误列表
        // 错误列表使用 <ul>/<li> 渲染
        <ErrorBoundary fallback=|errors| {
            let errors = errors.clone();
            view! {
                <div style="border: 1px solid red; padding: 8px; margin: 8px 0;">
                    <p style="color: red; font-weight: bold;">"操作失败"</p>
                    <details>
                        <summary>"点击查看错误详情"</summary>
                        <ul>
                            {move || errors.read().iter().map(|(_, e)| {
                                view! { <li>{e.to_string()}</li> }
                            }).collect::<Vec<_>>()}
                        </ul>
                    </details>
                </div>
            }
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
//     let (value, set_value) = signal(Ok("正常状态"));
//     view! {
//         <h2>"可展开的 ErrorBoundary"</h2>
//         <button on:click=move |_| set_value.set(Ok("正常状态"))>"恢复正常"</button>
//         <button on:click=move |_| set_value.set(Err(CrashError("发生了一个不可恢复的错误！请检查输入数据。".into())))>"触发错误"</button>
//         <ErrorBoundary fallback=|errors| {
//             let errors = errors.clone();
//             view! {
//                 <div style="border: 1px solid red; padding: 8px; margin: 8px 0;">
//                     <p style="color: red; font-weight: bold;">"操作失败"</p>
//                     <details>
//                         <summary>"点击查看错误详情"</summary>
//                         <ul>
//                             {move || errors.read().iter().map(|(_, e)| view! { <li>{e.to_string()}</li> }).collect::<Vec<_>>()}
//                         </ul>
//                     </details>
//                 </div>
//             }
//         }>
//             <p>{move || value.get()}</p>
//         </ErrorBoundary>
//     }
// }
//
// fn main() { mount_to_body(Exercise); }
//
// ### 知识点
// - fallback 闭包接收 `ArcRwSignal<Errors>`，可调用 .read() 或 .get()
// - `.read()` 返回引用，适用于不需要 Clone 的场景
// - `.get()` 返回 Errors 的克隆
// - Errors 内部是 FxHashMap<ErrorId, Error>，可用 .iter() 遍历
// - <details>/<summary> 是原生 HTML，用户点击展开/折叠
//
// </details>
