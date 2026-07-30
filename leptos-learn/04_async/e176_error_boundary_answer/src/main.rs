// ============================================================
// Exercise 176 - Answer
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
