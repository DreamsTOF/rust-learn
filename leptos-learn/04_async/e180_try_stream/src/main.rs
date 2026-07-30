// ============================================================
// 练习 e180: try_stream — 流式错误处理
//
// 核心知识点:
//   - futures::stream 结合 Result 实现流式错误处理
//   - 将流中的 Err 转换为 ErrorBoundary 可捕获的错误
//   - spawn_local 在 CSR 中消费异步流
//
// 难度: ⭐⭐⭐ (TODO 约 50%)
// ============================================================

use futures::stream::{self, StreamExt};
use leptos::prelude::*;
use std::fmt;

/// 流式处理中的错误类型
#[derive(Debug, Clone)]
struct StreamError(String);

impl fmt::Display for StreamError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for StreamError {}

/// 生成一个模拟数据流，包含成功和失败
fn create_data_stream() -> impl futures::Stream<Item = Result<i32, StreamError>> {
    stream::iter(vec![
        Ok(1),
        Ok(2),
        Ok(3),
        Err(StreamError("流处理错误：第 4 项数据损坏".into())),
        Ok(4),
        Ok(5),
        Err(StreamError("流处理错误：连接中断".into())),
    ])
}

#[component]
fn Exercise() -> impl IntoView {
    let (items, set_items) = signal(Vec::<String>::new());
    let (stream_error, set_stream_error) = signal::<Option<StreamError>>(None);

    // 消费异步流
    let start_stream = move || {
        set_items.set(Vec::new());
        set_stream_error.set(None);
        let stream = create_data_stream();
        leptos::task::spawn_local(async move {
            let mut stream = stream;
            while let Some(item) = stream.next().await {
                match item {
                    Ok(val) => {
                        set_items.update(|v| v.push(format!("成功: {}", val)));
                    }
                    Err(e) => {
                        // TODO: 将流错误转为 ErrorBoundary 可捕获的信号
                        // 提示: 设置 stream_error 信号为 Some(e)
                        set_stream_error.set(Some(e));
                        break;
                    }
                }
            }
        });
    };

    view! {
        <h2>"TryStream 流式错误处理"</h2>
        <button on:click=move |_| start_stream()>
            "启动数据流"
        </button>

        // TODO: 使用 <ErrorBoundary> 包裹数据显示区域
        // 当 stream_error 为 Some(...) 时触发错误
        // fallback 显示多条错误详情
        <ErrorBoundary fallback=|errors| {
            view! {
                <div style="border: 2px solid #d32f2f; padding: 12px; background: #fff0f0; border-radius: 4px;">
                    <p style="color: #d32f2f; font-weight: bold;">"⚠ 流处理过程中出现错误"</p>
                    <ul>
                        {move || errors.get().iter().map(|(_, e)| {
                            view! { <li>{e.to_string()}</li> }
                        }).collect::<Vec<_>>()}
                    </ul>
                </div>
            }
        }>
            <div style="margin: 8px 0;">
                <h3>"已处理的数据项"</h3>
                <ul>
                    {move || items.get().into_iter().map(|item| {
                        view! { <li>{item}</li> }
                    }).collect::<Vec<_>>()}
                </ul>
                // TODO: 如果 stream_error 为 Some(...)，渲染 Err 触发 ErrorBoundary
                // 提示: 使用 map() 转换 Option<StreamError> 为 Result<(), StreamError>
                // 然后渲染 Result 类型，ErrorBoundary 会自动捕获 Err
                {move || stream_error.get().map(|e| Err::<(), _>(e))}
            </div>
        </ErrorBoundary>
    }
}

fn main() {
    mount_to_body(Exercise);
}

// <details>
// 参考答案（去除注释后的纯净版本）:
//
// use futures::stream::{self, StreamExt};
// use leptos::prelude::*;
// use std::fmt;
//
// #[derive(Debug, Clone)]
// struct StreamError(String);
// impl fmt::Display for StreamError {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "{}", self.0)
//     }
// }
// impl std::error::Error for StreamError {}
//
// fn create_data_stream() -> impl futures::Stream<Item = Result<i32, StreamError>> {
//     stream::iter(vec![
//         Ok(1), Ok(2), Ok(3),
//         Err(StreamError("流处理错误：第 4 项数据损坏".into())),
//         Ok(4), Ok(5),
//         Err(StreamError("流处理错误：连接中断".into())),
//     ])
// }
//
// #[component]
// fn Exercise() -> impl IntoView {
//     let (items, set_items) = signal(Vec::<String>::new());
//     let (stream_error, set_stream_error) = signal::<Option<StreamError>>(None);
//
//     let start_stream = move || {
//         set_items.set(Vec::new());
//         set_stream_error.set(None);
//         let stream = create_data_stream();
//         leptos::task::spawn_local(async move {
//             let mut stream = stream;
//             while let Some(item) = stream.next().await {
//                 match item {
//                     Ok(val) => set_items.update(|v| v.push(format!("成功: {}", val))),
//                     Err(e) => { set_stream_error.set(Some(e)); break; }
//                 }
//             }
//         });
//     };
//
//     view! {
//         <h2>"TryStream 流式错误处理"</h2>
//         <button on:click=move |_| start_stream()>"启动数据流"</button>
//         <ErrorBoundary fallback=|errors| {
//             view! {
//                 <div style="border: 2px solid #d32f2f; padding: 12px; background: #fff0f0; border-radius: 4px;">
//                     <p style="color: #d32f2f; font-weight: bold;">"⚠ 流处理过程中出现错误"</p>
//                     <ul>
//                         {move || errors.get().iter().map(|(_, e)| view! { <li>{e.to_string()}</li> }).collect::<Vec<_>>()}
//                     </ul>
//                 </div>
//             }
//         }>
//             <div style="margin: 8px 0;">
//                 <h3>"已处理的数据项"</h3>
//                 <ul>
//                     {move || items.get().into_iter().map(|item| view! { <li>{item}</li> }).collect::<Vec<_>>()}
//                 </ul>
//                 {move || stream_error.get().map(|e| Err::<(), _>(e))}
//             </div>
//         </ErrorBoundary>
//     }
// }
//
// fn main() { mount_to_body(Exercise); }
//
// ### 知识点
// - `futures::stream::iter` 从迭代器创建同步流
// - `StreamExt::next()` 异步获取流中的下一项
// - `spawn_local` 在 WASM CSR 中执行异步任务
// - 将 `stream_error` 信号转为 `Result::Err` 渲染，ErrorBoundary 自动捕获
// - 流中的错误会终止流处理（break），后续项不再处理
//
// </details>
