// ============================================================
// 练习 e117: props_compile_check — 缺失必填 Prop 的编译报错
//
// 核心知识点:
//   - Leptos 组件在编译期检查必填 props
//   - 缺少必填 prop 时编译器会给出清晰错误
//   - 这是 Rust 类型系统的安全优势
//
// 难度: ⭐⭐
// ============================================================

use leptos::prelude::*;

// TODO: 定义 Greeting 组件，要求传入 name（必填）和 title（可选）
#[component]
fn Greeting(
    /// 必填 prop — 姓名
    name: &'static str,
    /// 可选 prop — 头衔
    #[prop(optional)]
    title: Option<&'static str>,
) -> impl IntoView {
    let display = match title {
        Some(t) => format!("{t} {name}"),
        None => name.to_string(),
    };

    view! {
        <p style="font-size:1.2rem;">"Hello, " {display} "!"</p>
    }
}

#[component]
fn Exercise() -> impl IntoView {
    view! {
        <div style="padding:8px;">
            <h3>"Props 编译期检查"</h3>

            // 正确用法: 传入所有必填 prop
            <Greeting name="World" title="Dr." />

            // 正确用法: 省略可选 prop
            <Greeting name="Rust" />

            // ❌ 下面这行若取消注释会导致编译错误:
            //  error: missing required prop `name` for `Greeting`
            // <Greeting />
            //
            // 原因: name 是必填 prop（没有 #[prop(optional)]），必须传值。

            <p style="color:#888; font-size:0.9em; margin-top:12px;">
                "提示: 尝试删除上方某个 "name" 参数, 观察编译错误"
            </p>
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
// #[component]
// fn Greeting(
//     name: &'static str,
//     #[prop(optional)]
//     title: Option<&'static str>,
// ) -> impl IntoView {
//     let display = match title {
//         Some(t) => format!("{t} {name}"),
//         None => name.to_string(),
//     };
//     view! { <p>"Hello, " {display} "!"</p> }
// }
//
// #[component]
// fn Exercise() -> impl IntoView {
//     view! {
//         <div>
//             <h3>"Props 编译期检查"</h3>
//             <Greeting name="World" title="Dr." />
//             <Greeting name="Rust" />
//         </div>
//     }
// }
//
// fn main() { mount_to_body(Exercise); }
// ```
//
// ### 知识点
// - 不标记 `#[prop(optional)]` 的 prop 是必填的
// - 缺少必填 prop 时，`#[component]` 宏在编译期生成错误
// - 这利用了 Rust 类型系统，杜绝运行时遗漏 prop 的可能
// - 对比 JavaScript 框架：运行时警告 vs Rust 编译期报错
//
// </details>
