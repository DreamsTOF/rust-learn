// ============================================================
// Exercise e112: props_struct — Answer
// ============================================================

use leptos::prelude::*;

#[derive(Clone)]
struct CardProps {
    title: &'static str,
    content: &'static str,
    theme: &'static str,
}

#[component]
fn InfoCard(props: CardProps) -> impl IntoView {
    view! {
        <div class={props.theme}>
            <h3>{props.title}</h3>
            <p>{props.content}</p>
        </div>
    }
}

#[component]
fn Exercise() -> impl IntoView {
    let card1 = CardProps {
        title: "Rust 语言",
        content: "一门安全、并发、实用的系统编程语言",
        theme: "dark",
    };
    let card2 = CardProps {
        title: "Leptos 框架",
        content: "使用 Rust 构建高性能 Web 应用的全栈框架",
        theme: "light",
    };
    view! {
        <div>
            <h3>"Props Struct 独立定义"</h3>
            <InfoCard props=card1 />
            <InfoCard props=card2 />
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
