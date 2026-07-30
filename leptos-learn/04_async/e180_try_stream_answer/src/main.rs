// ============================================================
// Exercise 180 - TryStream
// ============================================================

use futures::stream::{self, StreamExt};
use leptos::prelude::*;
use std::fmt;

#[derive(Debug, Clone)]
struct StreamError(String);

impl fmt::Display for StreamError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for StreamError {}

fn create_data_stream() -> impl futures::Stream<Item = Result<i32, StreamError>> {
    stream::iter(vec![
        Ok(1), Ok(2), Ok(3),
        Err(StreamError("流处理错误：第 4 项数据损坏".into())),
        Ok(4), Ok(5),
        Err(StreamError("流处理错误：连接中断".into())),
    ])
}

#[component]
fn Exercise() -> impl IntoView {
    let (items, set_items) = signal(Vec::<String>::new());
    let (stream_error, set_stream_error) = signal::<Option<StreamError>>(None);

    let start_stream = move || {
        set_items.set(Vec::new());
        set_stream_error.set(None);
        let stream = create_data_stream();
        leptos::task::spawn_local(async move {
            let mut stream = stream;
            while let Some(item) = stream.next().await {
                match item {
                    Ok(val) => set_items.update(|v| v.push(format!("成功: {}", val))),
                    Err(e) => { set_stream_error.set(Some(e)); break; }
                }
            }
        });
    };

    view! {
        <h2>"TryStream 流式错误处理"</h2>
        <button on:click=move |_| start_stream()>"启动数据流"</button>
        <ErrorBoundary fallback=|errors| {
            view! {
                <div style="border: 2px solid #d32f2f; padding: 12px; background: #fff0f0; border-radius: 4px;">
                    <p style="color: #d32f2f; font-weight: bold;">"⚠ 流处理过程中出现错误"</p>
                    <ul>
                        {move || errors.get().iter().map(|(_, e)| view! { <li>{e.to_string()}</li> }).collect::<Vec<_>>()}
                    </ul>
                </div>
            }
        }>
            <div style="margin: 8px 0;">
                <h3>"已处理的数据项"</h3>
                <ul>
                    {move || items.get().into_iter().map(|item| view! { <li>{item}</li> }).collect::<Vec<_>>()}
                </ul>
                {move || stream_error.get().map(|e| Err::<(), _>(e))}
            </div>
        </ErrorBoundary>
    }
}

fn main() {
    mount_to_body(Exercise);
}
