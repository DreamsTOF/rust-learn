// ============================================================
// 练习 e109: 透传 Props (Pass-Through Props)
//
// 核心知识点:
//   - 将组件接收的 props 传递给子 HTML 元素
//   - #[prop(into)] 实现 &str 到 String 的自动转换
//
// 难度: ⭐⭐⭐
// ============================================================

use leptos::prelude::*;

// 包装组件：将 title、class 和 style 透传给外层 <div>
#[component]
fn Panel(
    /// 标题文本（支持 &str → String 转换）
    #[prop(into)]
    title: String,
    /// CSS 类名，透传给外层 div
    #[prop(into)]
    class: String,
    /// 内联样式，透传给外层 div
    #[prop(into)]
    style: String,
    /// 子节点
    children: Children,
) -> impl IntoView {
    view! {
        // TODO: 将 class 和 style 透传给此 div
        <div class={class} style={style}>
            <h3>{title}</h3>
            <div class="panel-body">
                {children()}
            </div>
        </div>
    }
}

#[component]
fn App() -> impl IntoView {
    view! {
        <h3>"练习 109: pass_through_props"</h3>
        // TODO: 使用 Panel 组件，传入 title、class、style 和子节点
        <Panel
            title="第一个面板"
            class="primary"
            style="border: 2px solid #4CAF50; padding: 10px;"
        >
            <p>"这个面板有 class 和 style"</p>
        </Panel>
    }
}

fn main() {
    mount_to_body(App);
}

// <details>
// 参考答案:
//
// use leptos::prelude::*;
//
// #[component]
// fn Panel(
//     #[prop(into)]
//     title: String,
//     #[prop(into)]
//     class: String,
//     #[prop(into)]
//     style: String,
//     children: Children,
// ) -> impl IntoView {
//     view! {
//         <div class={class} style={style}>
//             <h3>{title}</h3>
//             <div class="panel-body">
//                 {children()}
//             </div>
//         </div>
//     }
// }
//
// #[component]
// fn App() -> impl IntoView {
//     view! {
//         <h3>"练习 109: pass_through_props"</h3>
//         <Panel
//             title="第一个面板"
//             class="primary"
//             style="border: 2px solid #4CAF50; padding: 10px;"
//         >
//             <p>"这个面板有 class 和 style"</p>
//         </Panel>
//     }
// }
//
// fn main() {
//     mount_to_body(App);
// }
// </details>
