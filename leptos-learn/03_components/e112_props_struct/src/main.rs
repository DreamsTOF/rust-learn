// ============================================================
// 练习 e112: props_struct — 分离 Prop 结构体
//
// 核心知识点:
//   - Props Struct 独立定义
//   - 将多个相关 prop 组织到结构体中
//
// 难度: ⭐⭐
// ============================================================

use leptos::prelude::*;

// TODO: 定义 CardProps 结构体，包含 title、content 和 theme 字段
// 提示: 组件需要 Clone，所以派生 Clone
#[derive(Clone)]
struct CardProps {
    title: &'static str,
    content: &'static str,
    theme: &'static str,
}

// TODO: 定义组件 InfoCard，接收 CardProps 结构体作为唯一参数
// 提示: 直接用 props: CardProps 作为函数参数
#[component]
fn InfoCard(props: CardProps) -> impl IntoView {
    view! {
        // TODO: 使用 props.theme 作为 class，显示 title 和 content
        <div class={props.theme}>
            <h3>{props.title}</h3>
            <p>{props.content}</p>
        </div>
    }
}

#[component]
fn Exercise() -> impl IntoView {
    // TODO: 创建两个 CardProps 实例
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
            // TODO: 使用 InfoCard 组件并传入 props
            <InfoCard props=card1 />
            <InfoCard props=card2 />
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}

// ============================================================
// 参考答案 (思考后再看!)
// ============================================================
// <details>
// <summary>点击展开答案</summary>
//
// ### 代码
// ```rust
// use leptos::prelude::*;
//
// #[derive(Clone)]
// struct CardProps {
//     title: &'static str,
//     content: &'static str,
//     theme: &'static str,
// }
//
// #[component]
// fn InfoCard(props: CardProps) -> impl IntoView {
//     view! {
//         <div class={props.theme}>
//             <h3>{props.title}</h3>
//             <p>{props.content}</p>
//         </div>
//     }
// }
//
// #[component]
// fn Exercise() -> impl IntoView {
//     let card1 = CardProps {
//         title: "Rust 语言",
//         content: "一门安全、并发、实用的系统编程语言",
//         theme: "dark",
//     };
//     let card2 = CardProps {
//         title: "Leptos 框架",
//         content: "使用 Rust 构建高性能 Web 应用的全栈框架",
//         theme: "light",
//     };
//     view! {
//         <div>
//             <h3>"Props Struct 独立定义"</h3>
//             <InfoCard props=card1 />
//             <InfoCard props=card2 />
//         </div>
//     }
// }
//
// fn main() {
//     mount_to_body(Exercise);
// }
// ```
//
// ### 知识点
// - 将多个 prop 定义为一个结构体，可以让组件参数更清晰、更易复用
// - `#[derive(Clone)]` 是必需的，因为 Leptos 需要克隆 prop 值
// - 通过结构体传递 props，适合 prop 较多或需要复用的场景
// - 与直接解构相比，结构体方式更易于扩展和维护
//
// </details>
