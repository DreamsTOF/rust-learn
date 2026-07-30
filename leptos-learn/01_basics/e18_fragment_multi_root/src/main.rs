// ============================================================
// 练习 e18: view! 多根节点 Fragment
//
// 核心知识点:
//   - <></> Fragment 语法
//   - Fragment 嵌套
//   - 组件中多个根节点返回
//
// 难度: ⭐ (填空题 — 每行都有 TODO 指引)
// ============================================================

use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    view! {
        // TODO: 方式一 — 使用 <></> 返回多个根节点，Fragment 不会产生额外 DOM 节点
        <>
            <h2>"方式一：基础 Fragment"</h2>
            <p>"Fragment 不会在 DOM 中产生额外节点"</p>
            <p>"这是第二个根节点"</p>
        </>

        // TODO: 方式二 — Fragment 可以嵌套使用
        <>
            <h2>"方式二：嵌套 Fragment"</h2>
            <>
                <p>"这是嵌套 Fragment 中的内容"</p>
                <p>"Fragment 可以任意嵌套，不影响 DOM 结构"</p>
            </>
        </>

        // TODO: 方式三 — Fragment 中可以放 0 个、1 个或多个元素
        <>
            <h2>"方式三：Fragment 的灵活性"</h2>
            // 空 Fragment — 不渲染任何内容
            <></>
        </>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(Exercise);
}

// <details>
// 参考答案（去除注释后的纯净版本）:
//
// use leptos::prelude::*;
//
// #[component]
// fn Exercise() -> impl IntoView {
//     view! {
//         <>
//             <h2>"方式一：基础 Fragment"</h2>
//             <p>"Fragment 不会在 DOM 中产生额外节点"</p>
//             <p>"这是第二个根节点"</p>
//         </>
//         <>
//             <h2>"方式二：嵌套 Fragment"</h2>
//             <>
//                 <p>"这是嵌套 Fragment 中的内容"</p>
//                 <p>"Fragment 可以任意嵌套，不影响 DOM 结构"</p>
//             </>
//         </>
//         <>
//             <h2>"方式三：Fragment 的灵活性"</h2>
//             <></>
//         </>
//     }
// }
//
// fn main() {
//     console_error_panic_hook::set_once();
//     mount_to_body(Exercise);
// }
// </details>
