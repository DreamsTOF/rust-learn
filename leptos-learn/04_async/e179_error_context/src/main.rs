// ============================================================
// 练习 e179: error_context — 异步 Result 转为 Context 错误
//
// 核心知识点:
//   - spawn_local 执行异步操作，Result 配合 ErrorBoundary
//   - 异步操作返回 Result，ErrorBoundary 自动捕获 Err
//   - 错误类型携带上下文信息，方便定位问题
//
// 难度: ⭐⭐⭐ (TODO 约 50%)
// ============================================================

use leptos::prelude::*;
use leptos::task::spawn_local;
use std::fmt;

/// 带上下文的错误类型
#[derive(Debug, Clone)]
struct FetchError {
    context: &'static str,
    detail: &'static str,
}

impl fmt::Display for FetchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}] {}", self.context, self.detail)
    }
}

impl std::error::Error for FetchError {}

/// 模拟异步数据获取
async fn fetch_data(should_fail: bool) -> Result<String, FetchError> {
    if should_fail {
        Err(FetchError {
            context: "用户模块",
            detail: "获取用户信息失败：网络超时",
        })
    } else {
        Ok("用户数据加载成功".to_string())
    }
}

// TODO: 定义子组件 DataDisplay
// 接收 data 信号，在 ErrorBoundary 内显示
#[component]
fn DataDisplay(
    // TODO: 接收 ReadSignal<Option<Result<String, FetchError>>>
    data: ReadSignal<Option<Result<String, FetchError>>>,
) -> impl IntoView {
    view! {
        <div style="border: 1px solid #ccc; padding: 12px; margin: 8px 0;">
            <h3>"数据展示区"</h3>
            // TODO: 使用 <ErrorBoundary> 包裹数据显示
            // 当 data 为 Some(Err(...)) 时触发 fallback
            // fallback 显示 context + detail
            <ErrorBoundary fallback=move |errors| {
                view! {
                    <div style="border: 1px solid #d32f2f; padding: 12px; background: #ffebee; border-radius: 4px;">
                        <p style="color: #d32f2f; font-weight: bold;">"异步操作失败"</p>
                        <ul>
                            {move || errors.get().iter().map(|(_, e)| {
                                view! { <li>{e.to_string()}</li> }
                            }).collect::<Vec<_>>()}
                        </ul>
                    </div>
                }
            }>
                // 渲染数据: data().and_then(|r| r.ok())
                <p>{move || data().and_then(|r| r.ok())}</p>
            </ErrorBoundary>
        </div>
    }
}

#[component]
fn Exercise() -> impl IntoView {
    let (should_fail, set_should_fail) = signal(false);
    let (data, set_data) = signal::<Option<Result<String, FetchError>>>(None);

    // TODO: 使用 spawn_local 执行异步操作
    // 在回调中调用 fetch_data，并将结果写入 set_data
    let fetch = move || {
        set_data.set(None);
        let fail = should_fail.get();
        spawn_local(async move {
            let result = fetch_data(fail).await;
            set_data.set(Some(result));
        });
    };

    // 初始加载
    fetch();

    view! {
        <h2>"异步错误上下文处理"</h2>
        <p>"当前模式: " {move || if should_fail() { "错误模式" } else { "正常模式" }}</p>
        <button on:click=move |_| { set_should_fail.update(|v| *v = !*v); fetch(); }>
            {move || if should_fail() { "切换到正常" } else { "切换到错误" }}
        </button>

        // TODO: 使用 DataDisplay 组件，传入 data 信号
        <DataDisplay data />
    }
}

fn main() {
    mount_to_body(Exercise);
}

// <details>
// 参考答案（去除注释后的纯净版本）:
//
// use leptos::prelude::*;
// use leptos::task::spawn_local;
// use std::fmt;
//
// #[derive(Debug, Clone)]
// struct FetchError { context: &'static str, detail: &'static str }
// impl fmt::Display for FetchError {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "[{}] {}", self.context, self.detail)
//     }
// }
// impl std::error::Error for FetchError {}
//
// async fn fetch_data(should_fail: bool) -> Result<String, FetchError> {
//     if should_fail {
//         Err(FetchError { context: "用户模块", detail: "获取用户信息失败：网络超时" })
//     } else {
//         Ok("用户数据加载成功".to_string())
//     }
// }
//
// #[component]
// fn DataDisplay(data: ReadSignal<Option<Result<String, FetchError>>>) -> impl IntoView {
//     view! {
//         <div style="border: 1px solid #ccc; padding: 12px; margin: 8px 0;">
//             <h3>"数据展示区"</h3>
//             <ErrorBoundary fallback=move |errors| {
//                 view! {
//                     <div style="border: 1px solid #d32f2f; padding: 12px; background: #ffebee; border-radius: 4px;">
//                         <p style="color: #d32f2f; font-weight: bold;">"异步操作失败"</p>
//                         <ul>
//                             {move || errors.get().iter().map(|(_, e)| view! { <li>{e.to_string()}</li> }).collect::<Vec<_>>()}
//                         </ul>
//                     </div>
//                 }
//             }>
//                 <p>{move || data().and_then(|r| r.ok())}</p>
//             </ErrorBoundary>
//         </div>
//     }
// }
//
// #[component]
// fn Exercise() -> impl IntoView {
//     let (should_fail, set_should_fail) = signal(false);
//     let (data, set_data) = signal::<Option<Result<String, FetchError>>>(None);
//     let fetch = move || {
//         set_data.set(None);
//         let fail = should_fail.get();
//         spawn_local(async move {
//             let result = fetch_data(fail).await;
//             set_data.set(Some(result));
//         });
//     };
//     fetch();
//     view! {
//         <h2>"异步错误上下文处理"</h2>
//         <p>"当前模式: " {move || if should_fail() { "错误模式" } else { "正常模式" }}</p>
//         <button on:click=move |_| { set_should_fail.update(|v| *v = !*v); fetch(); }>
//             {move || if should_fail() { "切换到正常" } else { "切换到错误" }}
//         </button>
//         <DataDisplay data />
//     }
// }
//
// fn main() { mount_to_body(Exercise); }
//
// ### 知识点
// - ErrorBoundary 捕获渲染中的 Result::Err
// - spawn_local 在 CSR 中执行异步操作，结果写入信号
// - 自定义错误类型可携带上下文信息（模块名、操作名等）
// - 异步操作的成功/失败通过信号驱动视图更新
//
// </details>
