// ============================================================
// 练习 e101: Children Slot — 组件接受子节点插槽
//
// 核心知识点:
//   - children: Children 参数类型
//   - 在 view! 中用 children() 渲染子节点
//
// 难度: ⭐⭐
// ============================================================

use leptos::prelude::*;

// Panel 组件通过 children: Children 接受子节点
// Children = Box<dyn FnOnce() -> AnyView + Send>
// 在组件内调用 children() 即可渲染传入的子节点
#[component]
fn Panel(children: Children) -> impl IntoView {
    view! {
        <div style="border:2px solid #4a90d9;padding:20px;border-radius:10px;margin:10px 0;">
            <h3>"📦 Panel 组件"</h3>
            <hr/>
            {children()}
        </div>
    }
}

#[component]
fn Exercise() -> impl IntoView {
    // TODO: 使用 Panel 组件包裹一段内容
    // 提示: <Panel>...</Panel> 中的内容会自动传入 children 参数
    view! {
        <Panel>
            <p>"这是 Panel 内部的内容"</p>
            <p>"所有子节点都会被渲染到 Panel 的 children 插槽中"</p>
        </Panel>
    }
}

fn main() {
    mount_to_body(Exercise);
}

// <details>
// 参考答案（去除注释后的纯净版本）:
//
// use leptos::prelude::*;
//
// #[component]
// fn Panel(children: Children) -> impl IntoView {
//     view! {
//         <div style="border:2px solid #4a90d9;padding:20px;border-radius:10px;margin:10px 0;">
//             <h3>"📦 Panel 组件"</h3>
//             <hr/>
//             {children()}
//         </div>
//     }
// }
//
// #[component]
// fn Exercise() -> impl IntoView {
//     view! {
//         <Panel>
//             <p>"这是 Panel 内部的内容"</p>
//             <p>"所有子节点都会被渲染到 Panel 的 children 插槽中"</p>
//         </Panel>
//     }
// }
//
// fn main() {
//     mount_to_body(Exercise);
// }
// </details>
